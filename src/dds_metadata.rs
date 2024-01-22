#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct DDSMetaData {
    /// DDPIXELFORMAT.dwSize
    pub size: u32,
    /// DDPIXELFORMAT.dwFlags
    pub flags: u32,
    /// DDPIXELFORMAT.dwFourCC
    pub four_cc: u32,
    /// DDPIXELFORMAT.dwRGBBitCount/dwYUVBitCount/dwAlphaBitDepth/dwLuminanceBitCount/dwBumpBitCount
    pub rgb_bit_count: u32,
    /// DDPIXELFORMAT.dwRBitMask/dwYBitMask/dwLuminanceBitMask/dwBumpDuBitMask
    pub r_bit_mask: u32,
    /// DDPIXELFORMAT.dwGBitMask/dwUBitMask/dwBumpDvBitMask
    pub g_bit_mask: u32,
    /// DDPIXELFORMAT.dwBBitMask/dwVBitMask/dwBumpLuminanceBitMask
    pub b_bit_mask: u32,
    /// DDPIXELFORMAT.dwRGBAlphaBitMask/dwYUVAlphaBitMask/dwLuminanceAlphaBitMask
    pub a_bit_mask: u32,
}

impl DDSMetaData {
    #[must_use]
    pub fn is_dx10(&self) -> bool {
        self.four_cc == 0x30315844
    }
}

#[cfg(test)]
mod tests {
    use crate::{ffi, DDSMetaData};
    use core::mem;

    #[test]
    fn verify_layout() {
        assert_eq!(mem::size_of::<DDSMetaData>(), unsafe {
            ffi::DirectXTexFFI_DDSMetaData_Sizeof()
        });
        assert_eq!(mem::align_of::<DDSMetaData>(), unsafe {
            ffi::DirectXTexFFI_DDSMetaData_Alignof()
        });
    }
}
