use crate::{
    ffi::{self, prelude::*},
    Blob, Result, ScratchImage, TexMetadata, DDS_FLAGS, DXGI_FORMAT, TEX_COMPRESS_FLAGS,
    TEX_FILTER_FLAGS, TEX_PMALPHA_FLAGS, TGA_FLAGS,
};
use core::ptr;

#[derive(Debug)]
#[repr(C)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub format: DXGI_FORMAT,
    pub row_pitch: usize,
    pub slice_pitch: usize,
    pub pixels: *mut u8,
}

impl Image {
    pub fn save_dds(&self, flags: DDS_FLAGS) -> Result<Blob> {
        let mut result = Blob::default();
        let hr = unsafe {
            ffi::DirectXTexFFI_SaveToDDSMemory1(self.into(), flags, (&mut result).into())
        };
        hr.success(result)
    }

    pub fn save_hdr(&self) -> Result<Blob> {
        let mut result = Blob::default();
        let hr = unsafe { ffi::DirectXTexFFI_SaveToHDRMemory(self.into(), (&mut result).into()) };
        hr.success(result)
    }

    pub fn save_tga(&self, flags: TGA_FLAGS, metadata: Option<&TexMetadata>) -> Result<Blob> {
        let mut result = Blob::default();
        let hr = unsafe {
            ffi::DirectXTexFFI_SaveToTGAMemory(
                self.into(),
                flags,
                (&mut result).into(),
                metadata.into_ffi_ptr(),
            )
        };
        hr.success(result)
    }

    /// Resize the image to width x height. Defaults to Fant filtering.
    ///
    /// Note for a complex resize, the result will always have mipLevels == 1
    pub fn resize(
        &self,
        width: usize,
        height: usize,
        filter: TEX_FILTER_FLAGS,
    ) -> Result<ScratchImage> {
        let mut result = ScratchImage::default();
        let hr = unsafe {
            ffi::DirectXTexFFI_Resize1(self.into(), width, height, filter, (&mut result).into())
        };
        hr.success(result)
    }

    /// Convert the image to a new format
    pub fn convert(
        &self,
        format: DXGI_FORMAT,
        filter: TEX_FILTER_FLAGS,
        threshold: f32,
    ) -> Result<ScratchImage> {
        let mut result = ScratchImage::default();
        let hr = unsafe {
            ffi::DirectXTexFFI_Convert1(
                self.into(),
                format,
                filter,
                threshold,
                (&mut result).into(),
            )
        };
        hr.success(result)
    }

    /// Converts the image from a planar format to an equivalent non-planar format
    pub fn convert_to_single_plane(&self) -> Result<ScratchImage> {
        let mut result = ScratchImage::default();
        let hr =
            unsafe { ffi::DirectXTexFFI_ConvertToSinglePlane1(self.into(), (&mut result).into()) };
        hr.success(result)
    }

    /// levels of '0' indicates a full mipchain, otherwise is generates that number of total levels (including the source base image)
    ///
    /// Defaults to Fant filtering which is equivalent to a box filter
    pub fn generate_mip_maps(
        &self,
        filter: TEX_FILTER_FLAGS,
        levels: usize,
        allow_1d: bool,
    ) -> Result<ScratchImage> {
        let mut result = ScratchImage::default();
        let hr = unsafe {
            ffi::DirectXTexFFI_GenerateMipMaps1(
                self.into(),
                filter,
                levels,
                (&mut result).into(),
                allow_1d,
            )
        };
        hr.success(result)
    }

    /// Converts to/from a premultiplied alpha version of the texture
    pub fn premultiply_alpha(&self, flags: TEX_PMALPHA_FLAGS) -> Result<ScratchImage> {
        let mut result = ScratchImage::default();
        let hr = unsafe {
            ffi::DirectXTexFFI_PremultiplyAlpha1(self.into(), flags, (&mut result).into())
        };
        hr.success(result)
    }

    /// Note that threshold is only used by BC1. TEX_THRESHOLD_DEFAULT is a typical value to use
    pub fn compress(
        &self,
        format: DXGI_FORMAT,
        compress: TEX_COMPRESS_FLAGS,
        threshold: f32,
    ) -> Result<ScratchImage> {
        let mut result = ScratchImage::default();
        let hr = unsafe {
            ffi::DirectXTexFFI_Compress1(
                self.into(),
                format,
                compress,
                threshold,
                (&mut result).into(),
            )
        };
        hr.success(result)
    }

    pub fn decompress(&self, format: DXGI_FORMAT) -> Result<ScratchImage> {
        let mut result = ScratchImage::default();
        let hr =
            unsafe { ffi::DirectXTexFFI_Decompress1(self.into(), format, (&mut result).into()) };
        hr.success(result)
    }
}

impl Default for Image {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            format: DXGI_FORMAT::default(),
            row_pitch: 0,
            slice_pitch: 0,
            pixels: ptr::null_mut(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ffi, Image, ScratchImage};
    use core::mem;
    use std::fs;

    #[test]
    fn verify_layout() {
        assert_eq!(mem::size_of::<Image>(), unsafe {
            ffi::DirectXTexFFI_Image_Sizeof()
        });
        assert_eq!(mem::align_of::<Image>(), unsafe {
            ffi::DirectXTexFFI_Image_Alignof()
        });
    }

    #[test]
    fn save_hdr() {
        let original = fs::read("data/ferris_wheel.hdr").unwrap();
        let scratch = ScratchImage::load_hdr(&original, None).unwrap();

        let images = scratch.images();
        assert_eq!(images.len(), 1);

        let image = &images[0];
        let copy = image.save_hdr().unwrap();
        let copy = copy.buffer();

        assert_eq!(original.len(), copy.len());
        assert_eq!(original, copy);
    }

    #[test]
    fn save_tga() {
        let original = fs::read("data/ferris_wheel.tga").unwrap();
        let scratch = ScratchImage::load_tga(&original, Default::default(), None).unwrap();

        let images = scratch.images();
        assert_eq!(images.len(), 1);

        let image = &images[0];
        let copy = image.save_tga(Default::default(), None).unwrap();
        let copy = copy.buffer();

        assert_eq!(original.len(), copy.len());
        assert_eq!(original, copy);
    }
}
