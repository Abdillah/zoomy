extern crate libc;
extern crate drm;
extern crate mmap;

mod mode;

use std::fs::OpenOptions;
use self::drm::drm_mode::Connection;
use std::os::unix::io::AsRawFd;

const DRM_IOCTL_MODE_CREATE_DUMB: u64 = 0b11000000001000000110010010110010;
const DRM_IOCTL_MODE_MAP_DUMB: u64    = 0b11000000000100000110010010110011;

fn main() {
    // Open card device
    let file = OpenOptions::new()
    .read(true)
    .write(true)
    .open("/dev/dri/card0")
    .expect("Cannot open card file");

    // Find connected connectors
    let drm_res = if let Some(res) = drm::drm_mode::get_resources(file.as_raw_fd()) {
        res
    } else {
        panic!("Resources failed to fetch")
    };

    let connector_ids = drm_res.get_connectors();

    let connectors: Vec<_> = connector_ids.into_iter().map(|connector_id| {
        let connector = drm::drm_mode::get_connector(file.as_raw_fd(), connector_id).unwrap();
        if connector.get_connection() != Connection::Connected {
            return None
        }
        Some(connector)
    })
    .filter(|item| item.is_some())
    .map(|item| item.unwrap())
    .collect();


    let modes: Vec<drm::drm_mode::ModeInfo> = connectors
    .iter().map(|connector| {
        let all_modes = connector.get_modes();
        if all_modes.len() > 0 {
            all_modes[0].clone()
        } else {
            panic!("No mode available")
        }
    }).collect();

    // Get encoder and CRTC
    let crtcs: Vec<Option<_>> = connectors.iter().map(|connector| {
        let encoder_id = connector.get_encoder_id();
        let encoder = drm::drm_mode::get_encoder(file.as_raw_fd(), encoder_id);

        match encoder {
            Some(enc) => {
                let crtc_id = enc.get_crtc_id();
                drm::drm_mode::get_crtc(file.as_raw_fd(), crtc_id)
            },
            _ => None
        }
    }).collect();

    let mut available_modes: Vec<mode::Mode> = Vec::new();
    for (i, _) in connectors.iter().enumerate() {
        let height = modes[i].get_vdisplay();
        let width  = modes[i].get_hdisplay();

        available_modes.push(mode::Mode {
            conn: &connectors[i],
            mode: &modes[i],
            crtc: &crtcs[i].as_ref().unwrap(),
            height: height,
            width: width,
            size: 0,
            stride: 0,
            handle: 0,
            fb: 0,
        })
    }

    // Creating dumb framebuffer
    let mut ffi_create_dumb_struct = ffi::drm_mode_create_dumb::default();
    ffi_create_dumb_struct.width = available_modes[0].width as u32;
    ffi_create_dumb_struct.height = available_modes[0].height as u32;
    ffi_create_dumb_struct.bpp = 32;
    ffi_create_dumb_struct = unsafe {
        let ret = libc::ioctl(
            file.as_raw_fd(),
            DRM_IOCTL_MODE_CREATE_DUMB,
            (&ffi_create_dumb_struct as *const _) as *const libc::c_void
        );
        if ret < 0 {
            None
        } else {
            Some(ffi_create_dumb_struct)
        }
    }.expect("Failed to CREATE_DUMB");
    available_modes[0].stride = ffi_create_dumb_struct.pitch;
    available_modes[0].size   = ffi_create_dumb_struct.size;
    available_modes[0].handle = ffi_create_dumb_struct.handle;

    // Add framebuffer
    let buffer_id = drm::drm_mode::add_fb(
        file.as_raw_fd(),
        available_modes[0].width as u32,
        available_modes[0].height as u32,
        24,
        32,
        available_modes[0].stride,
        available_modes[0].handle,
    ).expect("Failed to ADD_FB");

    // Map dumb framebuffer
    use drm::ffi::drm_mode as ffi;
    let mut ffi_map_dumb_struct = ffi::drm_mode_map_dumb::default();
    ffi_map_dumb_struct.handle = available_modes[0].handle;
    ffi_map_dumb_struct = unsafe {
        let ret = libc::ioctl(
            file.as_raw_fd(),
            DRM_IOCTL_MODE_MAP_DUMB,
            (&ffi_map_dumb_struct as *const _) as *const libc::c_void
        );
        if ret < 0 {
            None
        } else {
            Some(ffi_map_dumb_struct)
        }
    }.expect("Failed to MAP_DUMB");

    // MMAP
    let moption = [
        mmap::MapOption::MapReadable,
        mmap::MapOption::MapWritable,
        mmap::MapOption::MapFd(file.as_raw_fd()),
        mmap::MapOption::MapOffset(ffi_map_dumb_struct.offset as usize),
        mmap::MapOption::MapNonStandardFlags(0x01) // MAP_SHARED
    ];
    let map = mmap::MemoryMap::new(available_modes[0].size as usize, &moption)
        .expect("Failed to MMAP");
    println!("Addr start at {} with size of {}", map.data() as u32, available_modes[0].size);
    // unsafe { libc::memset(map.data() as *mut libc::c_void, 0, available_modes[0].size as usize); };

    // Set CRTC
    drm::drm_mode::set_crtc(
        file.as_raw_fd(),
        available_modes[0].crtc.get_crtc_id(),
        buffer_id,
        0,
        0,
        &[available_modes[0].conn.get_connector_id()],
        available_modes[0].mode
    ).expect("Failed SET_CRTC");

    // Draw
    for i in 0..50 {
        for j in 0..available_modes[0].height {
            for k in 0..available_modes[0].width {
                let r: u32 = unsafe { libc::rand() as u32 % 0xff + ((libc::rand() as u32).wrapping_mul(10)) };
                let g: u32 = unsafe { libc::rand() as u32 % 0xff + ((libc::rand() as u32).wrapping_mul(10)) };
                let b: u32 = unsafe { libc::rand() as u32 % 0xff + ((libc::rand() as u32).wrapping_mul(10)) };
                // println!("R {} : G {} : B {}", r, g, b);

                let offset = available_modes[0].stride * j as u32 + (k as u32 * 4 as u32);
                unsafe { std::ptr::write(map.data().offset(offset as isize) as *mut u32, (r << 16) | (g << 8) | b); }
            }
        }
    }
}