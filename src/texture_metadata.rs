use crate::{ffi, macros, HResultError, DXGI_FORMAT};
use core::ptr;

type Result<T> = core::result::Result<T, HResultError>;

macros::c_enum! {
    /// Subset here matches D3D10_RESOURCE_DIMENSION and D3D11_RESOURCE_DIMENSION
    TEX_DIMENSION(u32) => {
        TEX_DIMENSION_TEXTURE1D = 2,
        TEX_DIMENSION_TEXTURE2D = 3,
        TEX_DIMENSION_TEXTURE3D = 4,
    }
}

macros::c_enum! {
    /// Subset here matches D3D10_RESOURCE_MISC_FLAG and D3D11_RESOURCE_MISC_FLAG
    TEX_MISC_FLAG(u32) => {
        TEX_MISC_TEXTURECUBE = 0x4,
    }
}

macros::c_enum! {
    TEX_MISC_FLAG2(u32) => {
        TEX_MISC2_ALPHA_MODE_MASK = 0x7,
    }
}

macros::c_enum! {
    /// Matches DDS_ALPHA_MODE, encoded in MISC_FLAGS2
    TEX_ALPHA_MODE(u32) => {
        TEX_ALPHA_MODE_UNKNOWN = 0,
        TEX_ALPHA_MODE_STRAIGHT = 1,
        TEX_ALPHA_MODE_PREMULTIPLIED = 2,
        TEX_ALPHA_MODE_OPAQUE = 3,
        TEX_ALPHA_MODE_CUSTOM = 4,
    }
}

macros::c_bits! {
    DDS_FLAGS(u32) => {
        /// Assume pitch is DWORD aligned instead of BYTE aligned (used by some legacy DDS files)
        DDS_FLAGS_LEGACY_DWORD = 0x1,

        /// Do not implicitly convert legacy formats that result in larger pixel sizes (24 bpp, 3:3:2, A8L8, A4L4, P8, A8P8)
        DDS_FLAGS_NO_LEGACY_EXPANSION = 0x2,

        /// Do not use work-around for long-standing D3DX DDS file format issue which reversed the 10:10:10:2 color order masks
        DDS_FLAGS_NO_R10B10G10A2_FIXUP = 0x4,

        /// Convert DXGI 1.1 BGR formats to DXGI_FORMAT_R8G8B8A8_UNORM to avoid use of optional WDDM 1.1 formats
        DDS_FLAGS_FORCE_RGB = 0x8,

        /// Conversions avoid use of 565, 5551, and 4444 formats and instead expand to 8888 to avoid use of optional WDDM 1.2 formats
        DDS_FLAGS_NO_16BPP = 0x10,

        /// When loading legacy luminance formats expand replicating the color channels rather than leaving them packed (L8, L16, A8L8)
        DDS_FLAGS_EXPAND_LUMINANCE = 0x20,

        /// Some older DXTn DDS files incorrectly handle mipchain tails for blocks smaller than 4x4
        DDS_FLAGS_BAD_DXTN_TAILS = 0x40,

        /// Allow some file variants due to common bugs in the header written by various leagcy DDS writers
        DDS_FLAGS_PERMISSIVE = 0x80,

        /// Always use the 'DX10' header extension for DDS writer (i.e. don't try to write DX9 compatible DDS files)
        DDS_FLAGS_FORCE_DX10_EXT = 0x10000,

        /// DDS_FLAGS_FORCE_DX10_EXT including miscFlags2 information (result may not be compatible with D3DX10 or D3DX11)
        DDS_FLAGS_FORCE_DX10_EXT_MISC2 = 0x20000,

        /// Force use of legacy header for DDS writer (will fail if unable to write as such)
        DDS_FLAGS_FORCE_DX9_LEGACY = 0x40000,

        /// Force use of 'RXGB' instead of 'DXT5' for DDS write of BC3_UNORM data
        DDS_FLAGS_FORCE_DXT5_RXGB = 0x80000,

        /// Enables the loader to read large dimension .dds files (i.e. greater than known hardware requirements)
        DDS_FLAGS_ALLOW_LARGE_FILES = 0x1000000,
    }
}

impl DDS_FLAGS {
    pub const DDS_FLAGS_NONE: Self = Self::empty();
}

macros::c_bits! {
    TGA_FLAGS(u32) => {
        /// 24bpp files are returned as BGRX; 32bpp files are returned as BGRA
        TGA_FLAGS_BGR = 0x1,

        /// If the loaded image has an all zero alpha channel, normally we assume it should be opaque. This flag leaves it alone.
        TGA_FLAGS_ALLOW_ALL_ZERO_ALPHA = 0x2,

        /// Ignores sRGB TGA 2.0 metadata if present in the file
        TGA_FLAGS_IGNORE_SRGB = 0x10,

        /// Writes sRGB metadata into the file reguardless of format (TGA 2.0 only)
        TGA_FLAGS_FORCE_SRGB = 0x20,

        /// Writes linear gamma metadata into the file reguardless of format (TGA 2.0 only)
        TGA_FLAGS_FORCE_LINEAR = 0x40,

        /// If no colorspace is specified in TGA 2.0 metadata, assume sRGB
        TGA_FLAGS_DEFAULT_SRGB = 0x80,
    }
}

impl TGA_FLAGS {
    pub const TGA_FLAGS_NONE: Self = Self::empty();
}

macros::c_bits! {
    WIC_FLAGS(u32) => {
        /// Loads DXGI 1.1 BGR formats as DXGI_FORMAT_R8G8B8A8_UNORM to avoid use of optional WDDM 1.1 formats
        WIC_FLAGS_FORCE_RGB = 0x1,

        /// Loads DXGI 1.1 X2 10:10:10:2 format as DXGI_FORMAT_R10G10B10A2_UNORM
        WIC_FLAGS_NO_X2_BIAS = 0x2,

        /// Loads 565, 5551, and 4444 formats as 8888 to avoid use of optional WDDM 1.2 formats
        WIC_FLAGS_NO_16BPP = 0x4,

        /// Loads 1-bit monochrome (black & white) as R1_UNORM rather than 8-bit grayscale
        WIC_FLAGS_ALLOW_MONO = 0x8,

        /// Loads all images in a multi-frame file, converting/resizing to match the first frame as needed, defaults to 0th frame otherwise
        WIC_FLAGS_ALL_FRAMES = 0x10,

        /// Ignores sRGB metadata if present in the file
        WIC_FLAGS_IGNORE_SRGB = 0x20,

        /// Writes sRGB metadata into the file reguardless of format
        WIC_FLAGS_FORCE_SRGB = 0x40,

        /// Writes linear gamma metadata into the file reguardless of format
        WIC_FLAGS_FORCE_LINEAR = 0x80,

        /// If no colorspace is specified, assume sRGB
        WIC_FLAGS_DEFAULT_SRGB = 0x100,

        /// Use ordered 4x4 dithering for any required conversions
        WIC_FLAGS_DITHER = 0x10000,

        /// Use error-diffusion dithering for any required conversions
        WIC_FLAGS_DITHER_DIFFUSION = 0x20000,

        WIC_FLAGS_FILTER_POINT = 0x100000,
        WIC_FLAGS_FILTER_LINEAR = 0x200000,
        WIC_FLAGS_FILTER_CUBIC = 0x300000,
        /// Combination of Linear and Box filter
        WIC_FLAGS_FILTER_FANT = 0x400000,
    }
}

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
                source.as_ptr(),
                source.len(),
                flags,
                (&mut metadata).into(),
                match dd_pixel_format {
                    Some(x) => ptr::addr_of_mut!(*x),
                    None => ptr::null_mut(),
                },
            )
        };
        result.success().map(|()| metadata)
    }

    pub fn from_hdr(source: &[u8]) -> Result<Self> {
        let mut metadata = Self::default();
        let result = unsafe {
            ffi::DirectXTexFFI_GetMetadataFromHDRMemory(
                source.as_ptr(),
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
                source.as_ptr(),
                source.len(),
                flags,
                (&mut metadata).into(),
            )
        };
        result.success().map(|()| metadata)
    }
}

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
    use crate::{TexMetadata, DXGI_FORMAT, TEX_ALPHA_MODE, TEX_DIMENSION};
    use std::fs;

    #[test]
    fn from_dds() {
        let file = fs::read("data/ferris_wheel.dds").unwrap();
        let tex = TexMetadata::from_dds(&file, Default::default(), None).unwrap();
        assert_eq!(tex.width, 720);
        assert_eq!(tex.height, 1280);
        assert_eq!(tex.depth, 1);
        assert_eq!(tex.mip_levels, 11);
        assert_eq!(tex.array_size, 1);
        assert_eq!(tex.format, DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM);
        assert_eq!(tex.dimension, TEX_DIMENSION::TEX_DIMENSION_TEXTURE2D);
        assert_eq!(tex.get_alpha_mode(), TEX_ALPHA_MODE::TEX_ALPHA_MODE_UNKNOWN);
    }

    #[test]
    fn from_hdr() {
        let file = fs::read("data/ferris_wheel.hdr").unwrap();
        let tex = TexMetadata::from_hdr(&file).unwrap();
        assert_eq!(tex.width, 720);
        assert_eq!(tex.height, 1280);
        assert_eq!(tex.depth, 1);
        assert_eq!(tex.mip_levels, 1);
        assert_eq!(tex.array_size, 1);
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
        assert_eq!(tex.mip_levels, 1);
        assert_eq!(tex.array_size, 1);
        assert_eq!(tex.format, DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM);
        assert_eq!(tex.dimension, TEX_DIMENSION::TEX_DIMENSION_TEXTURE2D);
        assert_eq!(tex.get_alpha_mode(), TEX_ALPHA_MODE::TEX_ALPHA_MODE_UNKNOWN);
    }
}
