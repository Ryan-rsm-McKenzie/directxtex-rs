use crate::macros;

macros::c_enum! {
    format_type::FORMAT_TYPE(u32) => {
        FORMAT_TYPE_TYPELESS = 0,
        FORMAT_TYPE_FLOAT = 1,
        FORMAT_TYPE_UNORM = 2,
        FORMAT_TYPE_SNORM = 3,
        FORMAT_TYPE_UINT = 4,
        FORMAT_TYPE_SINT = 5,
    }
}

macros::c_bits! {
    cp_flags::CP_FLAGS(u32) => {
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

    extra => {
        /// Normal operation
        CP_FLAGS_NONE = CP_FLAGS::empty();
    }
}

macros::c_enum! {
    /// Subset here matches D3D10_RESOURCE_DIMENSION and D3D11_RESOURCE_DIMENSION
    tex_dimension::TEX_DIMENSION(u32) => {
        TEX_DIMENSION_TEXTURE1D = 2,
        TEX_DIMENSION_TEXTURE2D = 3,
        TEX_DIMENSION_TEXTURE3D = 4,
    }
}

macros::c_enum! {
    /// Subset here matches D3D10_RESOURCE_MISC_FLAG and D3D11_RESOURCE_MISC_FLAG
    tex_misc_flag::TEX_MISC_FLAG(u32) => {
        TEX_MISC_TEXTURECUBE = 0x4,
    }
}

macros::c_enum! {
    tex_misc_flag2::TEX_MISC_FLAG2(u32) => {
        TEX_MISC2_ALPHA_MODE_MASK = 0x7,
    }
}

macros::c_enum! {
    /// Matches DDS_ALPHA_MODE, encoded in MISC_FLAGS2
    tex_alpha_mode::TEX_ALPHA_MODE(u32) => {
        TEX_ALPHA_MODE_UNKNOWN = 0,
        TEX_ALPHA_MODE_STRAIGHT = 1,
        TEX_ALPHA_MODE_PREMULTIPLIED = 2,
        TEX_ALPHA_MODE_OPAQUE = 3,
        TEX_ALPHA_MODE_CUSTOM = 4,
    }
}

macros::c_bits! {
    dds_flags::DDS_FLAGS(u32) => {
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

    extra => {
        DDS_FLAGS_NONE = DDS_FLAGS::empty();
    }
}

macros::c_bits! {
    tga_flags::TGA_FLAGS(u32) => {
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

    extra => {
        TGA_FLAGS_NONE = TGA_FLAGS::empty();
    }
}

macros::c_bits! {
    tex_filter_flags::TEX_FILTER_FLAGS(u32) => {
        /// Wrap vs. Mirror vs. Clamp filtering options
        TEX_FILTER_WRAP_U = 0x1,
        /// Wrap vs. Mirror vs. Clamp filtering options
        TEX_FILTER_WRAP_V = 0x2,
        /// Wrap vs. Mirror vs. Clamp filtering options
        TEX_FILTER_WRAP_W = 0x4,
        /// Wrap vs. Mirror vs. Clamp filtering options
        TEX_FILTER_MIRROR_U = 0x10,
        /// Wrap vs. Mirror vs. Clamp filtering options
        TEX_FILTER_MIRROR_V = 0x20,
        /// Wrap vs. Mirror vs. Clamp filtering options
        TEX_FILTER_MIRROR_W = 0x40,

        /// Resize color and alpha channel independently
        TEX_FILTER_SEPARATE_ALPHA = 0x100,

        /// Enable *2 - 1 conversion cases for unorm<->float and positive-only float formats
        TEX_FILTER_FLOAT_X2BIAS = 0x200,

        /// When converting RGB(A) to R, defaults to using grayscale. These flags indicate copying a specific channel instead
        ///
        /// When converting RGB(A) to RG, defaults to copying RED | GREEN. These flags control which channels are selected instead.
        TEX_FILTER_RGB_COPY_RED = 0x1000,
        /// When converting RGB(A) to R, defaults to using grayscale. These flags indicate copying a specific channel instead
        ///
        /// When converting RGB(A) to RG, defaults to copying RED | GREEN. These flags control which channels are selected instead.
        TEX_FILTER_RGB_COPY_GREEN = 0x2000,
        /// When converting RGB(A) to R, defaults to using grayscale. These flags indicate copying a specific channel instead
        ///
        /// When converting RGB(A) to RG, defaults to copying RED | GREEN. These flags control which channels are selected instead.
        TEX_FILTER_RGB_COPY_BLUE = 0x4000,
        /// When converting RGB(A) to R, defaults to using grayscale. These flags indicate copying a specific channel instead
        ///
        /// When converting RGB(A) to RG, defaults to copying RED | GREEN. These flags control which channels are selected instead.
        TEX_FILTER_RGB_COPY_ALPHA = 0x8000,

        /// Use ordered 4x4 dithering for any required conversions
        TEX_FILTER_DITHER = 0x10000,
        /// Use error-diffusion dithering for any required conversions
        TEX_FILTER_DITHER_DIFFUSION = 0x20000,

        /// Filtering mode to use for any required image resizing
        TEX_FILTER_POINT = 0x100000,
        /// Filtering mode to use for any required image resizing
        TEX_FILTER_LINEAR = 0x200000,
        /// Filtering mode to use for any required image resizing
        TEX_FILTER_CUBIC = 0x300000,
        /// Filtering mode to use for any required image resizing
        TEX_FILTER_BOX = 0x400000,
        /// Filtering mode to use for any required image resizing
        ///
        /// Equiv to Box filtering for mipmap generation
        TEX_FILTER_FANT = 0x400000,
        /// Filtering mode to use for any required image resizing
        TEX_FILTER_TRIANGLE = 0x500000,

        /// sRGB <-> RGB for use in conversion operations
        ///
        /// if the input format type is IsSRGB(), then SRGB_IN is on by default
        TEX_FILTER_SRGB_IN = 0x1000000,
        /// sRGB <-> RGB for use in conversion operations
        ///
        /// if the output format type is IsSRGB(), then SRGB_OUT is on by default
        TEX_FILTER_SRGB_OUT = 0x2000000,

        /// Forces use of the non-WIC path when both are an option
        TEX_FILTER_FORCE_NON_WIC = 0x10000000,

        /// Forces use of the WIC path even when logic would have picked a non-WIC path when both are an option
        TEX_FILTER_FORCE_WIC = 0x20000000,
    }

    extra => {
        TEX_FILTER_DEFAULT = TEX_FILTER_FLAGS::empty();

        /// Wrap vs. Mirror vs. Clamp filtering options
        TEX_FILTER_WRAP = TEX_FILTER_FLAGS::TEX_FILTER_WRAP_U
            .union(TEX_FILTER_FLAGS::TEX_FILTER_WRAP_V)
            .union(TEX_FILTER_FLAGS::TEX_FILTER_WRAP_W);

        /// Wrap vs. Mirror vs. Clamp filtering options
        TEX_FILTER_MIRROR = TEX_FILTER_FLAGS::TEX_FILTER_MIRROR_U
            .union(TEX_FILTER_FLAGS::TEX_FILTER_MIRROR_V)
            .union(TEX_FILTER_FLAGS::TEX_FILTER_MIRROR_W);

        /// sRGB <-> RGB for use in conversion operations
        ///
        /// if the output format type is IsSRGB(), then SRGB_OUT is on by default
        TEX_FILTER_SRGB = TEX_FILTER_FLAGS::TEX_FILTER_SRGB_IN
            .union(TEX_FILTER_FLAGS::TEX_FILTER_SRGB_OUT);
    }
}

macros::c_bits! {
    tex_pmalpha_flags::TEX_PMALPHA_FLAGS(u32) => {
        /// ignores sRGB colorspace conversions
        TEX_PMALPHA_IGNORE_SRGB = 0x1,

        /// converts from premultiplied alpha back to straight alpha
        TEX_PMALPHA_REVERSE = 0x2,

        /// if the input format type is IsSRGB(), then SRGB_IN is on by default
        TEX_PMALPHA_SRGB_IN = 0x1000000,
        /// if the output format type is IsSRGB(), then SRGB_OUT is on by default
        TEX_PMALPHA_SRGB_OUT = 0x2000000,
    }

    extra => {
        TEX_PMALPHA_DEFAULT = TEX_PMALPHA_FLAGS::empty();

        /// if the input format type is IsSRGB(), then SRGB_IN is on by default
        ///
        /// if the output format type is IsSRGB(), then SRGB_OUT is on by default
        TEX_PMALPHA_SRGB = TEX_PMALPHA_FLAGS::TEX_PMALPHA_SRGB_IN
            .union(TEX_PMALPHA_FLAGS::TEX_PMALPHA_SRGB_OUT);
    }
}

macros::c_bits! {
    tex_compress_flags::TEX_COMPRESS_FLAGS(u32) => {
        /// Enables dithering RGB colors for BC1-3 compression
        TEX_COMPRESS_RGB_DITHER = 0x10000,

        /// Enables dithering alpha for BC1-3 compression
        TEX_COMPRESS_A_DITHER = 0x20000,

        /// Enables both RGB and alpha dithering for BC1-3 compression
        TEX_COMPRESS_DITHER = 0x30000,

        /// Uniform color weighting for BC1-3 compression; by default uses perceptual weighting
        TEX_COMPRESS_UNIFORM = 0x40000,

        /// Enables exhaustive search for BC7 compress for mode 0 and 2; by default skips trying these modes
        TEX_COMPRESS_BC7_USE_3SUBSETS = 0x80000,

        /// Minimal modes (usually mode 6) for BC7 compression
        TEX_COMPRESS_BC7_QUICK = 0x100000,

        /// if the input format type is IsSRGB(), then SRGB_IN is on by default
        TEX_COMPRESS_SRGB_IN = 0x1000000,
        /// if the output format type is IsSRGB(), then SRGB_OUT is on by default
        TEX_COMPRESS_SRGB_OUT = 0x2000000,

        /// Compress is free to use multithreading to improve performance (by default it does not use multithreading)
        TEX_COMPRESS_PARALLEL = 0x10000000,
    }

    extra => {
        TEX_COMPRESS_DEFAULT = TEX_COMPRESS_FLAGS::empty();

        /// if the input format type is IsSRGB(), then SRGB_IN is on by default
        ///
        /// if the output format type is IsSRGB(), then SRGB_OUT is on by default
        TEX_COMPRESS_SRGB = TEX_COMPRESS_FLAGS::TEX_COMPRESS_SRGB_IN
            .union(TEX_COMPRESS_FLAGS::TEX_COMPRESS_SRGB_OUT);
    }
}

macros::c_bits! {
    cnmap_flags::CNMAP_FLAGS(u32) => {
        /// Channel selection when evaluting color value for height
        CNMAP_CHANNEL_RED = 0x1,
        /// Channel selection when evaluting color value for height
        CNMAP_CHANNEL_GREEN = 0x2,
        /// Channel selection when evaluting color value for height
        CNMAP_CHANNEL_BLUE = 0x3,
        /// Channel selection when evaluting color value for height
        CNMAP_CHANNEL_ALPHA = 0x4,
        /// Channel selection when evaluting color value for height
        ///
        /// Luminance is a combination of red, green, and blue
        CNMAP_CHANNEL_LUMINANCE = 0x5,

        /// Use mirror semantics for scanline references (defaults to wrap)
        CNMAP_MIRROR_U = 0x1000,
        /// Use mirror semantics for scanline references (defaults to wrap)
        CNMAP_MIRROR_V = 0x2000,
        /// Use mirror semantics for scanline references (defaults to wrap)
        CNMAP_MIRROR = 0x3000,

        /// Inverts normal sign
        CNMAP_INVERT_SIGN = 0x4000,

        /// Computes a crude occlusion term stored in the alpha channel
        CNMAP_COMPUTE_OCCLUSION = 0x8000,
    }

    extra => {
        CNMAP_DEFAULT = CNMAP_FLAGS::empty();
    }
}

macros::c_bits! {
    cmse_flags::CMSE_FLAGS(u32) => {
        /// Indicates that image needs gamma correction before comparision
        CMSE_IMAGE1_SRGB = 0x1,
        /// Indicates that image needs gamma correction before comparision
        CMSE_IMAGE2_SRGB = 0x2,

        /// Ignore the channel when computing MSE
        CMSE_IGNORE_RED = 0x10,
        /// Ignore the channel when computing MSE
        CMSE_IGNORE_GREEN = 0x20,
        /// Ignore the channel when computing MSE
        CMSE_IGNORE_BLUE = 0x40,
        /// Ignore the channel when computing MSE
        CMSE_IGNORE_ALPHA = 0x80,

        /// Indicates that image should be scaled and biased before comparison (i.e. UNORM -> SNORM)
        CMSE_IMAGE1_X2_BIAS = 0x100,
        /// Indicates that image should be scaled and biased before comparison (i.e. UNORM -> SNORM)
        CMSE_IMAGE2_X2_BIAS = 0x200,
    }

    extra => {
        CMSE_DEFAULT = CMSE_FLAGS::empty();
    }
}
