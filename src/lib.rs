#![warn(clippy::pedantic, clippy::std_instead_of_core)]
#![allow(clippy::missing_errors_doc, clippy::unreadable_literal)]

mod dxgi_format;
mod enums;
mod ffi;
mod hresult;
mod image;
mod macros;
mod scratch_image;
mod texture_metadata;

pub use self::{
    dxgi_format::{Pitch, DXGI_FORMAT},
    enums::{
        CP_FLAGS, DDS_FLAGS, FORMAT_TYPE, TEX_ALPHA_MODE, TEX_DIMENSION, TEX_MISC_FLAG,
        TEX_MISC_FLAG2, TGA_FLAGS, WIC_FLAGS,
    },
    hresult::{HResult, HResultError},
    image::Image,
    scratch_image::ScratchImage,
    texture_metadata::{DDSMetaData, TexMetadata},
};

macros::c_opaque!(pub IWICImagingFactory);
macros::c_opaque!(pub IWICMetadataQueryReader);
