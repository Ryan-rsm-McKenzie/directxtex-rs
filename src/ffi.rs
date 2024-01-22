use crate::{
    Blob, DDSMetaData, HResult, Image, ScratchImage, TexMetadata, CP_FLAGS, DDS_FLAGS, DXGI_FORMAT,
    FORMAT_TYPE, TGA_FLAGS,
};
use core::ptr::NonNull;

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
extern "C" {
    //---------------------------------------------------------------------------------
    // DXGI Format Utilities

    pub(crate) fn DirectXTexFFI_IsPacked(fmt: DXGI_FORMAT) -> bool;
    pub(crate) fn DirectXTexFFI_IsVideo(fmt: DXGI_FORMAT) -> bool;
    pub(crate) fn DirectXTexFFI_IsPlanar(fmt: DXGI_FORMAT) -> bool;
    pub(crate) fn DirectXTexFFI_IsDepthStencil(fmt: DXGI_FORMAT) -> bool;
    pub(crate) fn DirectXTexFFI_IsBGR(fmt: DXGI_FORMAT) -> bool;
    pub(crate) fn DirectXTexFFI_IsTypeless(fmt: DXGI_FORMAT, partialTypeless: bool) -> bool;

    pub(crate) fn DirectXTexFFI_HasAlpha(fmt: DXGI_FORMAT) -> bool;

    pub(crate) fn DirectXTexFFI_BitsPerPixel(fmt: DXGI_FORMAT) -> usize;

    pub(crate) fn DirectXTexFFI_BitsPerColor(fmt: DXGI_FORMAT) -> usize;

    pub(crate) fn DirectXTexFFI_FormatDataType(fmt: DXGI_FORMAT) -> FORMAT_TYPE;

    pub(crate) fn DirectXTexFFI_ComputePitch(
        fmt: DXGI_FORMAT,
        width: usize,
        height: usize,
        rowPitch: MutNonNull<usize>,
        slicePitch: MutNonNull<usize>,
        flags: CP_FLAGS,
    ) -> HResult;

    pub(crate) fn DirectXTexFFI_ComputeScanlines(fmt: DXGI_FORMAT, height: usize) -> usize;

    pub(crate) fn DirectXTexFFI_MakeSRGB(fmt: DXGI_FORMAT) -> DXGI_FORMAT;
    pub(crate) fn DirectXTexFFI_MakeLinear(fmt: DXGI_FORMAT) -> DXGI_FORMAT;
    pub(crate) fn DirectXTexFFI_MakeTypeless(fmt: DXGI_FORMAT) -> DXGI_FORMAT;
    pub(crate) fn DirectXTexFFI_MakeTypelessUNORM(fmt: DXGI_FORMAT) -> DXGI_FORMAT;
    pub(crate) fn DirectXTexFFI_MakeTypelessFLOAT(fmt: DXGI_FORMAT) -> DXGI_FORMAT;

    //---------------------------------------------------------------------------------
    // Texture metadata

    #[cfg(test)]
    pub(crate) fn DirectXTexFFI_TexMetadata_Sizeof() -> usize;
    #[cfg(test)]
    pub(crate) fn DirectXTexFFI_TexMetadata_Alignof() -> usize;

    #[cfg(test)]
    pub(crate) fn DirectXTexFFI_DDSMetaData_Sizeof() -> usize;
    #[cfg(test)]
    pub(crate) fn DirectXTexFFI_DDSMetaData_Alignof() -> usize;

    pub(crate) fn DirectXTexFFI_TexMetadata_ComputIndex(
        this: ConstNonNull<TexMetadata>,
        mip: usize,
        item: usize,
        slice: usize,
    ) -> usize;

    pub(crate) fn DirectXTexFFI_GetMetadataFromDDSMemoryEx(
        pSource: *const u8,
        size: usize,
        flags: DDS_FLAGS,
        metadata: MutNonNull<TexMetadata>,
        ddPixelFormat: *mut DDSMetaData,
    ) -> HResult;

    pub(crate) fn DirectXTexFFI_GetMetadataFromHDRMemory(
        pSource: *const u8,
        size: usize,
        metadata: MutNonNull<TexMetadata>,
    ) -> HResult;

    pub(crate) fn DirectXTexFFI_GetMetadataFromTGAMemory(
        pSource: *const u8,
        size: usize,
        flags: TGA_FLAGS,
        metadata: MutNonNull<TexMetadata>,
    ) -> HResult;

    //---------------------------------------------------------------------------------
    // Bitmap image container

    #[cfg(test)]
    pub(crate) fn DirectXTexFFI_Image_Sizeof() -> usize;
    #[cfg(test)]
    pub(crate) fn DirectXTexFFI_Image_Alignof() -> usize;

    #[cfg(test)]
    pub(crate) fn DirectXTexFFI_ScratchImage_Sizeof() -> usize;
    #[cfg(test)]
    pub(crate) fn DirectXTexFFI_ScratchImage_Alignof() -> usize;

    pub(crate) fn DirectXTexFFI_ScratchImage_Initialize(
        this: MutNonNull<ScratchImage>,
        mdata: ConstNonNull<TexMetadata>,
        flags: CP_FLAGS,
    ) -> HResult;

    pub(crate) fn DirectXTexFFI_ScratchImage_Initialize1D(
        this: MutNonNull<ScratchImage>,
        fmt: DXGI_FORMAT,
        length: usize,
        arraySize: usize,
        mipLevels: usize,
        flags: CP_FLAGS,
    ) -> HResult;
    pub(crate) fn DirectXTexFFI_ScratchImage_Initialize2D(
        this: MutNonNull<ScratchImage>,
        fmt: DXGI_FORMAT,
        width: usize,
        height: usize,
        arraySize: usize,
        mipLevels: usize,
        flags: CP_FLAGS,
    ) -> HResult;
    pub(crate) fn DirectXTexFFI_ScratchImage_Initialize3D(
        this: MutNonNull<ScratchImage>,
        fmt: DXGI_FORMAT,
        width: usize,
        height: usize,
        depth: usize,
        mipLevels: usize,
        flags: CP_FLAGS,
    ) -> HResult;
    pub(crate) fn DirectXTexFFI_ScratchImage_InitializeCube(
        this: MutNonNull<ScratchImage>,
        fmt: DXGI_FORMAT,
        width: usize,
        height: usize,
        nCubes: usize,
        mipLevels: usize,
        flags: CP_FLAGS,
    ) -> HResult;

    pub(crate) fn DirectXTexFFI_ScratchImage_InitializeFromImage(
        this: MutNonNull<ScratchImage>,
        srcImage: ConstNonNull<Image>,
        allow1D: bool,
        flags: CP_FLAGS,
    ) -> HResult;
    pub(crate) fn DirectXTexFFI_ScratchImage_InitializeArrayFromImages(
        this: MutNonNull<ScratchImage>,
        images: *const Image,
        nImages: usize,
        allow1D: bool,
        flags: CP_FLAGS,
    ) -> HResult;
    pub(crate) fn DirectXTexFFI_ScratchImage_InitializeCubeFromImages(
        this: MutNonNull<ScratchImage>,
        images: *const Image,
        nImages: usize,
        flags: CP_FLAGS,
    ) -> HResult;
    pub(crate) fn DirectXTexFFI_ScratchImage_Initialize3DFromImages(
        this: MutNonNull<ScratchImage>,
        images: *const Image,
        depth: usize,
        flags: CP_FLAGS,
    ) -> HResult;

    pub(crate) fn DirectXTexFFI_ScratchImage_Release(this: MutNonNull<ScratchImage>);

    pub(crate) fn DirectXTexFFI_ScratchImage_OverrideFormat(
        this: MutNonNull<ScratchImage>,
        f: DXGI_FORMAT,
    ) -> bool;

    pub(crate) fn DirectXTexFFI_ScratchImage_GetImage(
        this: ConstNonNull<ScratchImage>,
        mip: usize,
        item: usize,
        slice: usize,
    ) -> *const Image;

    pub(crate) fn DirectXTexFFI_ScratchImage_IsAlphaAllOpaque(
        this: ConstNonNull<ScratchImage>,
    ) -> bool;

    //---------------------------------------------------------------------------------
    // Memory blob (allocated buffer pointer is always 16-byte aligned)

    #[cfg(test)]
    pub(crate) fn DirectXTexFFI_Blob_Sizeof() -> usize;
    #[cfg(test)]
    pub(crate) fn DirectXTexFFI_Blob_Alignof() -> usize;

    pub(crate) fn DirectXTexFFI_Blob_Initialize(this: MutNonNull<Blob>, size: usize) -> HResult;
    pub(crate) fn DirectXTexFFI_Blob_Release(this: MutNonNull<Blob>);
    pub(crate) fn DirectXTexFFI_Blob_Resize(this: MutNonNull<Blob>, size: usize) -> HResult;
    pub(crate) fn DirectXTexFFI_Blob_Trim(this: MutNonNull<Blob>, size: usize) -> HResult;
}
