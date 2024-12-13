use crate::{
    Blob, DDSMetaData, HResult, Image, Rect, ScratchImage, TexMetadata, CMSE_FLAGS, CNMAP_FLAGS,
    CP_FLAGS, DDS_FLAGS, DXGI_FORMAT, FORMAT_TYPE, TEX_COMPRESS_FLAGS, TEX_FILTER_FLAGS,
    TEX_PMALPHA_FLAGS, TGA_FLAGS,
};
use core::{
    ptr::{self, NonNull},
    slice,
};

pub(crate) mod prelude {
    pub(crate) use super::OptionExt as _;
    pub(crate) use super::SliceExt as _;
}

pub(crate) trait SliceExt<T> {
    fn as_ffi_ptr(&self) -> *const T;
    fn as_mut_ffi_ptr(&mut self) -> *mut T;
}

impl<T> SliceExt<T> for [T] {
    fn as_ffi_ptr(&self) -> *const T {
        if self.is_empty() {
            ptr::null()
        } else {
            self.as_ptr()
        }
    }

    fn as_mut_ffi_ptr(&mut self) -> *mut T {
        if self.is_empty() {
            ptr::null_mut()
        } else {
            self.as_mut_ptr()
        }
    }
}

pub(crate) trait OptionExt {
    type Item;
    fn into_ffi_ptr(self) -> Self::Item;
}

impl<T> OptionExt for Option<&T> {
    type Item = *const T;
    fn into_ffi_ptr(self) -> *const T {
        match self {
            Some(some) => ptr::addr_of!(*some),
            None => ptr::null(),
        }
    }
}

impl<T> OptionExt for Option<&mut T> {
    type Item = *mut T;
    fn into_ffi_ptr(self) -> *mut T {
        match self {
            Some(some) => ptr::addr_of_mut!(*some),
            None => ptr::null_mut(),
        }
    }
}

pub(crate) unsafe fn from_raw_ffi_parts<'a, T>(data: *const T, len: usize) -> &'a [T] {
    let data = NonNull::new(data.cast_mut()).unwrap_or(NonNull::dangling());
    slice::from_raw_parts(data.as_ptr(), len)
}

pub(crate) unsafe fn from_raw_ffi_parts_mut<'a, T>(data: *mut T, len: usize) -> &'a mut [T] {
    let data = NonNull::new(data).unwrap_or(NonNull::dangling());
    slice::from_raw_parts_mut(data.as_ptr(), len)
}

#[repr(transparent)]
pub(crate) struct ConstNonNull<T>(NonNull<T>);

impl<T> From<&T> for ConstNonNull<T> {
    fn from(value: &T) -> Self {
        Self(value.into())
    }
}

#[repr(transparent)]
pub(crate) struct MutNonNull<T>(NonNull<T>);

impl<T> From<&mut T> for MutNonNull<T> {
    fn from(value: &mut T) -> Self {
        Self(value.into())
    }
}

#[allow(non_snake_case)]
#[link(name = "directxtex-ffi")]
unsafe extern "C" {
    //---------------------------------------------------------------------------------
    // DXGI Format Utilities

    pub(crate) unsafe fn DirectXTexFFI_IsPacked(fmt: DXGI_FORMAT) -> bool;
    pub(crate) unsafe fn DirectXTexFFI_IsVideo(fmt: DXGI_FORMAT) -> bool;
    pub(crate) unsafe fn DirectXTexFFI_IsPlanar(fmt: DXGI_FORMAT) -> bool;
    pub(crate) unsafe fn DirectXTexFFI_IsDepthStencil(fmt: DXGI_FORMAT) -> bool;
    pub(crate) unsafe fn DirectXTexFFI_IsBGR(fmt: DXGI_FORMAT) -> bool;
    pub(crate) unsafe fn DirectXTexFFI_IsTypeless(fmt: DXGI_FORMAT, partialTypeless: bool) -> bool;

    pub(crate) unsafe fn DirectXTexFFI_HasAlpha(fmt: DXGI_FORMAT) -> bool;

    pub(crate) unsafe fn DirectXTexFFI_BitsPerPixel(fmt: DXGI_FORMAT) -> usize;

    pub(crate) unsafe fn DirectXTexFFI_BitsPerColor(fmt: DXGI_FORMAT) -> usize;

    pub(crate) unsafe fn DirectXTexFFI_FormatDataType(fmt: DXGI_FORMAT) -> FORMAT_TYPE;

    pub(crate) unsafe fn DirectXTexFFI_ComputePitch(
        fmt: DXGI_FORMAT,
        width: usize,
        height: usize,
        rowPitch: MutNonNull<usize>,
        slicePitch: MutNonNull<usize>,
        flags: CP_FLAGS,
    ) -> HResult;

    pub(crate) unsafe fn DirectXTexFFI_ComputeScanlines(fmt: DXGI_FORMAT, height: usize) -> usize;

    pub(crate) unsafe fn DirectXTexFFI_MakeSRGB(fmt: DXGI_FORMAT) -> DXGI_FORMAT;
    pub(crate) unsafe fn DirectXTexFFI_MakeLinear(fmt: DXGI_FORMAT) -> DXGI_FORMAT;
    pub(crate) unsafe fn DirectXTexFFI_MakeTypeless(fmt: DXGI_FORMAT) -> DXGI_FORMAT;
    pub(crate) unsafe fn DirectXTexFFI_MakeTypelessUNORM(fmt: DXGI_FORMAT) -> DXGI_FORMAT;
    pub(crate) unsafe fn DirectXTexFFI_MakeTypelessFLOAT(fmt: DXGI_FORMAT) -> DXGI_FORMAT;

    //---------------------------------------------------------------------------------
    // Texture metadata

    #[cfg(test)]
    pub(crate) unsafe fn DirectXTexFFI_TexMetadata_Sizeof() -> usize;
    #[cfg(test)]
    pub(crate) unsafe fn DirectXTexFFI_TexMetadata_Alignof() -> usize;

    #[cfg(test)]
    pub(crate) unsafe fn DirectXTexFFI_DDSMetaData_Sizeof() -> usize;
    #[cfg(test)]
    pub(crate) unsafe fn DirectXTexFFI_DDSMetaData_Alignof() -> usize;

    pub(crate) unsafe fn DirectXTexFFI_TexMetadata_ComputIndex(
        this: ConstNonNull<TexMetadata>,
        mip: usize,
        item: usize,
        slice: usize,
    ) -> usize;

    pub(crate) unsafe fn DirectXTexFFI_GetMetadataFromDDSMemoryEx(
        pSource: *const u8,
        size: usize,
        flags: DDS_FLAGS,
        metadata: MutNonNull<TexMetadata>,
        ddPixelFormat: *mut DDSMetaData,
    ) -> HResult;

    pub(crate) unsafe fn DirectXTexFFI_GetMetadataFromHDRMemory(
        pSource: *const u8,
        size: usize,
        metadata: MutNonNull<TexMetadata>,
    ) -> HResult;

    pub(crate) unsafe fn DirectXTexFFI_GetMetadataFromTGAMemory(
        pSource: *const u8,
        size: usize,
        flags: TGA_FLAGS,
        metadata: MutNonNull<TexMetadata>,
    ) -> HResult;

    //---------------------------------------------------------------------------------
    // Bitmap image container

    #[cfg(test)]
    pub(crate) unsafe fn DirectXTexFFI_Image_Sizeof() -> usize;
    #[cfg(test)]
    pub(crate) unsafe fn DirectXTexFFI_Image_Alignof() -> usize;

    #[cfg(test)]
    pub(crate) unsafe fn DirectXTexFFI_ScratchImage_Sizeof() -> usize;
    #[cfg(test)]
    pub(crate) unsafe fn DirectXTexFFI_ScratchImage_Alignof() -> usize;

    pub(crate) unsafe fn DirectXTexFFI_ScratchImage_Initialize(
        this: MutNonNull<ScratchImage>,
        mdata: ConstNonNull<TexMetadata>,
        flags: CP_FLAGS,
    ) -> HResult;

    pub(crate) unsafe fn DirectXTexFFI_ScratchImage_Initialize1D(
        this: MutNonNull<ScratchImage>,
        fmt: DXGI_FORMAT,
        length: usize,
        arraySize: usize,
        mipLevels: usize,
        flags: CP_FLAGS,
    ) -> HResult;
    pub(crate) unsafe fn DirectXTexFFI_ScratchImage_Initialize2D(
        this: MutNonNull<ScratchImage>,
        fmt: DXGI_FORMAT,
        width: usize,
        height: usize,
        arraySize: usize,
        mipLevels: usize,
        flags: CP_FLAGS,
    ) -> HResult;
    pub(crate) unsafe fn DirectXTexFFI_ScratchImage_Initialize3D(
        this: MutNonNull<ScratchImage>,
        fmt: DXGI_FORMAT,
        width: usize,
        height: usize,
        depth: usize,
        mipLevels: usize,
        flags: CP_FLAGS,
    ) -> HResult;
    pub(crate) unsafe fn DirectXTexFFI_ScratchImage_InitializeCube(
        this: MutNonNull<ScratchImage>,
        fmt: DXGI_FORMAT,
        width: usize,
        height: usize,
        nCubes: usize,
        mipLevels: usize,
        flags: CP_FLAGS,
    ) -> HResult;

    pub(crate) unsafe fn DirectXTexFFI_ScratchImage_InitializeFromImage(
        this: MutNonNull<ScratchImage>,
        srcImage: ConstNonNull<Image>,
        allow1D: bool,
        flags: CP_FLAGS,
    ) -> HResult;
    pub(crate) unsafe fn DirectXTexFFI_ScratchImage_InitializeArrayFromImages(
        this: MutNonNull<ScratchImage>,
        images: *const Image,
        nImages: usize,
        allow1D: bool,
        flags: CP_FLAGS,
    ) -> HResult;
    pub(crate) unsafe fn DirectXTexFFI_ScratchImage_InitializeCubeFromImages(
        this: MutNonNull<ScratchImage>,
        images: *const Image,
        nImages: usize,
        flags: CP_FLAGS,
    ) -> HResult;
    pub(crate) unsafe fn DirectXTexFFI_ScratchImage_Initialize3DFromImages(
        this: MutNonNull<ScratchImage>,
        images: *const Image,
        depth: usize,
        flags: CP_FLAGS,
    ) -> HResult;

    pub(crate) unsafe fn DirectXTexFFI_ScratchImage_Release(this: MutNonNull<ScratchImage>);

    pub(crate) unsafe fn DirectXTexFFI_ScratchImage_OverrideFormat(
        this: MutNonNull<ScratchImage>,
        f: DXGI_FORMAT,
    ) -> bool;

    pub(crate) unsafe fn DirectXTexFFI_ScratchImage_GetImage(
        this: ConstNonNull<ScratchImage>,
        mip: usize,
        item: usize,
        slice: usize,
    ) -> *const Image;

    pub(crate) unsafe fn DirectXTexFFI_ScratchImage_IsAlphaAllOpaque(
        this: ConstNonNull<ScratchImage>,
    ) -> bool;

    //---------------------------------------------------------------------------------
    // Memory blob (allocated buffer pointer is always 16-byte aligned)

    #[cfg(test)]
    pub(crate) unsafe fn DirectXTexFFI_Blob_Sizeof() -> usize;
    #[cfg(test)]
    pub(crate) unsafe fn DirectXTexFFI_Blob_Alignof() -> usize;

    pub(crate) unsafe fn DirectXTexFFI_Blob_Initialize(
        this: MutNonNull<Blob>,
        size: usize,
    ) -> HResult;
    pub(crate) unsafe fn DirectXTexFFI_Blob_Release(this: MutNonNull<Blob>);
    pub(crate) unsafe fn DirectXTexFFI_Blob_Resize(this: MutNonNull<Blob>, size: usize) -> HResult;
    pub(crate) unsafe fn DirectXTexFFI_Blob_Trim(this: MutNonNull<Blob>, size: usize) -> HResult;

    //---------------------------------------------------------------------------------
    // Image I/O

    // DDS operations
    pub(crate) unsafe fn DirectXTexFFI_LoadFromDDSMemoryEx(
        pSource: *const u8,
        size: usize,
        flags: DDS_FLAGS,
        metadata: *mut TexMetadata,
        ddPixelFormat: *mut DDSMetaData,
        image: MutNonNull<ScratchImage>,
    ) -> HResult;

    pub(crate) unsafe fn DirectXTexFFI_SaveToDDSMemory1(
        image: ConstNonNull<Image>,
        flags: DDS_FLAGS,
        blob: MutNonNull<Blob>,
    ) -> HResult;
    pub(crate) unsafe fn DirectXTexFFI_SaveToDDSMemory2(
        images: *const Image,
        nimages: usize,
        metadata: ConstNonNull<TexMetadata>,
        flags: DDS_FLAGS,
        blob: MutNonNull<Blob>,
    ) -> HResult;

    // HDR operations
    pub(crate) unsafe fn DirectXTexFFI_LoadFromHDRMemory(
        pSource: *const u8,
        size: usize,
        metadata: *mut TexMetadata,
        image: MutNonNull<ScratchImage>,
    ) -> HResult;

    pub(crate) unsafe fn DirectXTexFFI_SaveToHDRMemory(
        image: ConstNonNull<Image>,
        blob: MutNonNull<Blob>,
    ) -> HResult;

    // TGA operations
    pub(crate) unsafe fn DirectXTexFFI_LoadFromTGAMemory(
        pSource: *const u8,
        size: usize,
        flags: TGA_FLAGS,
        metadata: *mut TexMetadata,
        image: MutNonNull<ScratchImage>,
    ) -> HResult;

    pub(crate) unsafe fn DirectXTexFFI_SaveToTGAMemory(
        image: ConstNonNull<Image>,
        flags: TGA_FLAGS,
        blob: MutNonNull<Blob>,
        metadata: *const TexMetadata,
    ) -> HResult;

    //---------------------------------------------------------------------------------
    // Texture conversion, resizing, mipmap generation, and block compression

    pub(crate) unsafe fn DirectXTexFFI_Resize1(
        srcImage: ConstNonNull<Image>,
        width: usize,
        height: usize,
        filter: TEX_FILTER_FLAGS,
        image: MutNonNull<ScratchImage>,
    ) -> HResult;
    pub(crate) unsafe fn DirectXTexFFI_Resize2(
        srcImages: *const Image,
        nimages: usize,
        metadata: ConstNonNull<TexMetadata>,
        width: usize,
        height: usize,
        filter: TEX_FILTER_FLAGS,
        result: MutNonNull<ScratchImage>,
    ) -> HResult;

    pub(crate) unsafe fn DirectXTexFFI_Convert1(
        srcImage: ConstNonNull<Image>,
        format: DXGI_FORMAT,
        filter: TEX_FILTER_FLAGS,
        threshold: f32,
        image: MutNonNull<ScratchImage>,
    ) -> HResult;
    pub(crate) unsafe fn DirectXTexFFI_Convert2(
        srcImages: *const Image,
        nimages: usize,
        metadata: ConstNonNull<TexMetadata>,
        format: DXGI_FORMAT,
        filter: TEX_FILTER_FLAGS,
        threshold: f32,
        result: MutNonNull<ScratchImage>,
    ) -> HResult;

    pub(crate) unsafe fn DirectXTexFFI_ConvertToSinglePlane1(
        srcImage: ConstNonNull<Image>,
        image: MutNonNull<ScratchImage>,
    ) -> HResult;
    pub(crate) unsafe fn DirectXTexFFI_ConvertToSinglePlane2(
        srcImages: *const Image,
        nimages: usize,
        metadata: ConstNonNull<TexMetadata>,
        image: MutNonNull<ScratchImage>,
    ) -> HResult;

    pub(crate) unsafe fn DirectXTexFFI_GenerateMipMaps1(
        baseImage: ConstNonNull<Image>,
        filter: TEX_FILTER_FLAGS,
        levels: usize,
        mipChain: MutNonNull<ScratchImage>,
        allow1D: bool,
    ) -> HResult;
    pub(crate) unsafe fn DirectXTexFFI_GenerateMipMaps2(
        srcImages: *const Image,
        nimages: usize,
        metadata: ConstNonNull<TexMetadata>,
        filter: TEX_FILTER_FLAGS,
        levels: usize,
        mipChain: MutNonNull<ScratchImage>,
    ) -> HResult;

    pub(crate) unsafe fn DirectXTexFFI_GenerateMipMaps3D1(
        baseImages: *const Image,
        depth: usize,
        filter: TEX_FILTER_FLAGS,
        levels: usize,
        mipChain: MutNonNull<ScratchImage>,
    ) -> HResult;
    pub(crate) unsafe fn DirectXTexFFI_GenerateMipMaps3D2(
        srcImages: *const Image,
        nimages: usize,
        metadata: ConstNonNull<TexMetadata>,
        filter: TEX_FILTER_FLAGS,
        levels: usize,
        mipChain: MutNonNull<ScratchImage>,
    ) -> HResult;

    pub(crate) unsafe fn DirectXTexFFI_ScaleMipMapsAlphaForCoverage(
        srcImages: *const Image,
        nimages: usize,
        metadata: ConstNonNull<TexMetadata>,
        item: usize,
        alphaReference: f32,
        mipChain: MutNonNull<ScratchImage>,
    ) -> HResult;

    pub(crate) unsafe fn DirectXTexFFI_PremultiplyAlpha1(
        srcImage: ConstNonNull<Image>,
        flags: TEX_PMALPHA_FLAGS,
        image: MutNonNull<ScratchImage>,
    ) -> HResult;
    pub(crate) unsafe fn DirectXTexFFI_PremultiplyAlpha2(
        srcImages: *const Image,
        nimages: usize,
        metadata: ConstNonNull<TexMetadata>,
        flags: TEX_PMALPHA_FLAGS,
        result: MutNonNull<ScratchImage>,
    ) -> HResult;

    pub(crate) unsafe fn DirectXTexFFI_Compress1(
        srcImage: ConstNonNull<Image>,
        format: DXGI_FORMAT,
        compress: TEX_COMPRESS_FLAGS,
        threshold: f32,
        cImage: MutNonNull<ScratchImage>,
    ) -> HResult;
    pub(crate) unsafe fn DirectXTexFFI_Compress2(
        srcImages: *const Image,
        nimages: usize,
        metadata: ConstNonNull<TexMetadata>,
        format: DXGI_FORMAT,
        compress: TEX_COMPRESS_FLAGS,
        threshold: f32,
        cImages: MutNonNull<ScratchImage>,
    ) -> HResult;

    pub(crate) unsafe fn DirectXTexFFI_Decompress1(
        cImage: ConstNonNull<Image>,
        format: DXGI_FORMAT,
        image: MutNonNull<ScratchImage>,
    ) -> HResult;
    pub(crate) unsafe fn DirectXTexFFI_Decompress2(
        cImages: *const Image,
        nimages: usize,
        metadata: ConstNonNull<TexMetadata>,
        format: DXGI_FORMAT,
        images: MutNonNull<ScratchImage>,
    ) -> HResult;

    //---------------------------------------------------------------------------------
    // Normal map operations

    pub(crate) unsafe fn DirectXTexFFI_ComputeNormalMap1(
        srcImage: ConstNonNull<Image>,
        flags: CNMAP_FLAGS,
        amplitude: f32,
        format: DXGI_FORMAT,
        normalMap: MutNonNull<ScratchImage>,
    ) -> HResult;
    pub(crate) unsafe fn DirectXTexFFI_ComputeNormalMap2(
        srcImages: *const Image,
        nimages: usize,
        metadata: ConstNonNull<TexMetadata>,
        flags: CNMAP_FLAGS,
        amplitude: f32,
        format: DXGI_FORMAT,
        normalMaps: MutNonNull<ScratchImage>,
    ) -> HResult;

    //---------------------------------------------------------------------------------
    // Misc image operations

    #[cfg(test)]
    pub(crate) unsafe fn DirectXTexFFI_Rect_Sizeof() -> usize;
    #[cfg(test)]
    pub(crate) unsafe fn DirectXTexFFI_Rect_Alignof() -> usize;

    pub(crate) unsafe fn DirectXTexFFI_CopyRectangle(
        srcImage: ConstNonNull<Image>,
        srcRect: ConstNonNull<Rect>,
        dstImage: MutNonNull<Image>,
        filter: TEX_FILTER_FLAGS,
        xOffset: usize,
        yOffset: usize,
    ) -> HResult;

    pub(crate) unsafe fn DirectXTexFFI_ComputeMSE(
        image1: ConstNonNull<Image>,
        image2: ConstNonNull<Image>,
        mse: MutNonNull<f32>,
        mseV: *mut f32,
        flags: CMSE_FLAGS,
    ) -> HResult;

    //---------------------------------------------------------------------------------
    // DDS helper functions

    pub(crate) unsafe fn DirectXTexFFI_EncodeDDSHeader(
        metadata: ConstNonNull<TexMetadata>,
        flags: DDS_FLAGS,
        pDestination: *mut u8,
        maxsize: usize,
        required: MutNonNull<usize>,
    ) -> HResult;
}
