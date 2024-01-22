use crate::{
    ffi::{self, prelude::*},
    DDSMetaData, Result, DDS_FLAGS, DXGI_FORMAT, TEX_ALPHA_MODE, TEX_DIMENSION, TEX_MISC_FLAG,
    TEX_MISC_FLAG2, TGA_FLAGS,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(C)]
pub struct TexMetadata {
    pub width: usize,
    /// Should be 1 for 1D textures
    pub height: usize,
    /// Should be 1 for 1D or 2D textures
    pub depth: usize,
    /// For cubemap, this is a multiple of 6
    pub array_size: usize,
    pub mip_levels: usize,
    pub misc_flags: u32,
    pub misc_flags2: u32,
    pub format: DXGI_FORMAT,
    pub dimension: TEX_DIMENSION,
}

impl TexMetadata {
    #[must_use]
    /// Returns `None` to indicate an out-of-range error
    pub fn compute_index(&self, mip: usize, item: usize, slice: usize) -> Option<usize> {
        let result =
            unsafe { ffi::DirectXTexFFI_TexMetadata_ComputIndex(self.into(), mip, item, slice) };
        (result != usize::MAX).then_some(result)
    }

    #[must_use]
    /// Helper for [`misc_flags`](Self::misc_flags)
    pub fn is_cubemap(&self) -> bool {
        (self.misc_flags & TEX_MISC_FLAG::TEX_MISC_TEXTURECUBE.bits()) != 0
    }

    #[must_use]
    /// Helpers for [`misc_flags2`](Self::misc_flags2)
    pub fn is_pm_alpha(&self) -> bool {
        (self.misc_flags2 & TEX_MISC_FLAG2::TEX_MISC2_ALPHA_MODE_MASK.bits())
            == TEX_ALPHA_MODE::TEX_ALPHA_MODE_PREMULTIPLIED.into()
    }

    /// Helpers for [`misc_flags2`](Self::misc_flags2)
    pub fn set_alpha_mode(&mut self, mode: TEX_ALPHA_MODE) {
        self.misc_flags2 =
            (self.misc_flags2 & !TEX_MISC_FLAG2::TEX_MISC2_ALPHA_MODE_MASK.bits()) | mode.bits();
    }

    #[must_use]
    /// Helpers for [`misc_flags2`](Self::misc_flags2)
    pub fn get_alpha_mode(&self) -> TEX_ALPHA_MODE {
        (self.misc_flags2 & TEX_MISC_FLAG2::TEX_MISC2_ALPHA_MODE_MASK.bits()).into()
    }

    #[must_use]
    /// Helper for [`dimension`](Self::dimension)
    pub fn is_volumemap(&self) -> bool {
        self.dimension == TEX_DIMENSION::TEX_DIMENSION_TEXTURE3D
    }

    pub fn from_dds(
        source: &[u8],
        flags: DDS_FLAGS,
        dd_pixel_format: Option<&mut DDSMetaData>,
    ) -> Result<Self> {
        let mut metadata = Self::default();
        let result = unsafe {
            ffi::DirectXTexFFI_GetMetadataFromDDSMemoryEx(
                source.as_ffi_ptr(),
                source.len(),
                flags,
                (&mut metadata).into(),
                dd_pixel_format.into_ffi_ptr(),
            )
        };
        result.success().map(|()| metadata)
    }

    pub fn from_hdr(source: &[u8]) -> Result<Self> {
        let mut metadata = Self::default();
        let result = unsafe {
            ffi::DirectXTexFFI_GetMetadataFromHDRMemory(
                source.as_ffi_ptr(),
                source.len(),
                (&mut metadata).into(),
            )
        };
        result.success().map(|()| metadata)
    }

    pub fn from_tga(source: &[u8], flags: TGA_FLAGS) -> Result<Self> {
        let mut metadata = Self::default();
        let result = unsafe {
            ffi::DirectXTexFFI_GetMetadataFromTGAMemory(
                source.as_ffi_ptr(),
                source.len(),
                flags,
                (&mut metadata).into(),
            )
        };
        result.success().map(|()| metadata)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ffi, TexMetadata, DXGI_FORMAT, TEX_ALPHA_MODE, TEX_DIMENSION};
    use core::mem;
    use std::fs;

    #[test]
    fn verify_layout() {
        assert_eq!(mem::size_of::<TexMetadata>(), unsafe {
            ffi::DirectXTexFFI_TexMetadata_Sizeof()
        });
        assert_eq!(mem::align_of::<TexMetadata>(), unsafe {
            ffi::DirectXTexFFI_TexMetadata_Alignof()
        });
    }

    #[test]
    fn from_dds() {
        let file = fs::read("data/ferris_wheel.dds").unwrap();
        let (tex, dds) = {
            let mut dds = Default::default();
            let tex = TexMetadata::from_dds(&file, Default::default(), Some(&mut dds)).unwrap();
            (tex, dds)
        };

        assert_eq!(tex.width, 720);
        assert_eq!(tex.height, 1280);
        assert_eq!(tex.depth, 1);
        assert_eq!(tex.array_size, 1);
        assert_eq!(tex.mip_levels, 11);
        assert_eq!(tex.misc_flags, 0);
        assert_eq!(tex.misc_flags2, 0);
        assert_eq!(tex.format, DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM);
        assert_eq!(tex.dimension, TEX_DIMENSION::TEX_DIMENSION_TEXTURE2D);
        assert_eq!(tex.get_alpha_mode(), TEX_ALPHA_MODE::TEX_ALPHA_MODE_UNKNOWN);

        assert_eq!(dds.size, 32);
        assert_eq!(dds.flags, 65);
        assert_eq!(dds.four_cc, 0);
        assert_eq!(dds.rgb_bit_count, 32);
        assert_eq!(dds.r_bit_mask, 0x000000FF);
        assert_eq!(dds.g_bit_mask, 0x0000FF00);
        assert_eq!(dds.b_bit_mask, 0x00FF0000);
        assert_eq!(dds.a_bit_mask, 0xFF000000);
    }

    #[test]
    fn from_hdr() {
        let file = fs::read("data/ferris_wheel.hdr").unwrap();
        let tex = TexMetadata::from_hdr(&file).unwrap();
        assert_eq!(tex.width, 720);
        assert_eq!(tex.height, 1280);
        assert_eq!(tex.depth, 1);
        assert_eq!(tex.array_size, 1);
        assert_eq!(tex.mip_levels, 1);
        assert_eq!(tex.misc_flags, 0);
        assert_eq!(tex.misc_flags2, 3);
        assert_eq!(tex.format, DXGI_FORMAT::DXGI_FORMAT_R32G32B32A32_FLOAT);
        assert_eq!(tex.dimension, TEX_DIMENSION::TEX_DIMENSION_TEXTURE2D);
        assert_eq!(tex.get_alpha_mode(), TEX_ALPHA_MODE::TEX_ALPHA_MODE_OPAQUE);
    }

    #[test]
    fn from_tga() {
        let file = fs::read("data/ferris_wheel.tga").unwrap();
        let tex = TexMetadata::from_tga(&file, Default::default()).unwrap();
        assert_eq!(tex.width, 720);
        assert_eq!(tex.height, 1280);
        assert_eq!(tex.depth, 1);
        assert_eq!(tex.array_size, 1);
        assert_eq!(tex.mip_levels, 1);
        assert_eq!(tex.misc_flags, 0);
        assert_eq!(tex.misc_flags2, 0);
        assert_eq!(tex.format, DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM);
        assert_eq!(tex.dimension, TEX_DIMENSION::TEX_DIMENSION_TEXTURE2D);
        assert_eq!(tex.get_alpha_mode(), TEX_ALPHA_MODE::TEX_ALPHA_MODE_UNKNOWN);
    }
}
