use crate::{DDSMetaData, TexMetadata};
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

    pub(crate) fn DirectXTexFFI_IsValid(fmt: u32) -> bool;
    pub(crate) fn DirectXTexFFI_IsCompressed(fmt: u32) -> bool;
    pub(crate) fn DirectXTexFFI_IsPacked(fmt: u32) -> bool;
    pub(crate) fn DirectXTexFFI_IsVideo(fmt: u32) -> bool;
    pub(crate) fn DirectXTexFFI_IsPlanar(fmt: u32) -> bool;
    pub(crate) fn DirectXTexFFI_IsPalettized(fmt: u32) -> bool;
    pub(crate) fn DirectXTexFFI_IsDepthStencil(fmt: u32) -> bool;
    pub(crate) fn DirectXTexFFI_IsSRGB(fmt: u32) -> bool;
    pub(crate) fn DirectXTexFFI_IsBGR(fmt: u32) -> bool;
    pub(crate) fn DirectXTexFFI_IsTypeless(fmt: u32, partialTypeless: bool) -> bool;

    pub(crate) fn DirectXTexFFI_HasAlpha(fmt: u32) -> bool;

    pub(crate) fn DirectXTexFFI_BitsPerPixel(fmt: u32) -> usize;

    pub(crate) fn DirectXTexFFI_BitsPerColor(fmt: u32) -> usize;

    pub(crate) fn DirectXTexFFI_FormatDataType(fmt: u32) -> u32;

    pub(crate) fn DirectXTexFFI_ComputePitch(
        fmt: u32,
        width: usize,
        height: usize,
        rowPitch: MutPtr<usize>,
        slicePitch: MutPtr<usize>,
        flags: u32,
    ) -> u32;

    pub(crate) fn DirectXTexFFI_ComputeScanlines(fmt: u32, height: usize) -> usize;

    pub(crate) fn DirectXTexFFI_MakeSRGB(fmt: u32) -> u32;
    pub(crate) fn DirectXTexFFI_MakeLinear(fmt: u32) -> u32;
    pub(crate) fn DirectXTexFFI_MakeTypeless(fmt: u32) -> u32;
    pub(crate) fn DirectXTexFFI_MakeTypelessUNORM(fmt: u32) -> u32;
    pub(crate) fn DirectXTexFFI_MakeTypelessFLOAT(fmt: u32) -> u32;

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
        flags: u32,
        metadata: MutPtr<TexMetadata>,
        ddPixelFormat: *mut DDSMetaData,
    ) -> u32;

    pub(crate) fn DirectXTexFFI_GetMetadataFromHDRMemory(
        pSource: *const u8,
        size: usize,
        metadata: MutPtr<TexMetadata>,
    ) -> u32;

    pub(crate) fn DirectXTexFFI_GetMetadataFromTGAMemory(
        pSource: *const u8,
        size: usize,
        flags: u32,
        metadata: MutPtr<TexMetadata>,
    ) -> u32;
}
