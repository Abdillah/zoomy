extern crate std;

use drm::drm_mode::{ModeInfo, Connector, Crtc};
use drm_buffer::DrmDoubleBuffer;

pub struct Modeset<'a> {
    pub width: u16,
    pub height: u16,

    pub mode: &'a ModeInfo,
    pub conn: &'a Connector,
    pub crtc: &'a Crtc,

    pub dblbuffer: DrmDoubleBuffer
}

impl<'a> Modeset<'a> {
    pub fn new(conn: &'a Connector, mode: &'a ModeInfo, crtc: &'a Crtc,
        dblbuffer: DrmDoubleBuffer, width: u16, height: u16) -> Self {
        Self {
            conn: conn,
            mode: mode,
            crtc: crtc,
            dblbuffer: dblbuffer,
            height: height,
            width: width,
        }
    }
}

impl<'a> std::fmt::Display for Modeset<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "width: {}, height: {}",
            self.width, self.height)
    }
}