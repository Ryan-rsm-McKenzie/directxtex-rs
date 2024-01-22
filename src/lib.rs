#![warn(clippy::pedantic, clippy::std_instead_of_core)]
#![allow(clippy::missing_errors_doc, clippy::unreadable_literal)]

mod blob;
mod dds_metadata;
mod dxgi_format;
mod enums;
mod ffi;
mod hresult;
mod image;
mod macros;
mod scratch_image;
mod texture_metadata;

pub use self::{
    blob::Blob,
    dds_metadata::DDSMetaData,
    dxgi_format::{Pitch, DXGI_FORMAT},
    enums::{
        WICCodecs, CMSE_FLAGS, CNMAP_FLAGS, CP_FLAGS, CREATETEX_FLAGS, DDS_FLAGS, FORMAT_TYPE,
        TEX_ALPHA_MODE, TEX_COMPRESS_FLAGS, TEX_DIMENSION, TEX_FILTER_FLAGS, TEX_FR_FLAGS,
        TEX_MISC_FLAG, TEX_MISC_FLAG2, TEX_PMALPHA_FLAGS, TGA_FLAGS, WIC_FLAGS,
    },
    hresult::{HResult, HResultError},
    image::Image,
    scratch_image::ScratchImage,
    texture_metadata::TexMetadata,
};

pub const TEX_FILTER_DITHER_MASK: u32 = 0xF0000;
pub const TEX_FILTER_MODE_MASK: u32 = 0xF00000;
pub const TEX_FILTER_SRGB_MASK: u32 = 0xF000000;

type Result<T> = core::result::Result<T, HResultError>;
