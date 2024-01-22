pub const TEX_FILTER_DITHER_MASK: u32 = 0xF0000;
pub const TEX_FILTER_MODE_MASK: u32 = 0xF00000;
pub const TEX_FILTER_SRGB_MASK: u32 = 0xF000000;

/// Default value for alpha threshold used when converting to 1-bit alpha
pub const TEX_THRESHOLD_DEFAULT: f32 = 0.5;

/// Default value for alpha weight used for GPU BC7 compression
pub const TEX_ALPHA_WEIGHT_DEFAULT: f32 = 1.0;
