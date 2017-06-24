extern crate libc;
extern crate drm;
extern crate mmap;

mod mode;
mod buffer;

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

    let mut modeset;
    let mut available_modes: Vec<mode::Modeset> = Vec::new();
    for (i, _) in connectors.iter().enumerate() {
        let height = modes[i].get_vdisplay();
        let width  = modes[i].get_hdisplay();

        let buffer = ::buffer::DrmBuffer::new(file.as_raw_fd(), width, height);
        modeset = mode::Modeset::new(&connectors[i], &modes[i], &crtcs[i].as_ref().unwrap(),
            buffer, height, width);
        available_modes.push(modeset)
    }

    let ref mut active_mode = available_modes[0];

    // Set CRTC
    drm::drm_mode::set_crtc(
        file.as_raw_fd(),
        active_mode.crtc.get_crtc_id(),
        active_mode.buffer.id,
        0,
        0,
        &[active_mode.conn.get_connector_id()],
        active_mode.mode
    ).expect("Failed SET_CRTC");

    // Draw
    for _ in 0..50 {
        for j in 0..active_mode.buffer.height {
            for k in 0..active_mode.buffer.width {
                let r: u8 = unsafe { (libc::rand() as u8 % 0xff).wrapping_add((libc::rand() as u8).wrapping_mul(10)) };
                let g: u8 = unsafe { (libc::rand() as u8 % 0xff).wrapping_add((libc::rand() as u8).wrapping_mul(10)) };
                let b: u8 = unsafe { (libc::rand() as u8 % 0xff).wrapping_add((libc::rand() as u8).wrapping_mul(10)) };

                let color: u32 = ((r as u32) << 16) | ((g as u32) << 8) | b as u32;
                // println!("R {} + G {} + B {} = Color {}", r, g, b, color);

                let offset = active_mode.buffer.stride * j as u32 + (k as u32 * 4 as u32);
                active_mode.buffer.write(offset, color);
            }
        }
    }
}