extern crate libc;

use drm::drm_mode::{ModeInfo, Connector, Crtc};

pub struct Mode<'a> {
	pub width: u16,
	pub height: u16,
	pub stride: u32,
	pub size: u64,
	pub handle: u32,
    // map: libc::uint8_t,

	pub mode: &'a ModeInfo,
	pub fb: libc::uint32_t,
	pub conn: &'a Connector,
	pub crtc: &'a Crtc,
}

impl<'a> Mode<'a> {
	pub fn new(conn: &'a Connector, mode: &'a ModeInfo, crtc: &'a Crtc) -> Self {
		Self {
            conn: conn,
            mode: mode,
            crtc: crtc,
            height: 0,
            width: 0,
            size: 0,
            stride: 0,
            handle: 0,
            fb: 0,
        }
	}

	pub fn set_dimension(&mut self, width: u16, height: u16) {
		self.width = width;
		self.height = height;
	}
}