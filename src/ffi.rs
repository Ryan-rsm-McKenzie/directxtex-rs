use core::ptr::NonNull;

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
        rowPitch: NonNull<usize>,
        slicePitch: NonNull<usize>,
        flags: u32,
    ) -> u32;

    pub(crate) fn DirectXTexFFI_ComputeScanlines(fmt: u32, height: usize) -> usize;

    pub(crate) fn DirectXTexFFI_MakeSRGB(fmt: u32) -> u32;
    pub(crate) fn DirectXTexFFI_MakeLinear(fmt: u32) -> u32;
    pub(crate) fn DirectXTexFFI_MakeTypeless(fmt: u32) -> u32;
    pub(crate) fn DirectXTexFFI_MakeTypelessUNORM(fmt: u32) -> u32;
    pub(crate) fn DirectXTexFFI_MakeTypelessFLOAT(fmt: u32) -> u32;
}
