extern crate alloc;

use alloc::vec::Vec;
use uefi::proto::console::gop::GraphicsOutput;

pub struct Framebuffer {
    pub resolution: (usize, usize),
    pub stride: usize,
    pub data: *mut u8
}

impl Framebuffer {
    pub fn from_gop(gop: &mut GraphicsOutput) -> Self {
        let mode_info = gop.current_mode_info();
        Framebuffer {
            resolution: mode_info.resolution(),
            stride: mode_info.stride(),
            data: gop.frame_buffer().as_mut_ptr()
        }
    }

    pub unsafe fn pixel_offset(&self, x: usize, y: usize) -> *mut u8 {
        let index: isize = (y * self.stride + x).try_into().unwrap();
        self.data.offset(index * 4)
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
