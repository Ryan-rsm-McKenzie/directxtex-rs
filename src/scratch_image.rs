use crate::{
    ffi::{self, prelude::*},
    Blob, DDSMetaData, Image, Result, TexMetadata, CP_FLAGS, DDS_FLAGS, DXGI_FORMAT, TGA_FLAGS,
};
use core::ptr;

#[derive(Debug)]
#[repr(C)]
pub struct ScratchImage {
    m_nimages: usize,
    m_size: usize,
    m_metadata: TexMetadata,
    m_image: *mut Image,
    m_memory: *mut u8,
}

impl Default for ScratchImage {
    fn default() -> Self {
        Self {
            m_nimages: 0,
            m_size: 0,
            m_metadata: TexMetadata::default(),
            m_image: ptr::null_mut(),
            m_memory: ptr::null_mut(),
        }
    }
}

impl Drop for ScratchImage {
    fn drop(&mut self) {
        self.release();
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
                images.as_ffi_ptr(),
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
                images.as_ffi_ptr(),
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
                images.as_ffi_ptr(),
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
        &self.m_metadata
    }

    #[must_use]
    pub fn get_image(&self, mip: usize, item: usize, slice: usize) -> Option<&Image> {
        let result =
            unsafe { ffi::DirectXTexFFI_ScratchImage_GetImage(self.into(), mip, item, slice) };
        unsafe { result.as_ref() }
    }

    #[must_use]
    pub fn get_images(&self) -> &[Image] {
        unsafe { ffi::from_raw_ffi_parts(self.m_image, self.m_nimages) }
    }

    #[must_use]
    pub fn get_pixels(&self) -> &[u8] {
        unsafe { ffi::from_raw_ffi_parts(self.m_memory, self.m_size) }
    }

    #[must_use]
    pub fn get_pixels_mut(&mut self) -> &mut [u8] {
        unsafe { ffi::from_raw_ffi_parts_mut(self.m_memory, self.m_size) }
    }

    #[must_use]
    pub fn is_alpha_all_opaque(&self) -> bool {
        unsafe { ffi::DirectXTexFFI_ScratchImage_IsAlphaAllOpaque(self.into()) }
    }

    pub fn load_dds(
        source: &[u8],
        flags: DDS_FLAGS,
        metadata: Option<&mut TexMetadata>,
        dd_pixel_format: Option<&mut DDSMetaData>,
    ) -> Result<Self> {
        let mut image = Self::default();
        let result = unsafe {
            ffi::DirectXTexFFI_LoadFromDDSMemoryEx(
                source.as_ffi_ptr(),
                source.len(),
                flags,
                metadata.into_ffi_ptr(),
                dd_pixel_format.into_ffi_ptr(),
                (&mut image).into(),
            )
        };
        result.success().map(|()| image)
    }

    pub fn save_dds(&self, flags: DDS_FLAGS, metadata: Option<&TexMetadata>) -> Result<Blob> {
        let mut blob = Blob::default();
        let images = self.get_images();
        let result = unsafe {
            ffi::DirectXTexFFI_SaveToDDSMemory2(
                images.as_ffi_ptr(),
                images.len(),
                metadata.unwrap_or(self.get_metadata()).into(),
                flags,
                (&mut blob).into(),
            )
        };
        result.success().map(|()| blob)
    }

    pub fn load_hdr(source: &[u8], metadata: Option<&mut TexMetadata>) -> Result<Self> {
        let mut image = Self::default();
        let result = unsafe {
            ffi::DirectXTexFFI_LoadFromHDRMemory(
                source.as_ffi_ptr(),
                source.len(),
                metadata.into_ffi_ptr(),
                (&mut image).into(),
            )
        };
        result.success().map(|()| image)
    }

    pub fn load_tga(
        source: &[u8],
        flags: TGA_FLAGS,
        metadata: Option<&mut TexMetadata>,
    ) -> Result<Self> {
        let mut image = Self::default();
        let result = unsafe {
            ffi::DirectXTexFFI_LoadFromTGAMemory(
                source.as_ffi_ptr(),
                source.len(),
                flags,
                metadata.into_ffi_ptr(),
                (&mut image).into(),
            )
        };
        result.success().map(|()| image)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ffi, ScratchImage, DXGI_FORMAT, TEX_ALPHA_MODE, TEX_DIMENSION};
    use core::mem;
    use std::fs;

    #[test]
    fn verify_layout() {
        assert_eq!(mem::size_of::<ScratchImage>(), unsafe {
            ffi::DirectXTexFFI_ScratchImage_Sizeof()
        });
        assert_eq!(mem::align_of::<ScratchImage>(), unsafe {
            ffi::DirectXTexFFI_ScratchImage_Alignof()
        });
    }

    #[test]
    fn load_dds() {
        let source = fs::read("data/ferris_wheel.dds").unwrap();
        let (scratch, meta, dds) = {
            let mut meta = Default::default();
            let mut dds = Default::default();
            let scratch = ScratchImage::load_dds(
                &source,
                Default::default(),
                Some(&mut meta),
                Some(&mut dds),
            )
            .unwrap();
            (scratch, meta, dds)
        };

        assert_eq!(meta.width, 720);
        assert_eq!(meta.height, 1280);
        assert_eq!(meta.depth, 1);
        assert_eq!(meta.array_size, 1);
        assert_eq!(meta.mip_levels, 11);
        assert_eq!(meta.misc_flags, 0);
        assert_eq!(meta.misc_flags2, 0);
        assert_eq!(meta.format, DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM);
        assert_eq!(meta.dimension, TEX_DIMENSION::TEX_DIMENSION_TEXTURE2D);
        assert_eq!(
            meta.get_alpha_mode(),
            TEX_ALPHA_MODE::TEX_ALPHA_MODE_UNKNOWN
        );

        assert_eq!(dds.size, 32);
        assert_eq!(dds.flags, 65);
        assert_eq!(dds.four_cc, 0);
        assert_eq!(dds.rgb_bit_count, 32);
        assert_eq!(dds.r_bit_mask, 0x000000FF);
        assert_eq!(dds.g_bit_mask, 0x0000FF00);
        assert_eq!(dds.b_bit_mask, 0x00FF0000);
        assert_eq!(dds.a_bit_mask, 0xFF000000);

        assert_eq!(scratch.get_metadata(), &meta);
        assert_eq!(scratch.get_pixels().len(), 4915052);

        let images = scratch.get_images();
        assert_eq!(images.len(), 11);

        let mut index = 0;
        let mut check_image = |width, height, row_pitch, slice_pitch| {
            let image = &images[index];
            index += 1;
            assert_eq!(image.width, width);
            assert_eq!(image.height, height);
            assert_eq!(image.format, DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM);
            assert_eq!(image.row_pitch, row_pitch);
            assert_eq!(image.slice_pitch, slice_pitch);
        };

        check_image(720, 1280, 2880, 3686400);
        check_image(360, 640, 1440, 921600);
        check_image(180, 320, 720, 230400);
        check_image(90, 160, 360, 57600);
        check_image(45, 80, 180, 14400);
        check_image(22, 40, 88, 3520);
        check_image(11, 20, 44, 880);
        check_image(5, 10, 20, 200);
        check_image(2, 5, 8, 40);
        check_image(1, 2, 4, 8);
        check_image(1, 1, 4, 4);
    }

    #[test]
    fn save_dds() {
        let original = fs::read("data/ferris_wheel.dds").unwrap();
        let scratch = ScratchImage::load_dds(&original, Default::default(), None, None).unwrap();
        let copy = scratch.save_dds(Default::default(), None).unwrap();
        let copy = copy.get_buffer();
        assert_eq!(original.len(), copy.len());
        assert_eq!(original, copy);
    }

    #[test]
    fn load_hdr() {
        let source = fs::read("data/ferris_wheel.hdr").unwrap();
        let (scratch, meta) = {
            let mut meta = Default::default();
            let scratch = ScratchImage::load_hdr(&source, Some(&mut meta)).unwrap();
            (scratch, meta)
        };

        assert_eq!(meta.width, 720);
        assert_eq!(meta.height, 1280);
        assert_eq!(meta.depth, 1);
        assert_eq!(meta.array_size, 1);
        assert_eq!(meta.mip_levels, 1);
        assert_eq!(meta.misc_flags, 0);
        assert_eq!(meta.misc_flags2, 3);
        assert_eq!(meta.format, DXGI_FORMAT::DXGI_FORMAT_R32G32B32A32_FLOAT);
        assert_eq!(meta.dimension, TEX_DIMENSION::TEX_DIMENSION_TEXTURE2D);
        assert_eq!(meta.get_alpha_mode(), TEX_ALPHA_MODE::TEX_ALPHA_MODE_OPAQUE);

        assert_ne!(scratch.get_metadata(), &meta);
        let tex = scratch.get_metadata();
        assert_eq!(tex.width, 720);
        assert_eq!(tex.height, 1280);
        assert_eq!(tex.depth, 1);
        assert_eq!(tex.array_size, 1);
        assert_eq!(tex.mip_levels, 1);
        assert_eq!(tex.misc_flags, 0);
        assert_eq!(tex.misc_flags2, 0);
        assert_eq!(tex.format, DXGI_FORMAT::DXGI_FORMAT_R32G32B32A32_FLOAT);
        assert_eq!(tex.dimension, TEX_DIMENSION::TEX_DIMENSION_TEXTURE2D);
        assert_eq!(tex.get_alpha_mode(), TEX_ALPHA_MODE::TEX_ALPHA_MODE_UNKNOWN);

        assert_eq!(scratch.get_pixels().len(), 14745600);

        let images = scratch.get_images();
        assert_eq!(images.len(), 1);
        assert_eq!(images[0].width, 720);
        assert_eq!(images[0].height, 1280);
        assert_eq!(
            images[0].format,
            DXGI_FORMAT::DXGI_FORMAT_R32G32B32A32_FLOAT
        );
        assert_eq!(images[0].row_pitch, 11520);
        assert_eq!(images[0].slice_pitch, 14745600);
    }

    #[test]
    fn load_tga() {
        let source = fs::read("data/ferris_wheel.tga").unwrap();
        let (scratch, meta) = {
            let mut meta = Default::default();
            let scratch =
                ScratchImage::load_tga(&source, Default::default(), Some(&mut meta)).unwrap();
            (scratch, meta)
        };

        assert_eq!(meta.width, 720);
        assert_eq!(meta.height, 1280);
        assert_eq!(meta.depth, 1);
        assert_eq!(meta.array_size, 1);
        assert_eq!(meta.mip_levels, 1);
        assert_eq!(meta.misc_flags, 0);
        assert_eq!(meta.misc_flags2, 3);
        assert_eq!(meta.format, DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM);
        assert_eq!(meta.dimension, TEX_DIMENSION::TEX_DIMENSION_TEXTURE2D);
        assert_eq!(meta.get_alpha_mode(), TEX_ALPHA_MODE::TEX_ALPHA_MODE_OPAQUE);

        assert_ne!(scratch.get_metadata(), &meta);
        let tex = scratch.get_metadata();
        assert_eq!(tex.width, 720);
        assert_eq!(tex.height, 1280);
        assert_eq!(tex.depth, 1);
        assert_eq!(tex.array_size, 1);
        assert_eq!(tex.mip_levels, 1);
        assert_eq!(tex.misc_flags, 0);
        assert_eq!(tex.misc_flags2, 0);
        assert_eq!(tex.format, DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM);
        assert_eq!(tex.dimension, TEX_DIMENSION::TEX_DIMENSION_TEXTURE2D);
        assert_eq!(tex.get_alpha_mode(), TEX_ALPHA_MODE::TEX_ALPHA_MODE_UNKNOWN);

        assert_eq!(scratch.get_pixels().len(), 3686400);

        let images = scratch.get_images();
        assert_eq!(images.len(), 1);
        assert_eq!(images[0].width, 720);
        assert_eq!(images[0].height, 1280);
        assert_eq!(images[0].format, DXGI_FORMAT::DXGI_FORMAT_R8G8B8A8_UNORM);
        assert_eq!(images[0].row_pitch, 2880);
        assert_eq!(images[0].slice_pitch, 3686400);
    }
}
