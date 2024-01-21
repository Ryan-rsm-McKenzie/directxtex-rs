use crate::{ffi, HResultError, ScratchImageFFI, TexMetadata, CP_FLAGS, DXGI_FORMAT};
use core::{
    ptr::{self, NonNull},
    slice,
};

type Result<T> = core::result::Result<T, HResultError>;

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

#[derive(Debug)]
pub struct ScratchImage(NonNull<ScratchImageFFI>);

impl ScratchImage {
    #[must_use]
    pub fn new() -> Option<Self> {
        let this = unsafe { ffi::DirectXTexFFI_ScratchImage_Ctor() };
        NonNull::new(this).map(Self)
    }
}

impl From<&ScratchImage> for ffi::ConstPtr<ScratchImageFFI> {
    fn from(value: &ScratchImage) -> Self {
        ffi::ConstPtr::new(value.0)
    }
}

impl From<&mut ScratchImage> for ffi::MutPtr<ScratchImageFFI> {
    fn from(value: &mut ScratchImage) -> Self {
        ffi::MutPtr::new(value.0)
    }
}

impl Drop for ScratchImage {
    fn drop(&mut self) {
        unsafe { ffi::DirectXTexFFI_ScratchImage_Dtor(self.into()) }
    }
}

impl ScratchImage {
    pub fn initialize(&mut self, mdata: &TexMetadata, flags: CP_FLAGS) -> Result<()> {
        let result =
            unsafe { ffi::DirectXTexFFI_ScratchImage_Initialize(self.into(), mdata.into(), flags) };
        result.success()
    }

    pub fn initialize_1d(
        &mut self,
        fmt: DXGI_FORMAT,
        length: usize,
        array_size: usize,
        mip_levels: usize,
        flags: CP_FLAGS,
    ) -> Result<()> {
        let result = unsafe {
            ffi::DirectXTexFFI_ScratchImage_Initialize1D(
                self.into(),
                fmt,
                length,
                array_size,
                mip_levels,
                flags,
            )
        };
        result.success()
    }

    pub fn initialize_2d(
        &mut self,
        fmt: DXGI_FORMAT,
        width: usize,
        height: usize,
        array_size: usize,
        mip_levels: usize,
        flags: CP_FLAGS,
    ) -> Result<()> {
        let result = unsafe {
            ffi::DirectXTexFFI_ScratchImage_Initialize2D(
                self.into(),
                fmt,
                width,
                height,
                array_size,
                mip_levels,
                flags,
            )
        };
        result.success()
    }

    pub fn initialize_3d(
        &mut self,
        fmt: DXGI_FORMAT,
        width: usize,
        height: usize,
        depth: usize,
        mip_levels: usize,
        flags: CP_FLAGS,
    ) -> Result<()> {
        let result = unsafe {
            ffi::DirectXTexFFI_ScratchImage_Initialize3D(
                self.into(),
                fmt,
                width,
                height,
                depth,
                mip_levels,
                flags,
            )
        };
        result.success()
    }

    pub fn initialize_cube(
        &mut self,
        fmt: DXGI_FORMAT,
        width: usize,
        height: usize,
        cubes: usize,
        mip_levels: usize,
        flags: CP_FLAGS,
    ) -> Result<()> {
        let result = unsafe {
            ffi::DirectXTexFFI_ScratchImage_InitializeCube(
                self.into(),
                fmt,
                width,
                height,
                cubes,
                mip_levels,
                flags,
            )
        };
        result.success()
    }

    pub fn initialize_from_image(
        &mut self,
        src_image: &Image,
        allow_1d: bool,
        flags: CP_FLAGS,
    ) -> Result<()> {
        let result = unsafe {
            ffi::DirectXTexFFI_ScratchImage_InitializeFromImage(
                self.into(),
                src_image.into(),
                allow_1d,
                flags,
            )
        };
        result.success()
    }

    pub fn initialize_array_from_images(
        &mut self,
        images: &[Image],
        allow_1d: bool,
        flags: CP_FLAGS,
    ) -> Result<()> {
        let result = unsafe {
            ffi::DirectXTexFFI_ScratchImage_InitializeArrayFromImages(
                self.into(),
                images.as_ptr(),
                images.len(),
                allow_1d,
                flags,
            )
        };
        result.success()
    }

    pub fn initialize_cube_from_images(&mut self, images: &[Image], flags: CP_FLAGS) -> Result<()> {
        let result = unsafe {
            ffi::DirectXTexFFI_ScratchImage_InitializeCubeFromImages(
                self.into(),
                images.as_ptr(),
                images.len(),
                flags,
            )
        };
        result.success()
    }

    pub fn initialize_3d_from_images(&mut self, images: &[Image], flags: CP_FLAGS) -> Result<()> {
        let result = unsafe {
            ffi::DirectXTexFFI_ScratchImage_Initialize3DFromImages(
                self.into(),
                images.as_ptr(),
                images.len(),
                flags,
            )
        };
        result.success()
    }

    pub fn release(&mut self) {
        unsafe { ffi::DirectXTexFFI_ScratchImage_Release(self.into()) }
    }

    #[must_use]
    pub fn override_format(&mut self, f: DXGI_FORMAT) -> Option<()> {
        let result = unsafe { ffi::DirectXTexFFI_ScratchImage_OverrideFormat(self.into(), f) };
        result.then_some(())
    }

    #[must_use]
    pub fn get_metadata(&self) -> &TexMetadata {
        let result = unsafe { ffi::DirectXTexFFI_ScratchImage_GetMetadata(self.into()) };
        unsafe { result.as_ref() }
    }

    #[must_use]
    pub fn get_image(&self, mip: usize, item: usize, slice: usize) -> Option<&Image> {
        let result =
            unsafe { ffi::DirectXTexFFI_ScratchImage_GetImage(self.into(), mip, item, slice) };
        unsafe { result.as_ref() }
    }

    #[must_use]
    pub fn get_images(&self) -> &[Image] {
        let len = unsafe { ffi::DirectXTexFFI_ScratchImage_GetImageCount(self.into()) };
        let ptr = unsafe { ffi::DirectXTexFFI_ScratchImage_GetImages(self.into()) };
        let ptr = NonNull::new(ptr.cast_mut()).unwrap_or(NonNull::dangling());
        unsafe { slice::from_raw_parts(ptr.as_ptr(), len) }
    }

    #[must_use]
    pub fn do_get_pixels(&self) -> (NonNull<u8>, usize) {
        let len = unsafe { ffi::DirectXTexFFI_ScratchImage_GetPixelsSize(self.into()) };
        let ptr = unsafe { ffi::DirectXTexFFI_ScratchImage_GetPixels(self.into()) };
        let ptr = NonNull::new(ptr).unwrap_or(NonNull::dangling());
        (ptr, len)
    }

    #[must_use]
    pub fn get_pixels(&self) -> &[u8] {
        let (ptr, len) = self.do_get_pixels();
        unsafe { slice::from_raw_parts(ptr.as_ptr(), len) }
    }

    #[must_use]
    pub fn get_pixels_mut(&mut self) -> &mut [u8] {
        let (ptr, len) = self.do_get_pixels();
        unsafe { slice::from_raw_parts_mut(ptr.as_ptr(), len) }
    }

    #[must_use]
    pub fn is_alpha_all_opaque(&self) -> bool {
        unsafe { ffi::DirectXTexFFI_ScratchImage_IsAlphaAllOpaque(self.into()) }
    }
}
