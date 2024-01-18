use crate::{CP_FLAGS, DXGI_FORMAT, FORMAT_TYPE, HRESULT};
use core::ptr::NonNull;

#[allow(non_snake_case)]
#[link(name = "directxtex-ffi")]
extern "C" {
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
        rowPitch: NonNull<usize>,
        slicePitch: NonNull<usize>,
        flags: CP_FLAGS,
    ) -> HRESULT;

    pub(crate) fn DirectXTexFFI_ComputeScanlines(fmt: DXGI_FORMAT, height: usize) -> usize;

    pub(crate) fn DirectXTexFFI_MakeSRGB(fmt: DXGI_FORMAT) -> DXGI_FORMAT;
    pub(crate) fn DirectXTexFFI_MakeLinear(fmt: DXGI_FORMAT) -> DXGI_FORMAT;
    pub(crate) fn DirectXTexFFI_MakeTypeless(fmt: DXGI_FORMAT) -> DXGI_FORMAT;
    pub(crate) fn DirectXTexFFI_MakeTypelessUNORM(fmt: DXGI_FORMAT) -> DXGI_FORMAT;
    pub(crate) fn DirectXTexFFI_MakeTypelessFLOAT(fmt: DXGI_FORMAT) -> DXGI_FORMAT;
}
