#include <cassert>
#include <climits>
#include <functional>
#include <new>
#include <stdint.h>
#include <type_traits>

#include "DirectXTex.h"

#define FFI(function) DirectXTexFFI_##function

static_assert(CHAR_BIT == 8);

template <class T, size_t BYTES>
static constexpr bool is_abi_compatible_v = sizeof(T) == BYTES && (std::is_integral_v<T> || std::is_enum_v<T>);

static_assert(is_abi_compatible_v<DirectX::WICCodecs, 4>);
static_assert(is_abi_compatible_v<DirectX::CMSE_FLAGS, 4>);
static_assert(is_abi_compatible_v<DirectX::CNMAP_FLAGS, 4>);
static_assert(is_abi_compatible_v<DirectX::CP_FLAGS, 4>);
static_assert(is_abi_compatible_v<DirectX::CREATETEX_FLAGS, 4>);
static_assert(is_abi_compatible_v<DirectX::DDS_FLAGS, 4>);
static_assert(is_abi_compatible_v<DirectX::FORMAT_TYPE, 4>);
static_assert(is_abi_compatible_v<DirectX::TEX_ALPHA_MODE, 4>);
static_assert(is_abi_compatible_v<DirectX::TEX_COMPRESS_FLAGS, 4>);
static_assert(is_abi_compatible_v<DirectX::TEX_DIMENSION, 4>);
static_assert(is_abi_compatible_v<DirectX::TEX_FILTER_FLAGS, 4>);
static_assert(is_abi_compatible_v<DirectX::TEX_FR_FLAGS, 4>);
static_assert(is_abi_compatible_v<DirectX::TEX_MISC_FLAG, 4>);
static_assert(is_abi_compatible_v<DirectX::TEX_MISC_FLAG2, 4>);
static_assert(is_abi_compatible_v<DirectX::TEX_PMALPHA_FLAGS, 4>);
static_assert(is_abi_compatible_v<DirectX::TGA_FLAGS, 4>);
static_assert(is_abi_compatible_v<DirectX::WIC_FLAGS, 4>);
static_assert(is_abi_compatible_v<DXGI_FORMAT, 4>);
static_assert(is_abi_compatible_v<HRESULT, 4>);

static_assert(sizeof(float) == 4);

extern "C"
{
	//---------------------------------------------------------------------------------
	// DXGI Format Utilities

	bool FFI(IsPacked)(
		DXGI_FORMAT fmt) noexcept
	{
		return DirectX::IsPacked(fmt);
	}

	bool FFI(IsVideo)(
		DXGI_FORMAT fmt) noexcept
	{
		return DirectX::IsVideo(fmt);
	}

	bool FFI(IsPlanar)(
		DXGI_FORMAT fmt) noexcept
	{
		return DirectX::IsPlanar(fmt);
	}

	bool FFI(IsDepthStencil)(
		DXGI_FORMAT fmt) noexcept
	{
		return DirectX::IsDepthStencil(fmt);
	}

	bool FFI(IsBGR)(
		DXGI_FORMAT fmt) noexcept
	{
		return DirectX::IsBGR(fmt);
	}

	bool FFI(IsTypeless)(
		DXGI_FORMAT fmt,
		bool partialTypeless) noexcept
	{
		return DirectX::IsTypeless(
			fmt,
			partialTypeless);
	}

	bool FFI(HasAlpha)(
		DXGI_FORMAT fmt) noexcept
	{
		return DirectX::HasAlpha(fmt);
	}

	size_t FFI(BitsPerPixel)(
		DXGI_FORMAT fmt) noexcept
	{
		return DirectX::BitsPerPixel(fmt);
	}

	size_t FFI(BitsPerColor)(
		DXGI_FORMAT fmt) noexcept
	{
		return DirectX::BitsPerColor(fmt);
	}

	DirectX::FORMAT_TYPE FFI(FormatDataType)(
		DXGI_FORMAT fmt) noexcept
	{
		return DirectX::FormatDataType(fmt);
	}

	HRESULT FFI(ComputePitch)(
		DXGI_FORMAT fmt,
		size_t width,
		size_t height,
		size_t* rowPitch,
		size_t* slicePitch,
		DirectX::CP_FLAGS flags) noexcept
	{
		assert(rowPitch != nullptr);
		assert(slicePitch != nullptr);
		return DirectX::ComputePitch(
			fmt,
			width,
			height,
			*rowPitch,
			*slicePitch,
			flags);
	}

	size_t FFI(ComputeScanlines)(
		DXGI_FORMAT fmt,
		size_t height) noexcept
	{
		return DirectX::ComputeScanlines(
			fmt,
			height);
	}

	DXGI_FORMAT FFI(MakeSRGB)(
		DXGI_FORMAT fmt) noexcept
	{
		return DirectX::MakeSRGB(fmt);
	}

	DXGI_FORMAT FFI(MakeLinear)(
		DXGI_FORMAT fmt) noexcept
	{
		return DirectX::MakeLinear(fmt);
	}

	DXGI_FORMAT FFI(MakeTypeless)(
		DXGI_FORMAT fmt) noexcept
	{
		return DirectX::MakeTypeless(fmt);
	}

	DXGI_FORMAT FFI(MakeTypelessUNORM)(
		DXGI_FORMAT fmt) noexcept
	{
		return DirectX::MakeTypelessUNORM(fmt);
	}

	DXGI_FORMAT FFI(MakeTypelessFLOAT)(
		DXGI_FORMAT fmt) noexcept
	{
		return DirectX::MakeTypelessFLOAT(fmt);
	}

	//---------------------------------------------------------------------------------
	// Texture metadata

	size_t FFI(TexMetadata_Sizeof)() noexcept
	{
		return sizeof(DirectX::TexMetadata);
	}

	size_t FFI(TexMetadata_Alignof)() noexcept
	{
		return alignof(DirectX::TexMetadata);
	}

	size_t FFI(DDSMetaData_Sizeof)() noexcept
	{
		return sizeof(DirectX::DDSMetaData);
	}

	size_t FFI(DDSMetaData_Alignof)() noexcept
	{
		return alignof(DirectX::DDSMetaData);
	}

	// struct TexMetadata {

	size_t FFI(TexMetadata_ComputIndex)(
		const DirectX::TexMetadata* self,
		size_t mip,
		size_t item,
		size_t slice) noexcept
	{
		assert(self != nullptr);
		return self->ComputeIndex(
			mip,
			item,
			slice);
	}

	// }; struct TexMetadata

	HRESULT FFI(GetMetadataFromDDSMemoryEx)(
		const uint8_t* pSource,
		size_t size,
		DirectX::DDS_FLAGS flags,
		DirectX::TexMetadata* metadata,
		DirectX::DDSMetaData* ddPixelFormat) noexcept
	{
		assert(metadata != nullptr);
		return DirectX::GetMetadataFromDDSMemoryEx(
			pSource,
			size,
			flags,
			*metadata,
			ddPixelFormat);
	}

	HRESULT FFI(GetMetadataFromHDRMemory)(
		const uint8_t* pSource,
		size_t size,
		DirectX::TexMetadata* metadata) noexcept
	{
		assert(metadata != nullptr);
		return DirectX::GetMetadataFromHDRMemory(
			pSource,
			size,
			*metadata);
	}

	HRESULT FFI(GetMetadataFromTGAMemory)(
		const uint8_t* pSource,
		size_t size,
		DirectX::TGA_FLAGS flags,
		DirectX::TexMetadata* metadata) noexcept
	{
		assert(metadata != nullptr);
		return DirectX::GetMetadataFromTGAMemory(
			pSource,
			size,
			flags,
			*metadata);
	}

	//---------------------------------------------------------------------------------
	// Bitmap image container

	size_t FFI(Image_Sizeof)() noexcept
	{
		return sizeof(DirectX::Image);
	}

	size_t FFI(Image_Alignof)() noexcept
	{
		return alignof(DirectX::Image);
	}

	size_t FFI(ScratchImage_Sizeof)() noexcept
	{
		return sizeof(DirectX::ScratchImage);
	}

	size_t FFI(ScratchImage_Alignof)() noexcept
	{
		return alignof(DirectX::ScratchImage);
	}

	// class ScratchImage {

	HRESULT FFI(ScratchImage_Initialize)(
		DirectX::ScratchImage* self,
		const DirectX::TexMetadata* mdata,
		DirectX::CP_FLAGS flags) noexcept
	{
		assert(self != nullptr);
		assert(mdata != nullptr);
		return self->Initialize(*mdata, flags);
	}

	HRESULT FFI(ScratchImage_Initialize1D)(
		DirectX::ScratchImage* self,
		DXGI_FORMAT fmt,
		size_t length,
		size_t arraySize,
		size_t mipLevels,
		DirectX::CP_FLAGS flags) noexcept
	{
		assert(self != nullptr);
		return self->Initialize1D(fmt, length, arraySize, mipLevels, flags);
	}

	HRESULT FFI(ScratchImage_Initialize2D)(
		DirectX::ScratchImage* self,
		DXGI_FORMAT fmt,
		size_t width,
		size_t height,
		size_t arraySize,
		size_t mipLevels,
		DirectX::CP_FLAGS flags) noexcept
	{
		assert(self != nullptr);
		return self->Initialize2D(fmt, width, height, arraySize, mipLevels, flags);
	}

	HRESULT FFI(ScratchImage_Initialize3D)(
		DirectX::ScratchImage* self,
		DXGI_FORMAT fmt,
		size_t width,
		size_t height,
		size_t depth,
		size_t mipLevels,
		DirectX::CP_FLAGS flags) noexcept
	{
		assert(self != nullptr);
		return self->Initialize3D(fmt, width, height, depth, mipLevels, flags);
	}

	HRESULT FFI(ScratchImage_InitializeCube)(
		DirectX::ScratchImage* self,
		DXGI_FORMAT fmt,
		size_t width,
		size_t height,
		size_t nCubes,
		size_t mipLevels,
		DirectX::CP_FLAGS flags) noexcept
	{
		assert(self != nullptr);
		return self->InitializeCube(fmt, width, height, nCubes, mipLevels, flags);
	}

	HRESULT FFI(ScratchImage_InitializeFromImage)(
		DirectX::ScratchImage* self,
		const DirectX::Image* srcImage,
		bool allow1D,
		DirectX::CP_FLAGS flags) noexcept
	{
		assert(self != nullptr);
		assert(srcImage != nullptr);
		return self->InitializeFromImage(*srcImage, allow1D, flags);
	}

	HRESULT FFI(ScratchImage_InitializeArrayFromImages)(
		DirectX::ScratchImage* self,
		const DirectX::Image* images,
		size_t nImages,
		bool allow1D,
		DirectX::CP_FLAGS flags) noexcept
	{
		assert(self != nullptr);
		return self->InitializeArrayFromImages(images, nImages, allow1D, flags);
	}

	HRESULT FFI(ScratchImage_InitializeCubeFromImages)(
		DirectX::ScratchImage* self,
		const DirectX::Image* images,
		size_t nImages,
		DirectX::CP_FLAGS flags) noexcept
	{
		assert(self != nullptr);
		return self->InitializeCubeFromImages(images, nImages, flags);
	}

	HRESULT FFI(ScratchImage_Initialize3DFromImages)(
		DirectX::ScratchImage* self,
		const DirectX::Image* images,
		size_t depth,
		DirectX::CP_FLAGS flags) noexcept
	{
		assert(self != nullptr);
		return self->Initialize3DFromImages(images, depth, flags);
	}

	void FFI(ScratchImage_Release)(
		DirectX::ScratchImage* self) noexcept
	{
		assert(self != nullptr);
		return self->Release();
	}

	bool FFI(ScratchImage_OverrideFormat)(
		DirectX::ScratchImage* self,
		DXGI_FORMAT f) noexcept
	{
		assert(self != nullptr);
		return self->OverrideFormat(f);
	}

	const DirectX::Image* FFI(ScratchImage_GetImage)(
		const DirectX::ScratchImage* self,
		size_t mip,
		size_t item,
		size_t slice) noexcept
	{
		assert(self != nullptr);
		return self->GetImage(mip, item, slice);
	}

	bool FFI(ScratchImage_IsAlphaAllOpaque)(
		const DirectX::ScratchImage* self) noexcept
	{
		assert(self != nullptr);
		return self->IsAlphaAllOpaque();
	}

	// }; class ScratchImage

	//---------------------------------------------------------------------------------
	// Memory blob (allocated buffer pointer is always 16-byte aligned)

	// class Blob {

	size_t FFI(Blob_Sizeof)() noexcept
	{
		return sizeof(DirectX::Blob);
	}

	size_t FFI(Blob_Alignof)() noexcept
	{
		return alignof(DirectX::Blob);
	}

	HRESULT FFI(Blob_Initialize)(
		DirectX::Blob* self,
		size_t size) noexcept
	{
		assert(self != nullptr);
		return self->Initialize(size);
	}

	void FFI(Blob_Release)(
		DirectX::Blob* self) noexcept
	{
		assert(self != nullptr);
		return self->Release();
	}

	HRESULT FFI(Blob_Resize)(
		DirectX::Blob* self,
		size_t size) noexcept
	{
		assert(self != nullptr);
		return self->Resize(size);
	}

	HRESULT FFI(Blob_Trim)(
		DirectX::Blob* self,
		size_t size) noexcept
	{
		assert(self != nullptr);
		return self->Trim(size);
	}

	// }; class Blob

	//---------------------------------------------------------------------------------
	// Image I/O

	// DDS operations
	HRESULT FFI(LoadFromDDSMemoryEx)(
		const uint8_t* pSource,
		size_t size,
		DirectX::DDS_FLAGS flags,
		DirectX::TexMetadata* metadata,
		DirectX::DDSMetaData* ddPixelFormat,
		DirectX::ScratchImage* image) noexcept
	{
		assert(image != nullptr);
		return DirectX::LoadFromDDSMemoryEx(pSource, size, flags, metadata, ddPixelFormat, *image);
	}

	HRESULT FFI(SaveToDDSMemory1)(
		const DirectX::Image* image,
		DirectX::DDS_FLAGS flags,
		DirectX::Blob* blob) noexcept
	{
		assert(image != nullptr);
		assert(blob != nullptr);
		return DirectX::SaveToDDSMemory(*image, flags, *blob);
	}

	HRESULT FFI(SaveToDDSMemory2)(
		const DirectX::Image* images,
		size_t nimages,
		const DirectX::TexMetadata* metadata,
		DirectX::DDS_FLAGS flags,
		DirectX::Blob* blob) noexcept
	{
		assert(metadata != nullptr);
		assert(blob != nullptr);
		return DirectX::SaveToDDSMemory(images, nimages, *metadata, flags, *blob);
	}

	HRESULT FFI(LoadFromHDRMemory)(
		const uint8_t* pSource,
		size_t size,
		DirectX::TexMetadata* metadata,
		DirectX::ScratchImage* image) noexcept
	{
		assert(image != nullptr);
		return DirectX::LoadFromHDRMemory(pSource, size, metadata, *image);
	}

	HRESULT FFI(SaveToHDRMemory)(
		const DirectX::Image* image,
		DirectX::Blob* blob) noexcept
	{
		assert(image != nullptr);
		assert(blob != nullptr);
		return DirectX::SaveToHDRMemory(*image, *blob);
	}

	// TGA operations
	HRESULT FFI(LoadFromTGAMemory)(
		const uint8_t* pSource,
		size_t size,
		DirectX::TGA_FLAGS flags,
		DirectX::TexMetadata* metadata,
		DirectX::ScratchImage* image) noexcept
	{
		assert(image != nullptr);
		return DirectX::LoadFromTGAMemory(pSource, size, flags, metadata, *image);
	}

	HRESULT FFI(SaveToTGAMemory)(
		const DirectX::Image* image,
		DirectX::TGA_FLAGS flags,
		DirectX::Blob* blob,
		const DirectX::TexMetadata* metadata) noexcept
	{
		assert(image != nullptr);
		assert(blob != nullptr);
		return DirectX::SaveToTGAMemory(*image, flags, *blob, metadata);
	}

	//---------------------------------------------------------------------------------
	// Texture conversion, resizing, mipmap generation, and block compression

	HRESULT FFI(Resize1)(
		const DirectX::Image* srcImage,
		size_t width,
		size_t height,
		DirectX::TEX_FILTER_FLAGS filter,
		DirectX::ScratchImage* image) noexcept
	{
		assert(srcImage != nullptr);
		assert(image != nullptr);
		return DirectX::Resize(*srcImage, width, height, filter, *image);
	}

	HRESULT FFI(Resize2)(
		const DirectX::Image* srcImages,
		size_t nimages,
		const DirectX::TexMetadata* metadata,
		size_t width,
		size_t height,
		DirectX::TEX_FILTER_FLAGS filter,
		DirectX::ScratchImage* result) noexcept
	{
		assert(metadata != nullptr);
		assert(result != nullptr);
		return DirectX::Resize(srcImages, nimages, *metadata, width, height, filter, *result);
	}

	HRESULT FFI(Convert1)(
		const DirectX::Image* srcImage,
		DXGI_FORMAT format,
		DirectX::TEX_FILTER_FLAGS filter,
		float threshold,
		DirectX::ScratchImage* image) noexcept
	{
		assert(srcImage != nullptr);
		assert(image != nullptr);
		return DirectX::Convert(*srcImage, format, filter, threshold, *image);
	}

	HRESULT FFI(Convert2)(
		const DirectX::Image* srcImages,
		size_t nimages,
		const DirectX::TexMetadata* metadata,
		DXGI_FORMAT format,
		DirectX::TEX_FILTER_FLAGS filter,
		float threshold,
		DirectX::ScratchImage* result) noexcept
	{
		assert(metadata != nullptr);
		assert(result != nullptr);
		return DirectX::Convert(srcImages, nimages, *metadata, format, filter, threshold, *result);
	}

	HRESULT FFI(ConvertToSinglePlane1)(
		const DirectX::Image* srcImage,
		DirectX::ScratchImage* image) noexcept
	{
		assert(srcImage != nullptr);
		assert(image != nullptr);
		return DirectX::ConvertToSinglePlane(*srcImage, *image);
	}

	HRESULT FFI(ConvertToSinglePlane2)(
		const DirectX::Image* srcImages,
		size_t nimages,
		const DirectX::TexMetadata* metadata,
		DirectX::ScratchImage* image) noexcept
	{
		assert(metadata != nullptr);
		assert(image != nullptr);
		return DirectX::ConvertToSinglePlane(srcImages, nimages, *metadata, *image);
	}

	HRESULT FFI(GenerateMipMaps1)(
		const DirectX::Image* baseImage,
		DirectX::TEX_FILTER_FLAGS filter,
		size_t levels,
		DirectX::ScratchImage* mipChain,
		bool allow1D) noexcept
	{
		assert(baseImage != nullptr);
		assert(mipChain != nullptr);
		return DirectX::GenerateMipMaps(*baseImage, filter, levels, *mipChain, allow1D);
	}

	HRESULT FFI(GenerateMipMaps2)(
		const DirectX::Image* srcImages,
		size_t nimages,
		const DirectX::TexMetadata* metadata,
		DirectX::TEX_FILTER_FLAGS filter,
		size_t levels,
		DirectX::ScratchImage* mipChain) noexcept
	{
		assert(metadata != nullptr);
		assert(mipChain != nullptr);
		return DirectX::GenerateMipMaps(srcImages, nimages, *metadata, filter, levels, *mipChain);
	}

	HRESULT FFI(GenerateMipMaps3D1)(
		const DirectX::Image* baseImages,
		size_t depth,
		DirectX::TEX_FILTER_FLAGS filter,
		size_t levels,
		DirectX::ScratchImage* mipChain) noexcept
	{
		assert(mipChain != nullptr);
		return DirectX::GenerateMipMaps3D(baseImages, depth, filter, levels, *mipChain);
	}

	HRESULT FFI(GenerateMipMaps3D2)(
		const DirectX::Image* srcImages,
		size_t nimages,
		const DirectX::TexMetadata* metadata,
		DirectX::TEX_FILTER_FLAGS filter,
		size_t levels,
		DirectX::ScratchImage* mipChain) noexcept
	{
		assert(metadata != nullptr);
		assert(mipChain != nullptr);
		return DirectX::GenerateMipMaps3D(srcImages, nimages, *metadata, filter, levels, *mipChain);
	}

	HRESULT FFI(ScaleMipMapsAlphaForCoverage)(
		const DirectX::Image* srcImages,
		size_t nimages,
		const DirectX::TexMetadata* metadata,
		size_t item,
		float alphaReference,
		DirectX::ScratchImage* mipChain) noexcept
	{
		assert(metadata != nullptr);
		assert(mipChain != nullptr);
		return DirectX::ScaleMipMapsAlphaForCoverage(srcImages, nimages, *metadata, item, alphaReference, *mipChain);
	}

	HRESULT FFI(PremultiplyAlpha1)(
		const DirectX::Image* srcImage,
		DirectX::TEX_PMALPHA_FLAGS flags,
		DirectX::ScratchImage* image) noexcept
	{
		assert(srcImage != nullptr);
		assert(image != nullptr);
		return DirectX::PremultiplyAlpha(*srcImage, flags, *image);
	}

	HRESULT FFI(PremultiplyAlpha2)(
		const DirectX::Image* srcImages,
		size_t nimages,
		const DirectX::TexMetadata* metadata,
		DirectX::TEX_PMALPHA_FLAGS flags,
		DirectX::ScratchImage* result) noexcept
	{
		assert(metadata != nullptr);
		assert(result != nullptr);
		return DirectX::PremultiplyAlpha(srcImages, nimages, *metadata, flags, *result);
	}

	HRESULT FFI(Compress1)(
		const DirectX::Image* srcImage,
		DXGI_FORMAT format,
		DirectX::TEX_COMPRESS_FLAGS compress,
		float threshold,
		DirectX::ScratchImage* cImage) noexcept
	{
		assert(srcImage != nullptr);
		assert(cImage != nullptr);
		return DirectX::Compress(*srcImage, format, compress, threshold, *cImage);
	}

	HRESULT FFI(Compress2)(
		const DirectX::Image* srcImages,
		size_t nimages,
		const DirectX::TexMetadata* metadata,
		DXGI_FORMAT format,
		DirectX::TEX_COMPRESS_FLAGS compress,
		float threshold,
		DirectX::ScratchImage* cImages) noexcept
	{
		assert(metadata != nullptr);
		assert(cImages != nullptr);
		return DirectX::Compress(srcImages, nimages, *metadata, format, compress, threshold, *cImages);
	}

	HRESULT FFI(Decompress1)(
		const DirectX::Image* cImage,
		DXGI_FORMAT format,
		DirectX::ScratchImage* image) noexcept
	{
		assert(cImage != nullptr);
		assert(image != nullptr);
		return DirectX::Decompress(*cImage, format, *image);
	}

	HRESULT FFI(Decompress2)(
		const DirectX::Image* cImages,
		size_t nimages,
		const DirectX::TexMetadata* metadata,
		DXGI_FORMAT format,
		DirectX::ScratchImage* images) noexcept
	{
		assert(metadata != nullptr);
		assert(images != nullptr);
		return DirectX::Decompress(cImages, nimages, *metadata, format, *images);
	}

	//---------------------------------------------------------------------------------
	// Normal map operations

	HRESULT FFI(ComputeNormalMap1)(
		const DirectX::Image* srcImage,
		DirectX::CNMAP_FLAGS flags,
		float amplitude,
		DXGI_FORMAT format,
		DirectX::ScratchImage* normalMap) noexcept
	{
		assert(srcImage != nullptr);
		assert(normalMap != nullptr);
		return DirectX::ComputeNormalMap(*srcImage, flags, amplitude, format, *normalMap);
	}

	HRESULT FFI(ComputeNormalMap2)(
		const DirectX::Image* srcImages,
		size_t nimages,
		const DirectX::TexMetadata* metadata,
		DirectX::CNMAP_FLAGS flags,
		float amplitude,
		DXGI_FORMAT format,
		DirectX::ScratchImage* normalMaps) noexcept
	{
		assert(metadata != nullptr);
		assert(normalMaps != nullptr);
		return DirectX::ComputeNormalMap(srcImages, nimages, *metadata, flags, amplitude, format, *normalMaps);
	}

	//---------------------------------------------------------------------------------
	// Misc image operations

	HRESULT FFI(CopyRectangle)(
		const DirectX::Image* srcImage,
		const DirectX::Rect* srcRect,
		const DirectX::Image* dstImage,
		DirectX::TEX_FILTER_FLAGS filter,
		size_t xOffset,
		size_t yOffset) noexcept
	{
		assert(srcImage != nullptr);
		assert(srcRect != nullptr);
		assert(dstImage != nullptr);
		return DirectX::CopyRectangle(*srcImage, *srcRect, *dstImage, filter, xOffset, yOffset);
	}

	HRESULT FFI(ComputeMSE)(
		const DirectX::Image* image1,
		const DirectX::Image* image2,
		float* mse,
		float* mseV,
		DirectX::CMSE_FLAGS flags) noexcept
	{
		assert(image1 != nullptr);
		assert(image2 != nullptr);
		assert(mse != nullptr);
		return DirectX::ComputeMSE(*image1, *image2, *mse, mseV, flags);
	}

	HRESULT FFI(EvaluateImage1)(
		const DirectX::Image* image,
		void (*pixelFunc)(const DirectX::XMVECTOR*, size_t, size_t)) noexcept
	{
		assert(image != nullptr);
		assert(pixelFunc != nullptr);
		return DirectX::EvaluateImage(*image, pixelFunc);
	}

	HRESULT FFI(EvaluateImage2)(
		const DirectX::Image* images,
		size_t nimages,
		const DirectX::TexMetadata* metadata,
		void (*pixelFunc)(const DirectX::XMVECTOR*, size_t, size_t)) noexcept
	{
		assert(metadata != nullptr);
		assert(pixelFunc != nullptr);
		return DirectX::EvaluateImage(images, nimages, *metadata, pixelFunc);
	}

	HRESULT FFI(TransformImage1)(
		const DirectX::Image* image,
		void (*pixelFunc)(const DirectX::XMVECTOR*, const DirectX::XMVECTOR*, size_t, size_t),
		DirectX::ScratchImage* result) noexcept
	{
		assert(image != nullptr);
		assert(pixelFunc != nullptr);
		assert(result != nullptr);
		return DirectX::TransformImage(*image, pixelFunc, *result);
	}

	HRESULT FFI(TransformImage2)(
		const DirectX::Image* srcImages,
		size_t nimages,
		const DirectX::TexMetadata* metadata,
		void (*pixelFunc)(const DirectX::XMVECTOR*, const DirectX::XMVECTOR*, size_t, size_t),
		DirectX::ScratchImage* result) noexcept
	{
		assert(metadata != nullptr);
		assert(pixelFunc != nullptr);
		assert(result != nullptr);
		return DirectX::TransformImage(srcImages, nimages, *metadata, pixelFunc, *result);
	}

	//---------------------------------------------------------------------------------
	// WIC utility code
#ifdef _WIN32
	const GUID* FFI(GetWICCodec)(
		DirectX::WICCodecs codec) noexcept
	{
		return &DirectX::GetWICCodec(codec);
	}

	IWICImagingFactory* FFI(GetWICFactory)(
		bool* iswic2) noexcept
	{
		assert(iswic2 != nullptr);
		return DirectX::GetWICFactory(*iswic2);
	}

	void FFI(SetWICFactory)(
		IWICImagingFactory* pWIC) noexcept
	{
		return DirectX::SetWICFactory(pWIC);
	}
#endif

	//---------------------------------------------------------------------------------
	// DDS helper functions
	HRESULT FFI(EncodeDDSHeader)(
		const DirectX::TexMetadata* metadata,
		DirectX::DDS_FLAGS flags,
		void* pDestination,
		size_t maxsize,
		size_t* required) noexcept
	{
		assert(metadata != nullptr);
		assert(required != nullptr);
		return DirectX::EncodeDDSHeader(*metadata, flags, pDestination, maxsize, *required);
	}
}
