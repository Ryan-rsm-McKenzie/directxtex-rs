use crate::{
    DDSMetaData, HResult, Image, ScratchImageFFI, TexMetadata, CP_FLAGS, DDS_FLAGS, DXGI_FORMAT,
    FORMAT_TYPE, TGA_FLAGS,
};
use core::ptr::NonNull;

#[repr(transparent)]
pub(crate) struct ConstPtr<T>(NonNull<T>);

impl<T> ConstPtr<T> {
    pub(crate) unsafe fn as_ref<'a>(&self) -> &'a T {
        self.0.as_ref()
    }

    pub(crate) fn new(value: NonNull<T>) -> Self {
        Self(value)
    }
}

impl<T> From<&T> for ConstPtr<T> {
    fn from(value: &T) -> Self {
        Self(value.into())
    }
}

#[repr(transparent)]
pub(crate) struct MutPtr<T>(NonNull<T>);

impl<T> MutPtr<T> {
    pub(crate) fn new(value: NonNull<T>) -> Self {
        Self(value)
    }
}

impl<T> From<&mut T> for MutPtr<T> {
    fn from(value: &mut T) -> Self {
        Self(value.into())
    }
}

#[allow(non_snake_case)]
#[link(name = "directxtex-ffi")]
extern "C" {
    //---------------------------------------------------------------------------------
    // DXGI Format Utilities

    pub(crate) fn DirectXTexFFI_IsValid(fmt: DXGI_FORMAT) -> bool;
    pub(crate) fn DirectXTexFFI_IsCompressed(fmt: DXGI_FORMAT) -> bool;
    pub(crate) fn DirectXTexFFI_IsPacked(fmt: DXGI_FORMAT) -> bool;
    pub(crate) fn DirectXTexFFI_IsVideo(fmt: DXGI_FORMAT) -> bool;
    pub(crate) fn DirectXTexFFI_IsPlanar(fmt: DXGI_FORMAT) -> bool;
    pub(crate) fn DirectXTexFFI_IsPalettized(fmt: DXGI_FORMAT) -> bool;
    pub(crate) fn DirectXTexFFI_IsDepthStencil(fmt: DXGI_FORMAT) -> bool;
    pub(crate) fn DirectXTexFFI_IsSRGB(fmt: DXGI_FORMAT) -> bool;
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
        rowPitch: MutPtr<usize>,
        slicePitch: MutPtr<usize>,
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

    pub(crate) fn DirectXTexFFI_TexMetadata_ComputIndex(
        this: ConstPtr<TexMetadata>,
        mip: usize,
        item: usize,
        slice: usize,
    ) -> usize;

    pub(crate) fn DirectXTexFFI_GetMetadataFromDDSMemoryEx(
        pSource: *const u8,
        size: usize,
        flags: DDS_FLAGS,
        metadata: MutPtr<TexMetadata>,
        ddPixelFormat: *mut DDSMetaData,
    ) -> HResult;

    pub(crate) fn DirectXTexFFI_GetMetadataFromHDRMemory(
        pSource: *const u8,
        size: usize,
        metadata: MutPtr<TexMetadata>,
    ) -> HResult;

    pub(crate) fn DirectXTexFFI_GetMetadataFromTGAMemory(
        pSource: *const u8,
        size: usize,
        flags: TGA_FLAGS,
        metadata: MutPtr<TexMetadata>,
    ) -> HResult;

    //---------------------------------------------------------------------------------
    // Bitmap image container

    pub(crate) fn DirectXTexFFI_ScratchImage_Ctor() -> *mut ScratchImageFFI;
    pub(crate) fn DirectXTexFFI_ScratchImage_Dtor(this: MutPtr<ScratchImageFFI>);

    pub(crate) fn DirectXTexFFI_ScratchImage_Initialize(
        this: MutPtr<ScratchImageFFI>,
        mdata: ConstPtr<TexMetadata>,
        flags: CP_FLAGS,
    ) -> HResult;

    pub(crate) fn DirectXTexFFI_ScratchImage_Initialize1D(
        this: MutPtr<ScratchImageFFI>,
        fmt: DXGI_FORMAT,
        length: usize,
        arraySize: usize,
        mipLevels: usize,
        flags: CP_FLAGS,
    ) -> HResult;
    pub(crate) fn DirectXTexFFI_ScratchImage_Initialize2D(
        this: MutPtr<ScratchImageFFI>,
        fmt: DXGI_FORMAT,
        width: usize,
        height: usize,
        arraySize: usize,
        mipLevels: usize,
        flags: CP_FLAGS,
    ) -> HResult;
    pub(crate) fn DirectXTexFFI_ScratchImage_Initialize3D(
        this: MutPtr<ScratchImageFFI>,
        fmt: DXGI_FORMAT,
        width: usize,
        height: usize,
        depth: usize,
        mipLevels: usize,
        flags: CP_FLAGS,
    ) -> HResult;
    pub(crate) fn DirectXTexFFI_ScratchImage_InitializeCube(
        this: MutPtr<ScratchImageFFI>,
        fmt: DXGI_FORMAT,
        width: usize,
        height: usize,
        nCubes: usize,
        mipLevels: usize,
        flags: CP_FLAGS,
    ) -> HResult;

    pub(crate) fn DirectXTexFFI_ScratchImage_InitializeFromImage(
        this: MutPtr<ScratchImageFFI>,
        srcImage: ConstPtr<Image>,
        allow1D: bool,
        flags: CP_FLAGS,
    ) -> HResult;
    pub(crate) fn DirectXTexFFI_ScratchImage_InitializeArrayFromImages(
        this: MutPtr<ScratchImageFFI>,
        images: *const Image,
        nImages: usize,
        allow1D: bool,
        flags: CP_FLAGS,
    ) -> HResult;
    pub(crate) fn DirectXTexFFI_ScratchImage_InitializeCubeFromImages(
        this: MutPtr<ScratchImageFFI>,
        images: *const Image,
        nImages: usize,
        flags: CP_FLAGS,
    ) -> HResult;
    pub(crate) fn DirectXTexFFI_ScratchImage_Initialize3DFromImages(
        this: MutPtr<ScratchImageFFI>,
        images: *const Image,
        depth: usize,
        flags: CP_FLAGS,
    ) -> HResult;

    pub(crate) fn DirectXTexFFI_ScratchImage_Release(this: MutPtr<ScratchImageFFI>);

    pub(crate) fn DirectXTexFFI_ScratchImage_OverrideFormat(
        this: MutPtr<ScratchImageFFI>,
        f: DXGI_FORMAT,
    ) -> bool;

    pub(crate) fn DirectXTexFFI_ScratchImage_GetMetadata(
        this: ConstPtr<ScratchImageFFI>,
    ) -> ConstPtr<TexMetadata>;
    pub(crate) fn DirectXTexFFI_ScratchImage_GetImage(
        this: ConstPtr<ScratchImageFFI>,
        mip: usize,
        item: usize,
        slice: usize,
    ) -> *const Image;

    pub(crate) fn DirectXTexFFI_ScratchImage_GetImages(
        this: ConstPtr<ScratchImageFFI>,
    ) -> *const Image;
    pub(crate) fn DirectXTexFFI_ScratchImage_GetImageCount(
        this: ConstPtr<ScratchImageFFI>,
    ) -> usize;

    pub(crate) fn DirectXTexFFI_ScratchImage_GetPixels(this: ConstPtr<ScratchImageFFI>) -> *mut u8;
    pub(crate) fn DirectXTexFFI_ScratchImage_GetPixelsSize(
        this: ConstPtr<ScratchImageFFI>,
    ) -> usize;

    pub(crate) fn DirectXTexFFI_ScratchImage_IsAlphaAllOpaque(
        this: ConstPtr<ScratchImageFFI>,
    ) -> bool;
}
