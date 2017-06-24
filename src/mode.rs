extern crate std;

use drm::drm_mode::{ModeInfo, Connector, Crtc};

pub struct Modeset<'a> {
    pub width: u16,
    pub height: u16,

    pub mode: &'a ModeInfo,
    pub conn: &'a Connector,
    pub crtc: &'a Crtc,

    pub buffer: &'a buffer::DrmBuffer
}

impl<'a> Modeset<'a> {
    pub fn new(conn: &'a Connector, mode: &'a ModeInfo, crtc: &'a Crtc, buffer: buffer::DrmBuffer) -> Self {
        Self {
            conn: conn,
            mode: mode,
            crtc: crtc,
            buffer: buffer,
            height: 0,
            width: 0,
        }
    }
}

impl<'a> std::fmt::Display for Modeset<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "width: {}, height: {}",
            self.width, self.height)
    }
}