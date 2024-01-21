#![warn(clippy::pedantic, clippy::std_instead_of_core)]
#![allow(clippy::missing_errors_doc, clippy::unreadable_literal)]

mod dxgi_format;
mod ffi;
mod hresult;
mod macros;
mod texture_metadata;

pub use self::{
    dxgi_format::{Pitch, CP_FLAGS, DXGI_FORMAT, FORMAT_TYPE},
    hresult::{HResult, HResultError},
    texture_metadata::{
        DDSMetaData, TexMetadata, DDS_FLAGS, TEX_ALPHA_MODE, TEX_DIMENSION, TEX_MISC_FLAG,
        TEX_MISC_FLAG2, TGA_FLAGS, WIC_FLAGS,
    },
};

macros::c_opaque!(IWICImagingFactory);
macros::c_opaque!(IWICMetadataQueryReader);
