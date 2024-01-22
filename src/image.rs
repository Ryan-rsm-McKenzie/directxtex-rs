use crate::DXGI_FORMAT;
use core::ptr;

#[derive(Debug)]
#[repr(C)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub format: DXGI_FORMAT,
    pub row_pitch: usize,
    pub slice_pitch: usize,
    pub pixels: *mut u8,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            format: DXGI_FORMAT::default(),
            row_pitch: 0,
            slice_pitch: 0,
            pixels: ptr::null_mut(),
        }
    }
}
