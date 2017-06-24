extern crate libc;
extern crate mmap;
extern crate drm;

use std::ptr;
use std::os::unix::io::RawFd;
use drm::ffi::drm_mode as ffi;

const DRM_IOCTL_MODE_CREATE_DUMB: u64 = 0b11000000001000000110010010110010;
const DRM_IOCTL_MODE_MAP_DUMB: u64    = 0b11000000000100000110010010110011;

pub struct DrmDoubleBuffer {
    active: usize,
    pair: Vec<DrmBuffer>,
}

impl DrmDoubleBuffer {
    pub fn new(b1: DrmBuffer, b2: DrmBuffer) -> Self {
        Self {
            active: 0,
            pair: vec!(b1, b2)
        }
    }

    pub fn switch(self: &mut Self) {
        self.active = self.active ^ 1;
    }

    pub fn get_back_buffer_mut(self: &mut Self) -> &mut DrmBuffer {
        &mut self.pair[self.active]
    }
}

pub struct DrmBuffer {
    pub id: u32,
    pub width: u16,
    pub height: u16,
    pub stride: u32,
    pub size: u64,
    pub offset: u64,
    handle: u32,

    map: mmap::MemoryMap,
}

impl DrmBuffer {
    pub fn new(fd: RawFd, w: u16, h: u16) -> Self {
        // Initialize DRM buffer
        let (size, w, h, stride, handle) = DrmBuffer::create_dumb_fb(fd, w, h);
        let buffer_id = drm::drm_mode::add_fb(fd, w as u32, h as u32, 24, 32, stride, handle)
            .expect("Failed to ADD_FB");
        let (map, offset) = DrmBuffer::map_dump_fb(fd, size, handle);

        DrmBuffer {
            id: buffer_id,
            size: size,
            height: h,
            width: w,
            stride: stride,
            handle: handle,
            offset: offset,
            map: map,
        }
    }

    pub fn write(self: &mut Self, offset: u32, val: u32) {
        unsafe {
            ptr::write(self.map.data().offset(offset as isize) as *mut u32, val);
        }
    }

    fn create_dumb_fb(fd: RawFd, w: u16, h: u16) -> (u64, u16, u16, u32, u32) {
        let mut ffi_create_dumb_struct = ffi::drm_mode_create_dumb::default();
        ffi_create_dumb_struct.width = w as u32;
        ffi_create_dumb_struct.height = h as u32;
        ffi_create_dumb_struct.bpp = 32;
        ffi_create_dumb_struct = unsafe {
            let ret = libc::ioctl(
                fd,
                DRM_IOCTL_MODE_CREATE_DUMB,
                (&ffi_create_dumb_struct as *const _) as *const libc::c_void
            );
            if ret < 0 {
                None
            } else {
                Some(ffi_create_dumb_struct)
            }
        }.expect("Failed to CREATE_DUMB");

        let stride = ffi_create_dumb_struct.pitch;
        let size   = ffi_create_dumb_struct.size;
        let handle = ffi_create_dumb_struct.handle;

        (size, w, h, stride, handle)
    }

    fn map_dump_fb(fd: RawFd, size: u64, handle: u32) -> (mmap::MemoryMap, u64) {
        // Map dumb framebuffer
        let mut ffi_map_dumb_struct = ffi::drm_mode_map_dumb::default();
        ffi_map_dumb_struct.handle = handle;
        ffi_map_dumb_struct = unsafe {
            let ret = libc::ioctl(
                fd,
                DRM_IOCTL_MODE_MAP_DUMB,
                (&ffi_map_dumb_struct as *const _) as *const libc::c_void
            );
            if ret < 0 {
                None
            } else {
                Some(ffi_map_dumb_struct)
            }
        }.expect("Failed to MAP_DUMB");
        let offset = ffi_map_dumb_struct.offset;

        // Create new MemoryMap
        let moption = [
            mmap::MapOption::MapReadable,
            mmap::MapOption::MapWritable,
            mmap::MapOption::MapFd(fd),
            mmap::MapOption::MapOffset(offset as usize),
            mmap::MapOption::MapNonStandardFlags(0x01) // MAP_SHARED
        ];

        let map = mmap::MemoryMap::new(size as usize, &moption)
            .expect("Failed to MMAP");

        (map, offset)
    }
}

impl ::std::fmt::Display for DrmBuffer {
    fn fmt(self: &Self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "id: {}, size: {}, height: {}, width: {}, stride: {}, handle: {}, offset: {}",
            self.id,
            self.size,
            self.height,
            self.width,
            self.stride,
            self.handle,
            self.offset)
    }
}