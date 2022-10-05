extern crate alloc;

use alloc::vec::Vec;
use uefi::proto::console::gop::GraphicsOutput;

pub struct Framebuffer {
    pub resolution: (usize, usize),
    pub data: *mut u8
}

impl Framebuffer {
    pub fn from_gop(gop: &mut GraphicsOutput) -> Self {
        Framebuffer {
            resolution: gop.current_mode_info().resolution(),
            data: gop.frame_buffer().as_mut_ptr()
        }
    }
}

pub struct RuntimeContext {
    pub framebuffers: Vec<Framebuffer>
}

impl RuntimeContext {
    pub const fn new() -> Self {
        RuntimeContext {
            framebuffers: Vec::new()
        }
    }
}
