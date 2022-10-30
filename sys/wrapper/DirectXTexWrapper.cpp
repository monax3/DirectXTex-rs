#include "DirectXTexWrapper.hpp"
#include <DirectXTex.h>
#include <DirectXTexEXR.h>

namespace Wrapper
{

#ifdef _WIN32

    extern "C" HRESULT __cdecl GetMetadataFromWICMemory(
        _In_reads_bytes_(size) const void *pSource, _In_ size_t size,
        _In_ WIC_FLAGS flags,
        _Out_ TexMetadata &metadata,
        _In_opt_ GetMQR *getMQR, void *callback_fn)
    {
        if (getMQR == nullptr)
        {
            return DirectX::GetMetadataFromWICMemory(pSource, size, flags, metadata, nullptr);
        }
        else
        {
            return DirectX::GetMetadataFromWICMemory(pSource, size, flags, metadata, [&](IWICMetadataQueryReader *mqr)
                                                     { getMQR(mqr, callback_fn); });
        }
    }

    extern "C" HRESULT __cdecl GetMetadataFromWICFile(
        _In_z_ const wchar_t *szFile,
        _In_ WIC_FLAGS flags,
        _Out_ TexMetadata &metadata,
        _In_opt_ GetMQR *getMQR, void *callback_fn)
    {
        if (getMQR == nullptr)
        {
            return DirectX::GetMetadataFromWICFile(szFile, flags, metadata, nullptr);
        }
        else
        {
            return DirectX::GetMetadataFromWICFile(szFile, flags, metadata, [&](IWICMetadataQueryReader *mqr)
                                                   { getMQR(mqr, callback_fn); });
        }
    }

    extern "C" HRESULT __cdecl LoadFromWICMemory(
        _In_reads_bytes_(size) const void *pSource, _In_ size_t size,
        _In_ WIC_FLAGS flags,
        _Out_opt_ TexMetadata *metadata, _Out_ ScratchImage &image,
        _In_opt_ GetMQR *getMQR, void *callback_fn)
    {
        if (getMQR == nullptr)
        {
            return DirectX::LoadFromWICMemory(pSource, size, flags, metadata, image, nullptr);
        }
        else
        {
            return DirectX::LoadFromWICMemory(pSource, size, flags, metadata, image, [&](IWICMetadataQueryReader *mqr)
                                              { getMQR(mqr, callback_fn); });
        }
    }

    extern "C" HRESULT __cdecl LoadFromWICFile(
        _In_z_ const wchar_t *szFile, _In_ WIC_FLAGS flags,
        _Out_opt_ TexMetadata *metadata, _Out_ ScratchImage &image,
        _In_opt_ GetMQR *getMQR, void *callback_fn)
    {
        if (getMQR == nullptr)
        {
            return DirectX::LoadFromWICFile(szFile, flags, metadata, image, nullptr);
        }
        else
        {
            return DirectX::LoadFromWICFile(szFile, flags, metadata, image, [&](IWICMetadataQueryReader *mqr)
                                            { getMQR(mqr, callback_fn); });
        }
    }

    extern "C" HRESULT __cdecl SaveToWICMemory(
        _In_ const Image &image, _In_ WIC_FLAGS flags, _In_ REFGUID guidContainerFormat,
        _Out_ Blob &blob, _In_opt_ const GUID *targetFormat,
        _In_opt_ SetCustomProps *setCustomProps, void *callback_fn)
    {
        if (setCustomProps == nullptr)
        {
            return DirectX::SaveToWICMemory(image, flags, guidContainerFormat, blob, targetFormat, nullptr);
        }
        else
        {
            return DirectX::SaveToWICMemory(image, flags, guidContainerFormat, blob, targetFormat, [&](IPropertyBag2 *props)
                                            { setCustomProps(props, callback_fn); });
        }
    }

    extern "C" HRESULT __cdecl SaveToWICMemory1(
        _In_count_(nimages) const Image *images, _In_ size_t nimages,
        _In_ WIC_FLAGS flags, _In_ REFGUID guidContainerFormat,
        _Out_ Blob &blob, _In_opt_ const GUID *targetFormat,
        _In_opt_ SetCustomProps *setCustomProps, void *callback_fn)
    {
        if (setCustomProps == nullptr)
        {
            return DirectX::SaveToWICMemory(images, nimages, flags, guidContainerFormat, blob, targetFormat, nullptr);
        }
        else
        {
            return DirectX::SaveToWICMemory(images, nimages, flags, guidContainerFormat, blob, targetFormat, [&](IPropertyBag2 *props)
                                            { setCustomProps(props, callback_fn); });
        }
    }

    extern "C" HRESULT __cdecl SaveToWICFile(
        _In_ const Image &image, _In_ WIC_FLAGS flags, _In_ REFGUID guidContainerFormat,
        _In_z_ const wchar_t *szFile, _In_opt_ const GUID *targetFormat,
        _In_opt_ SetCustomProps *setCustomProps, void *callback_fn)
    {
        if (setCustomProps == nullptr)
        {
            return DirectX::SaveToWICFile(image, flags, guidContainerFormat, szFile, targetFormat, nullptr);
        }
        else
        {
            return DirectX::SaveToWICFile(image, flags, guidContainerFormat, szFile, targetFormat, [&](IPropertyBag2 *props)
                                          { setCustomProps(props, callback_fn); });
        }
    }

    extern "C" HRESULT __cdecl SaveToWICFile1(
        _In_count_(nimages) const Image *images, _In_ size_t nimages,
        _In_ WIC_FLAGS flags, _In_ REFGUID guidContainerFormat,
        _In_z_ const wchar_t *szFile, _In_ const GUID *targetFormat,
        _In_opt_ SetCustomProps *setCustomProps, void *callback_fn)
    {
        if (setCustomProps == nullptr)
        {
            return DirectX::SaveToWICFile(images, nimages, flags, guidContainerFormat, szFile, targetFormat, nullptr);
        }
        else
        {
            return DirectX::SaveToWICFile(images, nimages, flags, guidContainerFormat, szFile, targetFormat, [&](IPropertyBag2 *props)
                                          { setCustomProps(props, callback_fn); });
        }
    }

#endif /* ifdef _WIN32 */

    extern "C" HRESULT __cdecl EvaluateImage(
        _In_ const Image &image,
        _In_ EvaluateFunc *pixelFunc, void *callback_fn)
    {
        return DirectX::EvaluateImage(image, [&](const XMVECTOR *pixels, size_t width, size_t y)
                                      { pixelFunc(pixels, width, y, callback_fn); });
    }

    extern "C" HRESULT __cdecl EvaluateImage1(
        _In_reads_(nimages) const Image *images, _In_ size_t nimages, _In_ const TexMetadata &metadata,
        _In_ EvaluateFunc *pixelFunc, void *callback_fn)
    {
        return DirectX::EvaluateImage(images, nimages, metadata, [&](const XMVECTOR *pixels, size_t width, size_t y)
                                      { pixelFunc(pixels, width, y, callback_fn); });
    }

    extern "C" HRESULT __cdecl TransformImage(
        _In_ const Image &image,
        _In_ TransformFunc *pixelFunc, void *callback_fn,
        ScratchImage &result)
    {
        return DirectX::TransformImage(
            image, [&](XMVECTOR *outPixels, const XMVECTOR *inPixels, size_t width, size_t y)
            { pixelFunc(outPixels, inPixels, width, y, callback_fn); },
            result);
    }

    extern "C" HRESULT __cdecl TransformImage1(
        _In_reads_(nimages) const Image *srcImages, _In_ size_t nimages, _In_ const TexMetadata &metadata,
        _In_ TransformFunc *pixelFunc, void *callback_fn,
        ScratchImage &result)
    {
        return DirectX::TransformImage(
            srcImages, nimages, metadata, [&](XMVECTOR *outPixels, const XMVECTOR *inPixels, size_t width, size_t y)
            { pixelFunc(outPixels, inPixels, width, y, callback_fn); },
            result);
    }

    extern "C" HRESULT __cdecl SaveToDDSMemory(
        _In_ const Image &image,
        _In_ DDS_FLAGS flags,
        _Out_ Blob &blob) noexcept
    {
        return DirectX::SaveToDDSMemory(image, flags, blob);
    }

    extern "C" HRESULT __cdecl SaveToDDSMemory1(
        _In_reads_(nimages) const Image *images, _In_ size_t nimages, _In_ const TexMetadata &metadata,
        _In_ DDS_FLAGS flags,
        _Out_ Blob &blob) noexcept
    {
        return DirectX::SaveToDDSMemory(images, nimages, metadata, flags, blob);
    }

    extern "C" HRESULT __cdecl SaveToDDSFile(_In_ const Image &image, _In_ DDS_FLAGS flags, _In_z_ const wchar_t *szFile) noexcept
    {
        return DirectX::SaveToDDSFile(image, flags, szFile);
    }

    extern "C" HRESULT __cdecl SaveToDDSFile1(
        _In_reads_(nimages) const Image *images, _In_ size_t nimages, _In_ const TexMetadata &metadata,
        _In_ DDS_FLAGS flags, _In_z_ const wchar_t *szFile) noexcept
    {
        return DirectX::SaveToDDSFile(images, nimages, metadata, flags, szFile);
    }

    extern "C" HRESULT __cdecl GetMetadataFromTGAMemory(
        _In_reads_bytes_(size) const void *pSource, _In_ size_t size,
        _In_ TGA_FLAGS flags,
        _Out_ TexMetadata &metadata) noexcept
    {
        return DirectX::GetMetadataFromTGAMemory(pSource, size, flags, metadata);
    }

    extern "C" HRESULT __cdecl GetMetadataFromTGAFile(
        _In_z_ const wchar_t *szFile,
        _In_ TGA_FLAGS flags,
        _Out_ TexMetadata &metadata) noexcept
    {
        return DirectX::GetMetadataFromTGAFile(szFile, flags, metadata);
    }

    extern "C" HRESULT __cdecl LoadFromTGAMemory(
        _In_reads_bytes_(size) const void *pSource, _In_ size_t size,
        _In_ TGA_FLAGS flags,
        _Out_opt_ TexMetadata *metadata, _Out_ ScratchImage &image) noexcept
    {
        return DirectX::LoadFromTGAMemory(pSource, size, flags, metadata, image);
    }

    extern "C" HRESULT __cdecl LoadFromTGAFile(
        _In_z_ const wchar_t *szFile,
        _In_ TGA_FLAGS flags,
        _Out_opt_ TexMetadata *metadata, _Out_ ScratchImage &image) noexcept
    {
        return DirectX::LoadFromTGAFile(szFile, flags, metadata, image);
    }

    extern "C" HRESULT __cdecl SaveToTGAMemory(_In_ const Image &image,
                                               _In_ TGA_FLAGS flags,
                                               _Out_ Blob &blob, _In_opt_ const TexMetadata *metadata) noexcept
    {
        return DirectX::SaveToTGAMemory(image, flags, blob, metadata);
    }

    extern "C" HRESULT __cdecl SaveToTGAFile(_In_ const Image &image,
                                             _In_ TGA_FLAGS flags,
                                             _In_z_ const wchar_t *szFile, _In_opt_ const TexMetadata *metadata) noexcept
    {
        return DirectX::SaveToTGAFile(image, flags, szFile, metadata);
    }

    extern "C" bool __cdecl IsBGR(_In_ DXGI_FORMAT fmt) noexcept
    {
        return DirectX::IsBGR(fmt);
    }
}
