//! # Migration Table
//!
//! C++ | Rust
//! --- | ---
//! `IsValid` | [`DXGI_FORMAT::is_valid`]
//! `IsCompressed` | [`DXGI_FORMAT::is_compressed`]
//! `IsPacked` | [`DXGI_FORMAT::is_packed`]
//! `IsVideo` | [`DXGI_FORMAT::is_video`]
//! `IsPlanar` | [`DXGI_FORMAT::is_planar`]
//! `IsPalettized` | [`DXGI_FORMAT::is_palettized`]
//! `IsDepthStencil` | [`DXGI_FORMAT::is_depth_stencil`]
//! `IsSRGB` | [`DXGI_FORMAT::is_srgb`]
//! `IsBGR` | [`DXGI_FORMAT::is_bgr`]
//! `IsTypeless` | [`DXGI_FORMAT::is_typeless`]
//! `HasAlpha` | [`DXGI_FORMAT::has_alpha`]
//! `BitsPerPixel` | [`DXGI_FORMAT::bits_per_pixel`]
//! `BitsPerColor` | [`DXGI_FORMAT::bits_per_color`]
//! `FORMAT_TYPE` | [`FORMAT_TYPE`]
//! `FormatDataType` | [`DXGI_FORMAT::format_data_type`]
//! `CP_FLAGS` | [`CP_FLAGS`]
//! `ComputePitch` | [`DXGI_FORMAT::compute_pitch`]
//! `ComputeScanlines` | [`DXGI_FORMAT::compute_scanlines`]
//! `MakeSRGB` | [`DXGI_FORMAT::make_srgb`]
//! `MakeLinear` | [`DXGI_FORMAT::make_linear`]
//! `MakeTypeless` | [`DXGI_FORMAT::make_typeless`]
//! `MakeTypelessUNORM` | [`DXGI_FORMAT::make_typeless_unorm`]
//! `MakeTypelessFLOAT` | [`DXGI_FORMAT::make_typeless_float`]
//! |
//! `TEX_DIMENSION` | [`TEX_DIMENSION`]
//! `TEX_MISC_FLAG` | [`TEX_MISC_FLAG`]
//! `TEX_MISC_FLAG2` | [`TEX_MISC_FLAG2`]
//! `TEX_ALPHA_MODE` | [`TEX_ALPHA_MODE`]
//! `TexMetadata` | [`TexMetadata`]
//! `TexMetadata::ComputeIndex` | [`TexMetadata::compute_index`]
//! `TexMetadata::IsCubemap` | [`TexMetadata::is_cubemap`]
//! `TexMetadata::IsPMAlpha` | [`TexMetadata::is_pm_alpha`]
//! `TexMetadata::SetAlphaMode` | [`TexMetadata::set_alpha_mode`]
//! `TexMetadata::GetAlphaMode` | [`TexMetadata::get_alpha_mode`]
//! `TexMetadata::IsVolumemap` | [`TexMetadata::is_volumemap`]
//! `DDSMetaData` | [`DDSMetaData`]
//! `DDSMetaData::IsDX10` | [`DDSMetaData::is_dx10`]
//! `DDS_FLAGS` | [`DDS_FLAGS`]
//! `TGA_FLAGS` | [`TGA_FLAGS`]
//! `GetMetadataFromDDSMemoryEx` | [`TexMetadata::from_dds`]
//! `GetMetadataFromHDRMemory` | [`TexMetadata::from_hdr`]
//! `GetMetadataFromTGAMemory` | [`TexMetadata::from_tga`]
//! |
//! `Image` | [`Image`]
//! `ScratchImage` | [`ScratchImage`]
//! `ScratchImage::Initialize` | [`ScratchImage::initialize`]
//! `ScratchImage::Initialize1D` | [`ScratchImage::initialize_1d`]
//! `ScratchImage::Initialize2D` | [`ScratchImage::initialize_2d`]
//! `ScratchImage::Initialize3D` | [`ScratchImage::initialize_3d`]
//! `ScratchImage::InitializeCube` | [`ScratchImage::initialize_cube`]
//! `ScratchImage::InitializeFromImage` | [`ScratchImage::initialize_from_image`]
//! `ScratchImage::InitializeArrayFromImages` | [`ScratchImage::initialize_array_from_images`]
//! `ScratchImage::InitializeCubeFromImages` | [`ScratchImage::initialize_cube_from_images`]
//! `ScratchImage::Initialize3DFromImages` | [`ScratchImage::initialize_3d_from_images`]
//! `ScratchImage::Release` | [`ScratchImage::release`]
//! `ScratchImage::OverrideFormat` | [`ScratchImage::override_format`]
//! `ScratchImage::GetMetadata` | [`ScratchImage::metadata`]
//! `ScratchImage::GetImage` | [`ScratchImage::image`]
//! `ScratchImage::GetImages` <br> `ScratchImage::GetImageCount` | [`ScratchImage::images`]
//! `ScratchImage::GetPixels` <br> `ScratchImage::GetPixelsSize` | [`ScratchImage::pixels`] <br> [`ScratchImage::pixels_mut`]
//! `ScratchImage::IsAlphaAllOpaque` | [`ScratchImage::is_alpha_all_opaque`]
//! |
//! `Blob` | [`Blob`]
//! `Blob::Initialize` | [`Blob::initialize`]
//! `Blob::Release` | [`Blob::release`]
//! `Blob::GetBufferPointer` <br> `Blob::GetBufferSize` | [`Blob::buffer`] <br> [`Blob::buffer_mut`]
//! `Blob::Resize` | [`Blob::resize`]
//! `Blob::Trim` | [`Blob::trim`]
//! |
//! `LoadFromDDSMemoryEx` | [`ScratchImage::load_dds`]
//! `SaveToDDSMemory` | [`save_dds`] <br> [`ScratchImage::save_dds`]
//! `LoadFromHDRMemory` | [`ScratchImage::load_hdr`]
//! `SaveToHDRMemory` | [`Image::save_hdr`]
//! `LoadFromTGAMemory` | [`ScratchImage::load_tga`]
//! `SaveToTGAMemory` | [`Image::save_tga`]
//! |
//! `TEX_FILTER_FLAGS` | [`TEX_FILTER_FLAGS`]
//! `TEX_FILTER_DITHER_MASK` | [`TEX_FILTER_DITHER_MASK`]
//! `TEX_FILTER_MODE_MASK` | [`TEX_FILTER_MODE_MASK`]
//! `TEX_FILTER_SRGB_MASK` | [`TEX_FILTER_SRGB_MASK`]
//! `Resize` | [`resize`] <br> [`ScratchImage::resize`] <br> [`Image::resize`]
//! `TEX_THRESHOLD_DEFAULT` | [`TEX_THRESHOLD_DEFAULT`]
//! `Convert` | [`convert`] <br> [`ScratchImage::convert`] <br> [`Image::convert`]
//! `ConvertToSinglePlane` | [`convert_to_single_plane`] <br> [`ScratchImage::convert_to_single_plane`] <br> [`Image::convert_to_single_plane`]
//! `GenerateMipMaps` | [`generate_mip_maps`] <br> [`ScratchImage::generate_mip_maps`] <br> [`Image::generate_mip_maps`]
//! `GenerateMipMaps3D` | [`generate_mip_maps_3d`] <br> [`ScratchImage::generate_mip_maps_3d`]
//! `ScaleMipMapsAlphaForCoverage` | [`scale_mip_maps_alpha_for_coverage`] <br> [`ScratchImage::scale_mip_maps_alpha_for_coverage`]
//! `TEX_PMALPHA_FLAGS` | [`TEX_PMALPHA_FLAGS`]
//! `PremultiplyAlpha` | [`premultiply_alpha`] <br> [`ScratchImage::premultiply_alpha`] <br> [`Image::premultiply_alpha`]
//! `TEX_COMPRESS_FLAGS` | [`TEX_COMPRESS_FLAGS`]
//! `TEX_ALPHA_WEIGHT_DEFAULT` | [`TEX_ALPHA_WEIGHT_DEFAULT`]
//! `Compress` | [`compress`] <br> [`ScratchImage::compress`] <br> [`Image::compress`]
//! `Decompress` | [`decompress`] <br> [`ScratchImage::decompress`] <br> [`Image::decompress`]
//! |
//! `CNMAP_FLAGS` | [`CNMAP_FLAGS`]
//! `ComputeNormalMap` | [`compute_normal_map`] <br> [`ScratchImage::compute_normal_map`] <br> [`Image::compute_normal_map`]
//! |
//! `Rect` | [`Rect`]
//! `CopyRectangle` | [`Image::copy_rectangle`]
//! `CMSE_FLAGS` | [`CMSE_FLAGS`]
//! `ComputeMSE` | [`Image::compute_mse`]
//! |
//! `EncodeDDSHeader` | [`TexMetadata::encode_dds_header`]

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
mod free_functions;
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
    free_functions::{
        compress, compute_normal_map, convert, convert_to_single_plane, decompress,
        generate_mip_maps, generate_mip_maps_3d, premultiply_alpha, resize, save_dds,
        scale_mip_maps_alpha_for_coverage,
    },
    hresult::HResultError,
    image::{Image, MeanSquaredError},
    rect::Rect,
    scratch_image::ScratchImage,
    texture_metadata::TexMetadata,
};

use hresult::HResult;

type Result<T> = core::result::Result<T, HResultError>;
