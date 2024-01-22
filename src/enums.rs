use crate::macros;

macros::c_enum! {
    FORMAT_TYPE(u32) => {
        FORMAT_TYPE_TYPELESS = 0,
        FORMAT_TYPE_FLOAT = 1,
        FORMAT_TYPE_UNORM = 2,
        FORMAT_TYPE_SNORM = 3,
        FORMAT_TYPE_UINT = 4,
        FORMAT_TYPE_SINT = 5,
    }
}

macros::c_bits! {
    CP_FLAGS(u32) => {
        /// Assume pitch is DWORD aligned instead of BYTE aligned
        CP_FLAGS_LEGACY_DWORD = 0x1,

        /// Assume pitch is 16-byte aligned instead of BYTE aligned
        CP_FLAGS_PARAGRAPH = 0x2,

        /// Assume pitch is 32-byte aligned instead of BYTE aligned
        CP_FLAGS_YMM = 0x4,

        /// Assume pitch is 64-byte aligned instead of BYTE aligned
        CP_FLAGS_ZMM = 0x8,

        /// Assume pitch is 4096-byte aligned instead of BYTE aligned
        CP_FLAGS_PAGE4K = 0x200,

        /// BC formats with malformed mipchain blocks smaller than 4x4
        CP_FLAGS_BAD_DXTN_TAILS = 0x1000,

        /// Override with a legacy 24 bits-per-pixel format size
        CP_FLAGS_24BPP = 0x10000,

        /// Override with a legacy 16 bits-per-pixel format size
        CP_FLAGS_16BPP = 0x20000,

        /// Override with a legacy 8 bits-per-pixel format size
        CP_FLAGS_8BPP = 0x40000,
    }
}

impl CP_FLAGS {
    /// Normal operation
    pub const CP_FLAGS_NONE: Self = Self::empty();
}

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
