#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct DDSMetaData {
    pub size: u32,
    pub flags: u32,
    pub four_cc: u32,
    pub rgb_bit_count: u32,
    pub r_bit_mask: u32,
    pub g_bit_mask: u32,
    pub b_bit_mask: u32,
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
