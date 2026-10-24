#if !defined(_WIN32)
#error "This file should only be compiled on Windows."
#endif

#include <cassert>

#include "DirectXTex.h"
#include <d3d11.h>

#define FFI(function) DirectXTexFFI_##function

extern "C"
{	
    //---------------------------------------------------------------------------------
	// D3D11 Compression functions

    HRESULT FFI(CompressD3D11_1)(
		ID3D11Device* pDevice,
		const DirectX::Image* srcImage,
		DXGI_FORMAT format,
		DirectX::TEX_COMPRESS_FLAGS compress,
		float threshold,
		DirectX::ScratchImage* cImage) noexcept
	{
        assert(pDevice != nullptr);
		assert(srcImage != nullptr);
		assert(cImage != nullptr);
		return DirectX::Compress(pDevice, *srcImage, format, compress, threshold, *cImage);
	}

	HRESULT FFI(CompressD3D11_2)(
		ID3D11Device* pDevice,
		const DirectX::Image* srcImages,
		size_t nimages,
		const DirectX::TexMetadata* metadata,
		DXGI_FORMAT format,
		DirectX::TEX_COMPRESS_FLAGS compress,
		float threshold,
		DirectX::ScratchImage* cImages) noexcept
	{
        assert(pDevice != nullptr);
		assert(metadata != nullptr);
		assert(cImages != nullptr);
		return DirectX::Compress(pDevice, srcImages, nimages, *metadata, format, compress, threshold, *cImages);
	}
}