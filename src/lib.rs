//! # Migration Table
//!
//! |C++|Rust|
//! |---|---|
//! |[`IsValid`](https://github.com/microsoft/DirectXTex/wiki/FormatTests)|[`DXGI_FORMAT::is_valid`]|
//! |[`IsCompressed`](https://github.com/microsoft/DirectXTex/wiki/FormatTests)|[`DXGI_FORMAT::is_compressed`]|
//! |[`IsPacked`](https://github.com/microsoft/DirectXTex/wiki/FormatTests)|[`DXGI_FORMAT::is_packed`]|
//! |[`IsVideo`](https://github.com/microsoft/DirectXTex/wiki/FormatTests)|[`DXGI_FORMAT::is_video`]|
//! |[`IsPlanar`](https://github.com/microsoft/DirectXTex/wiki/FormatTests)|[`DXGI_FORMAT::is_planar`]|
//! |[`IsPalettized`](https://github.com/microsoft/DirectXTex/wiki/FormatTests)|[`DXGI_FORMAT::is_palettized`]|
//! |[`IsDepthStencil`](https://github.com/microsoft/DirectXTex/wiki/FormatTests)|[`DXGI_FORMAT::is_depth_stencil`]|
//! |[`IsSRGB`](https://github.com/microsoft/DirectXTex/wiki/FormatTests)|[`DXGI_FORMAT::is_srgb`]|
//! |[`IsBGR`](https://github.com/microsoft/DirectXTex/wiki/FormatTests)|[`DXGI_FORMAT::is_bgr`]|
//! |[`IsTypeless`](https://github.com/microsoft/DirectXTex/wiki/FormatTests)|[`DXGI_FORMAT::is_typeless`]|
//! |[`HasAlpha`](https://github.com/microsoft/DirectXTex/wiki/FormatProperties)|[`DXGI_FORMAT::has_alpha`]|
//! |[`BitsPerPixel`](https://github.com/microsoft/DirectXTex/wiki/FormatProperties)|[`DXGI_FORMAT::bits_per_pixel`]|
//! |[`BitsPerColor`](https://github.com/microsoft/DirectXTex/wiki/FormatProperties)|[`DXGI_FORMAT::bits_per_color`]|
//! |[`FORMAT_TYPE`](https://github.com/microsoft/DirectXTex/wiki/FormatProperties#return-value)|[`FORMAT_TYPE`]|
//! |[`FormatDataType`](https://github.com/microsoft/DirectXTex/wiki/FormatProperties)|[`DXGI_FORMAT::format_data_type`]|
//! |[`CP_FLAGS`](https://github.com/microsoft/DirectXTex/wiki/ComputePitch#parameters)|[`CP_FLAGS`]|
//! |[`ComputePitch`](https://github.com/microsoft/DirectXTex/wiki/ComputePitch)|[`DXGI_FORMAT::compute_pitch`]|
//! |[`ComputeScanlines`](https://github.com/microsoft/DirectXTex/wiki/ComputeScanlines)|[`DXGI_FORMAT::compute_scanlines`]|
//! |[`MakeSRGB`](https://github.com/microsoft/DirectXTex/wiki/FormatPromoters)|[`DXGI_FORMAT::make_srgb`]|
//! |[`MakeLinear`](https://github.com/microsoft/DirectXTex/wiki/FormatPromoters)|[`DXGI_FORMAT::make_linear`]|
//! |[`MakeTypeless`](https://github.com/microsoft/DirectXTex/wiki/FormatPromoters)|[`DXGI_FORMAT::make_typeless`]|
//! |[`MakeTypelessUNORM`](https://github.com/microsoft/DirectXTex/wiki/FormatPromoters)|[`DXGI_FORMAT::make_typeless_unorm`]|
//! |[`MakeTypelessFLOAT`](https://github.com/microsoft/DirectXTex/wiki/FormatPromoters)|[`DXGI_FORMAT::make_typeless_float`]|
//! |||
//! |`TEX_DIMENSION`|[`TEX_DIMENSION`]|
//! |`TEX_MISC_FLAG`|[`TEX_MISC_FLAG`]|
//! |`TEX_MISC_FLAG2`|[`TEX_MISC_FLAG2`]|
//! |`TEX_ALPHA_MODE`|[`TEX_ALPHA_MODE`]|
//! |[`TexMetadata`](https://github.com/microsoft/DirectXTex/wiki/TexMetadata)|[`TexMetadata`]|
//! |[`TexMetadata::ComputeIndex`](https://github.com/microsoft/DirectXTex/wiki/TexMetadata#methods)|[`TexMetadata::compute_index`]|
//! |[`TexMetadata::IsCubemap`](https://github.com/microsoft/DirectXTex/wiki/TexMetadata#methods)|[`TexMetadata::is_cubemap`]|
//! |[`TexMetadata::IsPMAlpha`](https://github.com/microsoft/DirectXTex/wiki/TexMetadata#methods)|[`TexMetadata::is_pm_alpha`]|
//! |[`TexMetadata::SetAlphaMode`](https://github.com/microsoft/DirectXTex/wiki/TexMetadata#methods)|[`TexMetadata::set_alpha_mode`]|
//! |[`TexMetadata::GetAlphaMode`](https://github.com/microsoft/DirectXTex/wiki/TexMetadata#methods)|[`TexMetadata::get_alpha_mode`]|
//! |[`TexMetadata::IsVolumemap`](https://github.com/microsoft/DirectXTex/wiki/TexMetadata#methods)|[`TexMetadata::is_volumemap`]|
//! |`DDSMetaData`|[`DDSMetaData`]|
//! |`DDSMetaData::IsDX10`|[`DDSMetaData::is_dx10`]|
//! |[`DDS_FLAGS`](https://github.com/microsoft/DirectXTex/wiki/DDS-I-O-Functions#related-flags)|[`DDS_FLAGS`]|
//! |[`TGA_FLAGS`](https://github.com/microsoft/DirectXTex/wiki/TGA-I-O-Functions#related-flags)|[`TGA_FLAGS`]|
//! |[`GetMetadataFromDDSMemoryEx`](https://github.com/microsoft/DirectXTex/wiki/DDS-I-O-Functions#getmetadatafromddsmemory-getmetadatafromddsfile)|[`TexMetadata::from_dds`]|
//! |[`GetMetadataFromHDRMemory`](https://github.com/microsoft/DirectXTex/wiki/HDR-I-O-Functions#getmetadatafromhdrmemory-getmetadatafromhdrfile)|[`TexMetadata::from_hdr`]|
//! |[`GetMetadataFromTGAMemory`](https://github.com/microsoft/DirectXTex/wiki/TGA-I-O-Functions#getmetadatafromtgamemory-getmetadatafromtgafile)|[`TexMetadata::from_tga`]|
//! |||
//! |[`Image`](https://github.com/microsoft/DirectXTex/wiki/Image)|[`Image`]|
//! |[`ScratchImage`](https://github.com/microsoft/DirectXTex/wiki/Image#scratchimage)|[`ScratchImage`]|
//! |[`ScratchImage::Initialize`](https://github.com/microsoft/DirectXTex/wiki/Image#methods)|[`ScratchImage::initialize`]|
//! |[`ScratchImage::Initialize1D`](https://github.com/microsoft/DirectXTex/wiki/Image#methods)|[`ScratchImage::initialize_1d`]|
//! |[`ScratchImage::Initialize2D`](https://github.com/microsoft/DirectXTex/wiki/Image#methods)|[`ScratchImage::initialize_2d`]|
//! |[`ScratchImage::Initialize3D`](https://github.com/microsoft/DirectXTex/wiki/Image#methods)|[`ScratchImage::initialize_3d`]|
//! |[`ScratchImage::InitializeCube`](https://github.com/microsoft/DirectXTex/wiki/Image#methods)|[`ScratchImage::initialize_cube`]|
//! |[`ScratchImage::InitializeFromImage`](https://github.com/microsoft/DirectXTex/wiki/Image#methods)|[`ScratchImage::initialize_from_image`]|
//! |[`ScratchImage::InitializeArrayFromImages`](https://github.com/microsoft/DirectXTex/wiki/Image#methods)|[`ScratchImage::initialize_array_from_images`]|
//! |[`ScratchImage::InitializeCubeFromImages`](https://github.com/microsoft/DirectXTex/wiki/Image#methods)|[`ScratchImage::initialize_cube_from_images`]|
//! |[`ScratchImage::Initialize3DFromImages`](https://github.com/microsoft/DirectXTex/wiki/Image#methods)|[`ScratchImage::initialize_3d_from_images`]|
//! |[`ScratchImage::Release`](https://github.com/microsoft/DirectXTex/wiki/Image#methods)|[`ScratchImage::release`]|
//! |[`ScratchImage::OverrideFormat`](https://github.com/microsoft/DirectXTex/wiki/Image#methods)|[`ScratchImage::override_format`]|
//! |[`ScratchImage::GetMetadata`](https://github.com/microsoft/DirectXTex/wiki/Image#methods)|[`ScratchImage::metadata`]|
//! |[`ScratchImage::GetImage`](https://github.com/microsoft/DirectXTex/wiki/Image#methods)|[`ScratchImage::image`]|
//! |[`ScratchImage::GetImages`](https://github.com/microsoft/DirectXTex/wiki/Image#methods)<br>[`ScratchImage::GetImageCount`](https://github.com/microsoft/DirectXTex/wiki/Image#methods)|[`ScratchImage::images`]|
//! |[`ScratchImage::GetPixels`](https://github.com/microsoft/DirectXTex/wiki/Image#methods)<br>[`ScratchImage::GetPixelsSize`](https://github.com/microsoft/DirectXTex/wiki/Image#methods)|[`ScratchImage::pixels`]<br>[`ScratchImage::pixels_mut`]|
//! |[`ScratchImage::IsAlphaAllOpaque`](https://github.com/microsoft/DirectXTex/wiki/Image#methods)|[`ScratchImage::is_alpha_all_opaque`]|
//! |||
//! |[`Blob`](https://github.com/microsoft/DirectXTex/wiki/Blob)|[`Blob`]|
//! |[`Blob::Initialize`](https://github.com/microsoft/DirectXTex/wiki/Blob#methods)|[`Blob::initialize`]|
//! |[`Blob::Release`](https://github.com/microsoft/DirectXTex/wiki/Blob#methods)|[`Blob::release`]|
//! |[`Blob::GetBufferPointer`](https://github.com/microsoft/DirectXTex/wiki/Blob#methods)<br>[`Blob::GetBufferSize`](https://github.com/microsoft/DirectXTex/wiki/Blob#methods)|[`Blob::buffer`]<br>[`Blob::buffer_mut`]|
//! |[`Blob::Resize`](https://github.com/microsoft/DirectXTex/wiki/Blob#methods)|[`Blob::resize`]|
//! |[`Blob::Trim`](https://github.com/microsoft/DirectXTex/wiki/Blob#methods)|[`Blob::trim`]|
//! |||
//! |[`LoadFromDDSMemoryEx`](https://github.com/microsoft/DirectXTex/wiki/DDS-I-O-Functions#loadfromddsmemory-loadfromddsfile)|[`ScratchImage::load_dds`]|
//! |[`SaveToDDSMemory`](https://github.com/microsoft/DirectXTex/wiki/DDS-I-O-Functions#savetoddsmemory-savetoddsfile)|[`save_dds`]<br>[`ScratchImage::save_dds`]|
//! |[`LoadFromHDRMemory`](https://github.com/microsoft/DirectXTex/wiki/HDR-I-O-Functions#loadfromhdrmemory-loadfromhdrfile)|[`ScratchImage::load_hdr`]|
//! |[`SaveToHDRMemory`](https://github.com/microsoft/DirectXTex/wiki/HDR-I-O-Functions#savetohdrmemory-savetohdrfile)|[`Image::save_hdr`]|
//! |[`LoadFromTGAMemory`](https://github.com/microsoft/DirectXTex/wiki/TGA-I-O-Functions#loadfromtgamemory-loadfromtgafile)|[`ScratchImage::load_tga`]|
//! |[`SaveToTGAMemory`](https://github.com/microsoft/DirectXTex/wiki/TGA-I-O-Functions#savetotgamemory-savetotgafile)|[`Image::save_tga`]|
//! |||
//! |`TEX_FILTER_FLAGS`|[`TEX_FILTER_FLAGS`]|
//! |`TEX_FILTER_DITHER_MASK`|[`TEX_FILTER_DITHER_MASK`]|
//! |`TEX_FILTER_MODE_MASK`|[`TEX_FILTER_MODE_MASK`]|
//! |`TEX_FILTER_SRGB_MASK`|[`TEX_FILTER_SRGB_MASK`]|
//! |[`Resize`](https://github.com/microsoft/DirectXTex/wiki/Resize)|[`resize`]<br>[`ScratchImage::resize`]<br>[`Image::resize`]|
//! |`TEX_THRESHOLD_DEFAULT`|[`TEX_THRESHOLD_DEFAULT`]|
//! |[`Convert`](https://github.com/microsoft/DirectXTex/wiki/Convert)|[`convert`]<br>[`ScratchImage::convert`]<br>[`Image::convert`]|
//! |[`ConvertToSinglePlane`](https://github.com/microsoft/DirectXTex/wiki/ConvertToSinglePlane)|[`convert_to_single_plane`]<br>[`ScratchImage::convert_to_single_plane`]<br>[`Image::convert_to_single_plane`]|
//! |[`GenerateMipMaps`](https://github.com/microsoft/DirectXTex/wiki/GenerateMipMaps)|[`generate_mip_maps`]<br>[`ScratchImage::generate_mip_maps`]<br>[`Image::generate_mip_maps`]|
//! |[`GenerateMipMaps3D`](https://github.com/microsoft/DirectXTex/wiki/GenerateMipMaps3D)|[`generate_mip_maps_3d`]<br>[`ScratchImage::generate_mip_maps_3d`]|
//! |[`ScaleMipMapsAlphaForCoverage`](https://github.com/microsoft/DirectXTex/wiki/ScaleMipMapsAlphaForCoverage)|[`scale_mip_maps_alpha_for_coverage`]<br>[`ScratchImage::scale_mip_maps_alpha_for_coverage`]|
//! |[`TEX_PMALPHA_FLAGS`](https://github.com/microsoft/DirectXTex/wiki/PremultiplyAlpha#parameters)|[`TEX_PMALPHA_FLAGS`]|
//! |[`PremultiplyAlpha`](https://github.com/microsoft/DirectXTex/wiki/PremultiplyAlpha)|[`premultiply_alpha`]<br>[`ScratchImage::premultiply_alpha`]<br>[`Image::premultiply_alpha`]|
//! |[`TEX_COMPRESS_FLAGS`](https://github.com/microsoft/DirectXTex/wiki/Compress#related-flags)|[`TEX_COMPRESS_FLAGS`]|
//! |`TEX_ALPHA_WEIGHT_DEFAULT`|[`TEX_ALPHA_WEIGHT_DEFAULT`]|
//! |[`Compress`](https://github.com/microsoft/DirectXTex/wiki/Compress)|[`compress`]<br>[`ScratchImage::compress`]<br>[`Image::compress`]|
//! |[`Decompress`](https://github.com/microsoft/DirectXTex/wiki/Decompress)|[`decompress`]<br>[`ScratchImage::decompress`]<br>[`Image::decompress`]|
//! |||
//! |[`CNMAP_FLAGS`](https://github.com/microsoft/DirectXTex/wiki/ComputeNormalMap#parameters)|[`CNMAP_FLAGS`]|
//! |[`ComputeNormalMap`](https://github.com/microsoft/DirectXTex/wiki/ComputeNormalMap)|[`compute_normal_map`]<br>[`ScratchImage::compute_normal_map`]<br>[`Image::compute_normal_map`]|
//! |||
//! |[`Rect`](https://github.com/microsoft/DirectXTex/wiki/CopyRectangle#parameters)|[`Rect`]|
//! |[`CopyRectangle`](https://github.com/microsoft/DirectXTex/wiki/CopyRectangle)|[`Image::copy_rectangle`]|
//! |[`CMSE_FLAGS`](https://github.com/microsoft/DirectXTex/wiki/ComputeMSE#parameters)|[`CMSE_FLAGS`]|
//! |[`ComputeMSE`](https://github.com/microsoft/DirectXTex/wiki/ComputeMSE)|[`Image::compute_mse`]|
//! |||
//! |[`EncodeDDSHeader`](https://github.com/microsoft/DirectXTex/wiki/DDS-I-O-Functions#encodeddsheader)|[`TexMetadata::encode_dds_header`]|

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
    dxgi_format::{dxgi_format::*, Pitch, DXGI_FORMAT},
    enums::{
        cmse_flags::*, cnmap_flags::*, cp_flags::*, dds_flags::*, format_type::*,
        tex_alpha_mode::*, tex_compress_flags::*, tex_dimension::*, tex_filter_flags::*,
        tex_misc_flag::*, tex_misc_flag2::*, tex_pmalpha_flags::*, tga_flags::*, CMSE_FLAGS,
        CNMAP_FLAGS, CP_FLAGS, DDS_FLAGS, FORMAT_TYPE, TEX_ALPHA_MODE, TEX_COMPRESS_FLAGS,
        TEX_DIMENSION, TEX_FILTER_FLAGS, TEX_MISC_FLAG, TEX_MISC_FLAG2, TEX_PMALPHA_FLAGS,
        TGA_FLAGS,
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
