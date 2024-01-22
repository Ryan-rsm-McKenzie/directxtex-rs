use crate::{
    ffi::{self, prelude::*},
    Blob, Image, Result, ScratchImage, TexMetadata, CNMAP_FLAGS, DDS_FLAGS, DXGI_FORMAT,
    TEX_COMPRESS_FLAGS, TEX_FILTER_FLAGS, TEX_PMALPHA_FLAGS,
};

pub fn save_dds(images: &[Image], metadata: &TexMetadata, flags: DDS_FLAGS) -> Result<Blob> {
    let mut result = Blob::default();
    let hr = unsafe {
        ffi::DirectXTexFFI_SaveToDDSMemory2(
            images.as_ffi_ptr(),
            images.len(),
            metadata.into(),
            flags,
            (&mut result).into(),
        )
    };
    hr.success(result)
}

/// Resize the image to width x height. Defaults to Fant filtering.
///
/// Note for a complex resize, the result will always have mipLevels == 1
pub fn resize(
    images: &[Image],
    metadata: &TexMetadata,
    width: usize,
    height: usize,
    filter: TEX_FILTER_FLAGS,
) -> Result<ScratchImage> {
    let mut result = ScratchImage::default();
    let hr = unsafe {
        ffi::DirectXTexFFI_Resize2(
            images.as_ffi_ptr(),
            images.len(),
            metadata.into(),
            width,
            height,
            filter,
            (&mut result).into(),
        )
    };
    hr.success(result)
}

/// Convert the image to a new format
pub fn convert(
    images: &[Image],
    metadata: &TexMetadata,
    format: DXGI_FORMAT,
    filter: TEX_FILTER_FLAGS,
    threshold: f32,
) -> Result<ScratchImage> {
    let mut result = ScratchImage::default();
    let hr = unsafe {
        ffi::DirectXTexFFI_Convert2(
            images.as_ffi_ptr(),
            images.len(),
            metadata.into(),
            format,
            filter,
            threshold,
            (&mut result).into(),
        )
    };
    hr.success(result)
}

/// Converts the image from a planar format to an equivalent non-planar format
pub fn convert_to_single_plane(images: &[Image], metadata: &TexMetadata) -> Result<ScratchImage> {
    let mut result = ScratchImage::default();
    let hr = unsafe {
        ffi::DirectXTexFFI_ConvertToSinglePlane2(
            images.as_ffi_ptr(),
            images.len(),
            metadata.into(),
            (&mut result).into(),
        )
    };
    hr.success(result)
}

/// levels of '0' indicates a full mipchain, otherwise is generates that number of total levels (including the source base image)
///
/// Defaults to Fant filtering which is equivalent to a box filter
pub fn generate_mip_maps(
    images: &[Image],
    metadata: &TexMetadata,
    filter: TEX_FILTER_FLAGS,
    levels: usize,
) -> Result<ScratchImage> {
    let mut result = ScratchImage::default();
    let hr = unsafe {
        ffi::DirectXTexFFI_GenerateMipMaps2(
            images.as_ffi_ptr(),
            images.len(),
            metadata.into(),
            filter,
            levels,
            (&mut result).into(),
        )
    };
    hr.success(result)
}

/// levels of '0' indicates a full mipchain, otherwise is generates that number of total levels (including the source base image)
///
/// Defaults to Fant filtering which is equivalent to a box filter
pub fn generate_mip_maps_3d(
    images: &[Image],
    metadata: Option<&TexMetadata>,
    filter: TEX_FILTER_FLAGS,
    levels: usize,
) -> Result<ScratchImage> {
    let mut result = ScratchImage::default();
    let hr = if let Some(metadata) = metadata {
        unsafe {
            ffi::DirectXTexFFI_GenerateMipMaps3D2(
                images.as_ffi_ptr(),
                images.len(),
                metadata.into(),
                filter,
                levels,
                (&mut result).into(),
            )
        }
    } else {
        unsafe {
            ffi::DirectXTexFFI_GenerateMipMaps3D1(
                images.as_ffi_ptr(),
                images.len(),
                filter,
                levels,
                (&mut result).into(),
            )
        }
    };
    hr.success(result)
}

pub fn scale_mip_maps_alpha_for_coverage(
    images: &[Image],
    metadata: &TexMetadata,
    item: usize,
    alpha_reference: f32,
) -> Result<ScratchImage> {
    let mut result = ScratchImage::default();
    let hr = unsafe {
        ffi::DirectXTexFFI_ScaleMipMapsAlphaForCoverage(
            images.as_ffi_ptr(),
            images.len(),
            metadata.into(),
            item,
            alpha_reference,
            (&mut result).into(),
        )
    };
    hr.success(result)
}

/// Converts to/from a premultiplied alpha version of the texture
pub fn premultiply_alpha(
    images: &[Image],
    metadata: &TexMetadata,
    flags: TEX_PMALPHA_FLAGS,
) -> Result<ScratchImage> {
    let mut result = ScratchImage::default();
    let hr = unsafe {
        ffi::DirectXTexFFI_PremultiplyAlpha2(
            images.as_ffi_ptr(),
            images.len(),
            metadata.into(),
            flags,
            (&mut result).into(),
        )
    };
    hr.success(result)
}

/// Note that threshold is only used by BC1. TEX_THRESHOLD_DEFAULT is a typical value to use
pub fn compress(
    images: &[Image],
    metadata: &TexMetadata,
    format: DXGI_FORMAT,
    compress: TEX_COMPRESS_FLAGS,
    threshold: f32,
) -> Result<ScratchImage> {
    let mut result = ScratchImage::default();
    let hr = unsafe {
        ffi::DirectXTexFFI_Compress2(
            images.as_ffi_ptr(),
            images.len(),
            metadata.into(),
            format,
            compress,
            threshold,
            (&mut result).into(),
        )
    };
    hr.success(result)
}

pub fn decompress(
    images: &[Image],
    metadata: &TexMetadata,
    format: DXGI_FORMAT,
) -> Result<ScratchImage> {
    let mut result = ScratchImage::default();
    let hr = unsafe {
        ffi::DirectXTexFFI_Decompress2(
            images.as_ffi_ptr(),
            images.len(),
            metadata.into(),
            format,
            (&mut result).into(),
        )
    };
    hr.success(result)
}

pub fn compute_normal_map(
    images: &[Image],
    metadata: &TexMetadata,
    flags: CNMAP_FLAGS,
    amplitude: f32,
    format: DXGI_FORMAT,
) -> Result<ScratchImage> {
    let mut result = ScratchImage::default();
    let hr = unsafe {
        ffi::DirectXTexFFI_ComputeNormalMap2(
            images.as_ffi_ptr(),
            images.len(),
            metadata.into(),
            flags,
            amplitude,
            format,
            (&mut result).into(),
        )
    };
    hr.success(result)
}
