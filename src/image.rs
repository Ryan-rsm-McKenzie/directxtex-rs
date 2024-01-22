use crate::{
    ffi::{self, prelude::*},
    Blob, Result, TexMetadata, DDS_FLAGS, DXGI_FORMAT, TGA_FLAGS,
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
        let mut blob = Blob::default();
        let result =
            unsafe { ffi::DirectXTexFFI_SaveToDDSMemory1(self.into(), flags, (&mut blob).into()) };
        result.success().map(|()| blob)
    }

    pub fn save_hdr(&self) -> Result<Blob> {
        let mut blob = Blob::default();
        let result = unsafe { ffi::DirectXTexFFI_SaveToHDRMemory(self.into(), (&mut blob).into()) };
        result.success().map(|()| blob)
    }

    pub fn save_tga(&self, flags: TGA_FLAGS, metadata: Option<&TexMetadata>) -> Result<Blob> {
        let mut blob = Blob::default();
        let result = unsafe {
            ffi::DirectXTexFFI_SaveToTGAMemory(
                self.into(),
                flags,
                (&mut blob).into(),
                metadata.into_ffi_ptr(),
            )
        };
        result.success().map(|()| blob)
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
