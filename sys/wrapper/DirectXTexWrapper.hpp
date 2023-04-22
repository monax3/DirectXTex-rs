#include <DirectXTex.h>
#include <DirectXTexEXR.h>

using namespace DirectX;

typedef void __cdecl GetMQR(IWICMetadataQueryReader *mqr, void *callback_fn);

typedef void __cdecl SetCustomProps(IPropertyBag2 *props, void *callback_fn);

typedef void __cdecl EvaluateFunc(_In_reads_(width) const XMVECTOR *pixels, size_t width, size_t y, void *callback_fn);

typedef void __cdecl TransformFunc(_Out_writes_(width) XMVECTOR *outPixels,
                                   _In_reads_(width) const XMVECTOR *inPixels, size_t width, size_t y, void *callback_fn);

namespace Wrapper
{

#ifdef _WIN32

    extern "C" HRESULT __cdecl GetMetadataFromWICMemory(
        _In_reads_bytes_(size) const void *pSource, _In_ size_t size,
        _In_ WIC_FLAGS flags,
        _Out_ TexMetadata &metadata,
        _In_opt_ GetMQR *getMQR, void *callback_fn);

    extern "C" HRESULT __cdecl GetMetadataFromWICFile(
        _In_z_ const wchar_t *szFile,
        _In_ WIC_FLAGS flags,
        _Out_ TexMetadata &metadata,
        _In_opt_ GetMQR *getMQR, void *callback_fn);

    extern "C" HRESULT __cdecl LoadFromWICMemory(
        _In_reads_bytes_(size) const void *pSource, _In_ size_t size,
        _In_ WIC_FLAGS flags,
        _Out_opt_ TexMetadata *metadata, _Out_ ScratchImage &image,
        _In_opt_ GetMQR *getMQR, void *callback_fn);

    extern "C" HRESULT __cdecl LoadFromWICFile(
        _In_z_ const wchar_t *szFile, _In_ WIC_FLAGS flags,
        _Out_opt_ TexMetadata *metadata, _Out_ ScratchImage &image,
        _In_opt_ GetMQR *getMQR, void *callback_fn);

    extern "C" HRESULT __cdecl SaveToWICMemory(
        _In_ const Image &image, _In_ WIC_FLAGS flags, _In_ REFGUID guidContainerFormat,
        _Out_ Blob &blob, _In_opt_ const GUID *targetFormat,
        _In_opt_ SetCustomProps *setCustomProps, void *callback_fn);

    extern "C" HRESULT __cdecl SaveToWICMemory1(
        _In_count_(nimages) const Image *images, _In_ size_t nimages,
        _In_ WIC_FLAGS flags, _In_ REFGUID guidContainerFormat,
        _Out_ Blob &blob, _In_opt_ const GUID *targetFormat,
        _In_opt_ SetCustomProps *setCustomProps, void *callback_fn);

    extern "C" HRESULT __cdecl SaveToWICFile(
        _In_ const Image &image, _In_ WIC_FLAGS flags, _In_ REFGUID guidContainerFormat,
        _In_z_ const wchar_t *szFile, _In_opt_ const GUID *targetFormat,
        _In_opt_ SetCustomProps *setCustomProps, void *callback_fn);

    extern "C" HRESULT __cdecl SaveToWICFile1(
        _In_count_(nimages) const Image *images, _In_ size_t nimages,
        _In_ WIC_FLAGS flags, _In_ REFGUID guidContainerFormat,
        _In_z_ const wchar_t *szFile, _In_ const GUID *targetFormat,
        _In_opt_ SetCustomProps *setCustomProps, void *callback_fn);

#endif /* ifdef _WIN32 */

    extern "C" HRESULT __cdecl EvaluateImage(
        _In_ const Image &image,
        _In_ EvaluateFunc *pixelFunc, void *callback_fn);

    extern "C" HRESULT __cdecl EvaluateImage1(
        _In_reads_(nimages) const Image *images, _In_ size_t nimages, _In_ const TexMetadata &metadata,
        _In_ EvaluateFunc *pixelFunc, void *callback_fn);

    extern "C" HRESULT __cdecl TransformImage(
        _In_ const Image &image,
        _In_ TransformFunc *pixelFunc, void *callback_fn,
        ScratchImage &result);

    extern "C" HRESULT __cdecl TransformImage1(
        _In_reads_(nimages) const Image *srcImages, _In_ size_t nimages, _In_ const TexMetadata &metadata,
        _In_ TransformFunc *pixelFunc, void *callback_fn,
        ScratchImage &result);

    extern "C" HRESULT __cdecl SaveToDDSMemory(
        _In_ const Image &image,
        _In_ DDS_FLAGS flags,
        _Out_ Blob &blob) noexcept;

    extern "C" HRESULT __cdecl SaveToDDSMemory1(
        _In_reads_(nimages) const Image *images, _In_ size_t nimages, _In_ const TexMetadata &metadata,
        _In_ DDS_FLAGS flags,
        _Out_ Blob &blob) noexcept;

    extern "C" HRESULT __cdecl SaveToDDSFile(_In_ const Image &image, _In_ DDS_FLAGS flags, _In_z_ const wchar_t *szFile) noexcept;

    extern "C" HRESULT __cdecl SaveToDDSFile1(
        _In_reads_(nimages) const Image *images, _In_ size_t nimages, _In_ const TexMetadata &metadata,
        _In_ DDS_FLAGS flags, _In_z_ const wchar_t *szFile) noexcept;

    extern "C" HRESULT __cdecl GetMetadataFromTGAMemory(
        _In_reads_bytes_(size) const void *pSource, _In_ size_t size,
        _In_ TGA_FLAGS flags,
        _Out_ TexMetadata &metadata) noexcept;

    extern "C" HRESULT __cdecl GetMetadataFromTGAFile(
        _In_z_ const wchar_t *szFile,
        _In_ TGA_FLAGS flags,
        _Out_ TexMetadata &metadata) noexcept;

    extern "C" HRESULT __cdecl LoadFromTGAMemory(
        _In_reads_bytes_(size) const void *pSource, _In_ size_t size,
        _In_ TGA_FLAGS flags,
        _Out_opt_ TexMetadata *metadata, _Out_ ScratchImage &image) noexcept;

    extern "C" HRESULT __cdecl LoadFromTGAFile(
        _In_z_ const wchar_t *szFile,
        _In_ TGA_FLAGS flags,
        _Out_opt_ TexMetadata *metadata, _Out_ ScratchImage &image) noexcept;

    extern "C" HRESULT __cdecl SaveToTGAMemory(_In_ const Image &image,
                                               _In_ TGA_FLAGS flags,
                                               _Out_ Blob &blob, _In_opt_ const TexMetadata *metadata) noexcept;

    extern "C" HRESULT __cdecl SaveToTGAFile(_In_ const Image &image,
                                             _In_ TGA_FLAGS flags,
                                             _In_z_ const wchar_t *szFile, _In_opt_ const TexMetadata *metadata) noexcept;

    extern "C" bool __cdecl IsBGR(_In_ DXGI_FORMAT fmt) noexcept;
    extern "C" bool __cdecl IsSRGB(_In_ DXGI_FORMAT fmt) noexcept;
    extern "C" bool __cdecl IsPalettized(_In_ DXGI_FORMAT fmt) noexcept;
    extern "C" bool __cdecl IsCompressed(_In_ DXGI_FORMAT fmt) noexcept;
}
