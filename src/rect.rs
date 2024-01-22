#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

#[cfg(test)]
mod tests {
    use crate::{ffi, Rect};
    use core::mem;

    #[test]
    fn verify_layout() {
        assert_eq!(mem::size_of::<Rect>(), unsafe {
            ffi::DirectXTexFFI_Rect_Sizeof()
        });
        assert_eq!(mem::align_of::<Rect>(), unsafe {
            ffi::DirectXTexFFI_Rect_Alignof()
        });
    }
}
