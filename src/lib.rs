#![warn(clippy::pedantic, clippy::std_instead_of_core)]
#![allow(clippy::missing_errors_doc)]

pub mod dxgi_format;
mod ffi;
pub mod hresult;
mod macros;

pub use self::{dxgi_format::DXGI_FORMAT, hresult::HRESULT, macros::InvalidEnumRepresentation};
use core::ffi::{c_int, c_ulong};

macros::c_enum! {
    FORMAT_TYPE(c_int) => {
        FORMAT_TYPE_TYPELESS = 0,
        FORMAT_TYPE_FLOAT = 1,
        FORMAT_TYPE_UNORM = 2,
        FORMAT_TYPE_SNORM = 3,
        FORMAT_TYPE_UINT = 4,
        FORMAT_TYPE_SINT = 5,
    }
}

bitflags::bitflags! {
    #[allow(non_camel_case_types)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    pub struct CP_FLAGS: c_ulong {
        /// Assume pitch is DWORD aligned instead of BYTE aligned
        const CP_FLAGS_LEGACY_DWORD = 0x1;

        /// Assume pitch is 16-byte aligned instead of BYTE aligned
        const CP_FLAGS_PARAGRAPH = 0x2;

        /// Assume pitch is 32-byte aligned instead of BYTE aligned
        const CP_FLAGS_YMM = 0x4;

        /// Assume pitch is 64-byte aligned instead of BYTE aligned
        const CP_FLAGS_ZMM = 0x8;

        /// Assume pitch is 4096-byte aligned instead of BYTE aligned
        const CP_FLAGS_PAGE4K = 0x200;

        /// BC formats with malformed mipchain blocks smaller than 4x4
        const CP_FLAGS_BAD_DXTN_TAILS = 0x1000;

        /// Override with a legacy 24 bits-per-pixel format size
        const CP_FLAGS_24BPP = 0x10000;

        /// Override with a legacy 16 bits-per-pixel format size
        const CP_FLAGS_16BPP = 0x20000;

        /// Override with a legacy 8 bits-per-pixel format size
        const CP_FLAGS_8BPP = 0x40000;

        const _ = !0;
    }
}

impl CP_FLAGS {
    /// Normal operation
    pub const CP_FLAGS_NONE: Self = Self::empty();
}

impl Default for CP_FLAGS {
    fn default() -> Self {
        Self::CP_FLAGS_NONE
    }
}
