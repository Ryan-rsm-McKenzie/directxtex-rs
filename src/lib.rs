#![warn(clippy::pedantic, clippy::std_instead_of_core)]
#![allow(
    clippy::missing_errors_doc,
    clippy::unreadable_literal,
    clippy::doc_markdown
)]

mod blob;
mod constants;
mod dds_metadata;
mod dxgi_format;
mod enums;
mod ffi;
mod hresult;
mod image;
mod macros;
mod rect;
mod scratch_image;
mod texture_metadata;

pub use self::{
    blob::Blob,
    constants::{
        TEX_ALPHA_WEIGHT_DEFAULT, TEX_FILTER_DITHER_MASK, TEX_FILTER_MODE_MASK,
        TEX_FILTER_SRGB_MASK, TEX_THRESHOLD_DEFAULT,
    },
    dds_metadata::DDSMetaData,
    dxgi_format::{Pitch, DXGI_FORMAT},
    enums::{
        CMSE_FLAGS, CNMAP_FLAGS, CP_FLAGS, DDS_FLAGS, FORMAT_TYPE, TEX_ALPHA_MODE,
        TEX_COMPRESS_FLAGS, TEX_DIMENSION, TEX_FILTER_FLAGS, TEX_MISC_FLAG, TEX_MISC_FLAG2,
        TEX_PMALPHA_FLAGS, TGA_FLAGS,
    },
    hresult::HResultError,
    image::{Image, MeanSquaredError},
    rect::Rect,
    scratch_image::ScratchImage,
    texture_metadata::TexMetadata,
};

use hresult::HResult;

type Result<T> = core::result::Result<T, HResultError>;
