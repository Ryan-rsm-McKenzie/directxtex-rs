use crate::ffi;
use std::{
    error,
    fmt::{self, Debug, Display, Formatter},
};

#[derive(Debug)]
pub struct InvalidRepresentation<T>(T);

impl<T> Display for InvalidRepresentation<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} is not a valid variant of the enumeration", self.0)
    }
}

impl<T> error::Error for InvalidRepresentation<T> where T: Debug + Display {}

macro_rules! c_enum {
	($enumeration:ident($underlying:ty) => { $($variant:ident = $value:literal,)* }) => {
		#[allow(non_camel_case_types)]
		#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
		#[repr(transparent)]
		pub struct $enumeration($underlying);

		impl $enumeration {
			$(pub const $variant: Self = Self($value);)*

			pub fn from_unchecked(value: $underlying) -> Self {
				Self(value)
			}
		}

		impl From<$enumeration> for $underlying {
			fn from(value: $enumeration) -> Self {
				value.0
			}
		}

		impl TryFrom<$underlying> for $enumeration {
			type Error = InvalidRepresentation<$underlying>;

			fn try_from(value: $underlying) -> Result<Self, Self::Error> {
				match value {
					$($value => Ok(Self::$variant),)*
					_ => Err(InvalidRepresentation(value))
				}
			}
		}

		impl Display for $enumeration {
			fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
				match self.0 {
					$($value => write!(f, "{}", std::stringify!($variant)),)*
					unk => write!(f, "{unk:x}"),
				}
			}
		}
	};
}

c_enum! {
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

impl Default for DXGI_FORMAT {
    fn default() -> Self {
        Self::DXGI_FORMAT_UNKNOWN
    }
}

impl DXGI_FORMAT {
    pub fn is_valid(self) -> bool {
        unsafe { ffi::DirectXTexFFI_IsValid(self) }
    }

    pub fn is_compressed(self) -> bool {
        unsafe { ffi::DirectXTexFFI_IsCompressed(self) }
    }

    pub fn is_packed(self) -> bool {
        unsafe { ffi::DirectXTexFFI_IsPacked(self) }
    }

    pub fn is_video(self) -> bool {
        unsafe { ffi::DirectXTexFFI_IsVideo(self) }
    }

    pub fn is_planar(self) -> bool {
        unsafe { ffi::DirectXTexFFI_IsPlanar(self) }
    }

    pub fn is_palettized(self) -> bool {
        unsafe { ffi::DirectXTexFFI_IsPalettized(self) }
    }

    pub fn is_depth_stencil(self) -> bool {
        unsafe { ffi::DirectXTexFFI_IsDepthStencil(self) }
    }

    pub fn is_srgb(self) -> bool {
        unsafe { ffi::DirectXTexFFI_IsSRGB(self) }
    }

    pub fn is_bgr(self) -> bool {
        unsafe { ffi::DirectXTexFFI_IsBGR(self) }
    }

    pub fn is_typeless(self, partial_typeless: bool) -> bool {
        unsafe { ffi::DirectXTexFFI_IsTypeless(self, partial_typeless) }
    }
}

#[cfg(test)]
mod tests {
    use super::DXGI_FORMAT;

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
}
