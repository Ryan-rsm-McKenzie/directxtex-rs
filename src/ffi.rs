use crate::{
    DDSMetaData, HResult, TexMetadata, CP_FLAGS, DDS_FLAGS, DXGI_FORMAT, FORMAT_TYPE, TGA_FLAGS,
};
use core::ptr::NonNull;

#[repr(transparent)]
pub struct ConstPtr<T>(NonNull<T>);

impl<T> From<&T> for ConstPtr<T> {
    fn from(value: &T) -> Self {
        Self(value.into())
    }
}

#[repr(transparent)]
pub struct MutPtr<T>(NonNull<T>);

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
}
