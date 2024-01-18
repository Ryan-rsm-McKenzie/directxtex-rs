#![warn(clippy::pedantic)]

mod dxgi_format;
mod ffi;
mod macros;

pub use self::{dxgi_format::DXGI_FORMAT, macros::InvalidEnumRepresentation};

use std::ffi::c_int;

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
