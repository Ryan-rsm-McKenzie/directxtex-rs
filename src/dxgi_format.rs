use crate::{ffi, macros, HResultError};

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

type Result<T> = core::result::Result<T, HResultError>;

#[derive(Clone, Copy, Debug, Default)]
pub struct Pitch {
    pub row: usize,
    pub slice: usize,
}

macros::c_enum! {
    DXGI_FORMAT(u32) => {
        DXGI_FORMAT_UNKNOWN = 0,
        DXGI_FORMAT_R32G32B32A32_TYPELESS = 1,
        DXGI_FORMAT_R32G32B32A32_FLOAT = 2,
        DXGI_FORMAT_R32G32B32A32_UINT = 3,
        DXGI_FORMAT_R32G32B32A32_SINT = 4,
        DXGI_FORMAT_R32G32B32_TYPELESS = 5,
        DXGI_FORMAT_R32G32B32_FLOAT = 6,
        DXGI_FORMAT_R32G32B32_UINT = 7,
        DXGI_FORMAT_R32G32B32_SINT = 8,
        DXGI_FORMAT_R16G16B16A16_TYPELESS = 9,
        DXGI_FORMAT_R16G16B16A16_FLOAT = 10,
        DXGI_FORMAT_R16G16B16A16_UNORM = 11,
        DXGI_FORMAT_R16G16B16A16_UINT = 12,
        DXGI_FORMAT_R16G16B16A16_SNORM = 13,
        DXGI_FORMAT_R16G16B16A16_SINT = 14,
        DXGI_FORMAT_R32G32_TYPELESS = 15,
        DXGI_FORMAT_R32G32_FLOAT = 16,
        DXGI_FORMAT_R32G32_UINT = 17,
        DXGI_FORMAT_R32G32_SINT = 18,
        DXGI_FORMAT_R32G8X24_TYPELESS = 19,
        DXGI_FORMAT_D32_FLOAT_S8X24_UINT = 20,
        DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS = 21,
        DXGI_FORMAT_X32_TYPELESS_G8X24_UINT = 22,
        DXGI_FORMAT_R10G10B10A2_TYPELESS = 23,
        DXGI_FORMAT_R10G10B10A2_UNORM = 24,
        DXGI_FORMAT_R10G10B10A2_UINT = 25,
        DXGI_FORMAT_R11G11B10_FLOAT = 26,
        DXGI_FORMAT_R8G8B8A8_TYPELESS = 27,
        DXGI_FORMAT_R8G8B8A8_UNORM = 28,
        DXGI_FORMAT_R8G8B8A8_UNORM_SRGB = 29,
        DXGI_FORMAT_R8G8B8A8_UINT = 30,
        DXGI_FORMAT_R8G8B8A8_SNORM = 31,
        DXGI_FORMAT_R8G8B8A8_SINT = 32,
        DXGI_FORMAT_R16G16_TYPELESS = 33,
        DXGI_FORMAT_R16G16_FLOAT = 34,
        DXGI_FORMAT_R16G16_UNORM = 35,
        DXGI_FORMAT_R16G16_UINT = 36,
        DXGI_FORMAT_R16G16_SNORM = 37,
        DXGI_FORMAT_R16G16_SINT = 38,
        DXGI_FORMAT_R32_TYPELESS = 39,
        DXGI_FORMAT_D32_FLOAT = 40,
        DXGI_FORMAT_R32_FLOAT = 41,
        DXGI_FORMAT_R32_UINT = 42,
        DXGI_FORMAT_R32_SINT = 43,
        DXGI_FORMAT_R24G8_TYPELESS = 44,
        DXGI_FORMAT_D24_UNORM_S8_UINT = 45,
        DXGI_FORMAT_R24_UNORM_X8_TYPELESS = 46,
        DXGI_FORMAT_X24_TYPELESS_G8_UINT = 47,
        DXGI_FORMAT_R8G8_TYPELESS = 48,
        DXGI_FORMAT_R8G8_UNORM = 49,
        DXGI_FORMAT_R8G8_UINT = 50,
        DXGI_FORMAT_R8G8_SNORM = 51,
        DXGI_FORMAT_R8G8_SINT = 52,
        DXGI_FORMAT_R16_TYPELESS = 53,
        DXGI_FORMAT_R16_FLOAT = 54,
        DXGI_FORMAT_D16_UNORM = 55,
        DXGI_FORMAT_R16_UNORM = 56,
        DXGI_FORMAT_R16_UINT = 57,
        DXGI_FORMAT_R16_SNORM = 58,
        DXGI_FORMAT_R16_SINT = 59,
        DXGI_FORMAT_R8_TYPELESS = 60,
        DXGI_FORMAT_R8_UNORM = 61,
        DXGI_FORMAT_R8_UINT = 62,
        DXGI_FORMAT_R8_SNORM = 63,
        DXGI_FORMAT_R8_SINT = 64,
        DXGI_FORMAT_A8_UNORM = 65,
        DXGI_FORMAT_R1_UNORM = 66,
        DXGI_FORMAT_R9G9B9E5_SHAREDEXP = 67,
        DXGI_FORMAT_R8G8_B8G8_UNORM = 68,
        DXGI_FORMAT_G8R8_G8B8_UNORM = 69,
        DXGI_FORMAT_BC1_TYPELESS = 70,
        DXGI_FORMAT_BC1_UNORM = 71,
        DXGI_FORMAT_BC1_UNORM_SRGB = 72,
        DXGI_FORMAT_BC2_TYPELESS = 73,
        DXGI_FORMAT_BC2_UNORM = 74,
        DXGI_FORMAT_BC2_UNORM_SRGB = 75,
        DXGI_FORMAT_BC3_TYPELESS = 76,
        DXGI_FORMAT_BC3_UNORM = 77,
        DXGI_FORMAT_BC3_UNORM_SRGB = 78,
        DXGI_FORMAT_BC4_TYPELESS = 79,
        DXGI_FORMAT_BC4_UNORM = 80,
        DXGI_FORMAT_BC4_SNORM = 81,
        DXGI_FORMAT_BC5_TYPELESS = 82,
        DXGI_FORMAT_BC5_UNORM = 83,
        DXGI_FORMAT_BC5_SNORM = 84,
        DXGI_FORMAT_B5G6R5_UNORM = 85,
        DXGI_FORMAT_B5G5R5A1_UNORM = 86,
        DXGI_FORMAT_B8G8R8A8_UNORM = 87,
        DXGI_FORMAT_B8G8R8X8_UNORM = 88,
        DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM = 89,
        DXGI_FORMAT_B8G8R8A8_TYPELESS = 90,
        DXGI_FORMAT_B8G8R8A8_UNORM_SRGB = 91,
        DXGI_FORMAT_B8G8R8X8_TYPELESS = 92,
        DXGI_FORMAT_B8G8R8X8_UNORM_SRGB = 93,
        DXGI_FORMAT_BC6H_TYPELESS = 94,
        DXGI_FORMAT_BC6H_UF16 = 95,
        DXGI_FORMAT_BC6H_SF16 = 96,
        DXGI_FORMAT_BC7_TYPELESS = 97,
        DXGI_FORMAT_BC7_UNORM = 98,
        DXGI_FORMAT_BC7_UNORM_SRGB = 99,
        DXGI_FORMAT_AYUV = 100,
        DXGI_FORMAT_Y410 = 101,
        DXGI_FORMAT_Y416 = 102,
        DXGI_FORMAT_NV12 = 103,
        DXGI_FORMAT_P010 = 104,
        DXGI_FORMAT_P016 = 105,
        DXGI_FORMAT_420_OPAQUE = 106,
        DXGI_FORMAT_YUY2 = 107,
        DXGI_FORMAT_Y210 = 108,
        DXGI_FORMAT_Y216 = 109,
        DXGI_FORMAT_NV11 = 110,
        DXGI_FORMAT_AI44 = 111,
        DXGI_FORMAT_IA44 = 112,
        DXGI_FORMAT_P8 = 113,
        DXGI_FORMAT_A8P8 = 114,
        DXGI_FORMAT_B4G4R4A4_UNORM = 115,

        DXGI_FORMAT_P208 = 130,
        DXGI_FORMAT_V208 = 131,
        DXGI_FORMAT_V408 = 132,

        DXGI_FORMAT_SAMPLER_FEEDBACK_MIN_MIP_OPAQUE = 189,
        DXGI_FORMAT_SAMPLER_FEEDBACK_MIP_REGION_USED_OPAQUE = 190,
    }
}

impl DXGI_FORMAT {
    pub const XBOX_DXGI_FORMAT_R10G10B10_7E3_A2_FLOAT: Self = Self(116);
    pub const XBOX_DXGI_FORMAT_R10G10B10_6E4_A2_FLOAT: Self = Self(117);
    pub const XBOX_DXGI_FORMAT_D16_UNORM_S8_UINT: Self = Self(118);
    pub const XBOX_DXGI_FORMAT_R16_UNORM_X8_TYPELESS: Self = Self(119);
    pub const XBOX_DXGI_FORMAT_X16_TYPELESS_G8_UINT: Self = Self(120);

    pub const WIN10_DXGI_FORMAT_P208: Self = Self(130);
    pub const WIN10_DXGI_FORMAT_V208: Self = Self(131);
    pub const WIN10_DXGI_FORMAT_V408: Self = Self(132);

    pub const XBOX_DXGI_FORMAT_R10G10B10_SNORM_A2_UNORM: Self = Self(189);
    pub const XBOX_DXGI_FORMAT_R4G4_UNORM: Self = Self(190);

    pub const WIN11_DXGI_FORMAT_A4B4G4R4_UNORM: Self = Self(191);
}

impl DXGI_FORMAT {
    #[must_use]
    pub fn is_valid(self) -> bool {
        unsafe { ffi::DirectXTexFFI_IsValid(self) }
    }

    #[must_use]
    pub fn is_compressed(self) -> bool {
        unsafe { ffi::DirectXTexFFI_IsCompressed(self) }
    }

    #[must_use]
    pub fn is_packed(self) -> bool {
        unsafe { ffi::DirectXTexFFI_IsPacked(self) }
    }

    #[must_use]
    pub fn is_video(self) -> bool {
        unsafe { ffi::DirectXTexFFI_IsVideo(self) }
    }

    #[must_use]
    pub fn is_planar(self) -> bool {
        unsafe { ffi::DirectXTexFFI_IsPlanar(self) }
    }

    #[must_use]
    pub fn is_palettized(self) -> bool {
        unsafe { ffi::DirectXTexFFI_IsPalettized(self) }
    }

    #[must_use]
    pub fn is_depth_stencil(self) -> bool {
        unsafe { ffi::DirectXTexFFI_IsDepthStencil(self) }
    }

    #[must_use]
    pub fn is_srgb(self) -> bool {
        unsafe { ffi::DirectXTexFFI_IsSRGB(self) }
    }

    #[must_use]
    pub fn is_bgr(self) -> bool {
        unsafe { ffi::DirectXTexFFI_IsBGR(self) }
    }

    #[must_use]
    pub fn is_typeless(self, partial_typeless: bool) -> bool {
        unsafe { ffi::DirectXTexFFI_IsTypeless(self, partial_typeless) }
    }

    #[must_use]
    pub fn has_alpha(self) -> bool {
        unsafe { ffi::DirectXTexFFI_HasAlpha(self) }
    }

    #[must_use]
    pub fn bits_per_pixel(self) -> usize {
        unsafe { ffi::DirectXTexFFI_BitsPerPixel(self) }
    }

    #[must_use]
    pub fn bits_per_color(self) -> usize {
        unsafe { ffi::DirectXTexFFI_BitsPerColor(self) }
    }

    #[must_use]
    pub fn format_data_type(self) -> FORMAT_TYPE {
        unsafe { ffi::DirectXTexFFI_FormatDataType(self) }
    }

    pub fn compute_pitch(self, width: usize, height: usize, flags: CP_FLAGS) -> Result<Pitch> {
        let mut pitch = Pitch::default();
        let result = unsafe {
            ffi::DirectXTexFFI_ComputePitch(
                self,
                width,
                height,
                (&mut pitch.row).into(),
                (&mut pitch.slice).into(),
                flags,
            )
        };
        result.success().map(|()| pitch)
    }

    #[must_use]
    pub fn compute_scanlines(self, height: usize) -> usize {
        unsafe { ffi::DirectXTexFFI_ComputeScanlines(self, height) }
    }

    #[must_use]
    pub fn make_srgb(self) -> Self {
        unsafe { ffi::DirectXTexFFI_MakeSRGB(self) }
    }

    #[must_use]
    pub fn make_linear(self) -> Self {
        unsafe { ffi::DirectXTexFFI_MakeLinear(self) }
    }

    #[must_use]
    pub fn make_typeless(self) -> Self {
        unsafe { ffi::DirectXTexFFI_MakeTypeless(self) }
    }

    #[must_use]
    pub fn make_typeless_unorm(self) -> Self {
        unsafe { ffi::DirectXTexFFI_MakeTypelessUNORM(self) }
    }

    #[must_use]
    pub fn make_typeless_float(self) -> Self {
        unsafe { ffi::DirectXTexFFI_MakeTypelessFLOAT(self) }
    }
}

#[cfg(test)]
mod tests {
    use crate::{CP_FLAGS, DXGI_FORMAT, FORMAT_TYPE};

    #[test]
    fn is_valid() {
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_D32_FLOAT.is_valid(), true);
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_UNKNOWN.is_valid(), false);
    }

    #[test]
    fn is_compressed() {
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_BC1_TYPELESS.is_compressed(), true);
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS.is_compressed(),
            false
        );
    }

    #[test]
    fn is_packed() {
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_R8G8_B8G8_UNORM.is_packed(), true);
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R16G16B16A16_SINT.is_packed(),
            false
        );
    }

    #[test]
    fn is_video() {
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_AYUV.is_video(), true);
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R32G32B32_TYPELESS.is_video(),
            false
        );
    }

    #[test]
    fn is_planar() {
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_NV12.is_planar(), true);
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R9G9B9E5_SHAREDEXP.is_planar(),
            false
        );
    }

    #[test]
    fn is_palettized() {
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_AI44.is_palettized(), true);
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_R8_SNORM.is_palettized(), false);
    }

    #[test]
    fn is_depth_stencil() {
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R32G8X24_TYPELESS.is_depth_stencil(),
            true
        );
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R8G8_SNORM.is_depth_stencil(),
            false
        );
    }

    #[test]
    fn is_srgb() {
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM_SRGB.is_srgb(), true);
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_SNORM.is_srgb(), false);
    }

    #[test]
    fn is_bgr() {
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_B5G6R5_UNORM.is_bgr(), true);
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_BC4_UNORM.is_bgr(), false);
    }

    #[test]
    fn is_typeless() {
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R32G32B32A32_TYPELESS.is_typeless(false),
            true
        );
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS.is_typeless(true),
            true
        );
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS.is_typeless(false),
            false
        );
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS.is_typeless(true),
            true
        );
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_NV11.is_typeless(false), false);
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_NV11.is_typeless(true), false);
    }

    #[test]
    fn has_alpha() {
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R32G32B32A32_TYPELESS.has_alpha(),
            true
        );
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_R32_FLOAT.has_alpha(), false);
    }

    #[test]
    fn bits_per_pixel() {
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R32G32B32A32_TYPELESS.bits_per_pixel(),
            128
        );
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R32G32B32_TYPELESS.bits_per_pixel(),
            96
        );
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R16G16B16A16_TYPELESS.bits_per_pixel(),
            64
        );
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R10G10B10A2_TYPELESS.bits_per_pixel(),
            32
        );
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_P010.bits_per_pixel(), 24);
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_R8G8_TYPELESS.bits_per_pixel(), 16);
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_NV12.bits_per_pixel(), 12);
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_R8_TYPELESS.bits_per_pixel(), 8);
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_R1_UNORM.bits_per_pixel(), 1);
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_BC1_TYPELESS.bits_per_pixel(), 4);
    }

    #[test]
    fn bits_per_color() {
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R32G32B32A32_TYPELESS.bits_per_color(),
            32
        );
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_R24G8_TYPELESS.bits_per_color(), 24);
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R16G16B16A16_TYPELESS.bits_per_color(),
            16
        );
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R9G9B9E5_SHAREDEXP.bits_per_color(),
            14
        );
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R11G11B10_FLOAT.bits_per_color(),
            11
        );
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R10G10B10A2_TYPELESS.bits_per_color(),
            10
        );
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_TYPELESS.bits_per_color(),
            8
        );
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_BC7_TYPELESS.bits_per_color(), 7);
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_BC1_TYPELESS.bits_per_color(), 6);
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_B5G5R5A1_UNORM.bits_per_color(), 5);
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_B4G4R4A4_UNORM.bits_per_color(), 4);
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_R1_UNORM.bits_per_color(), 1);
    }

    #[test]
    fn format_data_type() {
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R16G16B16A16_FLOAT.format_data_type(),
            FORMAT_TYPE::FORMAT_TYPE_FLOAT
        );
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R16G16B16A16_UNORM.format_data_type(),
            FORMAT_TYPE::FORMAT_TYPE_UNORM
        );
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R16G16B16A16_UINT.format_data_type(),
            FORMAT_TYPE::FORMAT_TYPE_UINT
        );
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R16G16B16A16_SNORM.format_data_type(),
            FORMAT_TYPE::FORMAT_TYPE_SNORM
        );
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R16G16B16A16_SINT.format_data_type(),
            FORMAT_TYPE::FORMAT_TYPE_SINT
        );
    }

    #[test]
    fn compute_pitch() {
        let pitch = DXGI_FORMAT::DXGI_FORMAT_BC7_UNORM
            .compute_pitch(0x400, 0x400, CP_FLAGS::CP_FLAGS_NONE)
            .unwrap();
        assert_eq!(pitch.row, 0x1000);
        assert_eq!(pitch.slice, 0x100000);
    }

    #[test]
    fn compute_scanlines() {
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_BC1_TYPELESS.compute_scanlines(64),
            16
        );
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_NV11.compute_scanlines(64), 128);
        assert_eq!(
            DXGI_FORMAT::WIN10_DXGI_FORMAT_V208.compute_scanlines(64),
            128
        );
        assert_eq!(
            DXGI_FORMAT::WIN10_DXGI_FORMAT_V408.compute_scanlines(64),
            192
        );
        assert_eq!(DXGI_FORMAT::DXGI_FORMAT_NV12.compute_scanlines(64), 96);
    }

    #[test]
    fn make_srgb() {
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM.make_srgb(),
            DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM_SRGB
        );
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_BC7_UNORM.make_srgb(),
            DXGI_FORMAT::DXGI_FORMAT_BC7_UNORM_SRGB
        );
    }

    #[test]
    fn make_linear() {
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_BC1_UNORM_SRGB.make_linear(),
            DXGI_FORMAT::DXGI_FORMAT_BC1_UNORM
        );
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_BC2_UNORM_SRGB.make_linear(),
            DXGI_FORMAT::DXGI_FORMAT_BC2_UNORM
        );
    }

    #[test]
    fn make_typeless() {
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R32G32B32A32_FLOAT.make_typeless(),
            DXGI_FORMAT::DXGI_FORMAT_R32G32B32A32_TYPELESS
        );
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R16G16_FLOAT.make_typeless(),
            DXGI_FORMAT::DXGI_FORMAT_R16G16_TYPELESS
        );
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R8G8_UNORM.make_typeless(),
            DXGI_FORMAT::DXGI_FORMAT_R8G8_TYPELESS
        );
    }

    #[test]
    fn make_typeless_unorm() {
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R16G16B16A16_TYPELESS.make_typeless_unorm(),
            DXGI_FORMAT::DXGI_FORMAT_R16G16B16A16_UNORM
        );
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R8_TYPELESS.make_typeless_unorm(),
            DXGI_FORMAT::DXGI_FORMAT_R8_UNORM
        );
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_BC5_TYPELESS.make_typeless_unorm(),
            DXGI_FORMAT::DXGI_FORMAT_BC5_UNORM
        );
    }

    #[test]
    fn make_typeless_float() {
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R32G32B32A32_TYPELESS.make_typeless_float(),
            DXGI_FORMAT::DXGI_FORMAT_R32G32B32A32_FLOAT
        );
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R32G32B32_TYPELESS.make_typeless_float(),
            DXGI_FORMAT::DXGI_FORMAT_R32G32B32_FLOAT
        );
        assert_eq!(
            DXGI_FORMAT::DXGI_FORMAT_R32_TYPELESS.make_typeless_float(),
            DXGI_FORMAT::DXGI_FORMAT_R32_FLOAT
        );
    }
}
