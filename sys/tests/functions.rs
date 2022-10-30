use directxtex_sys::{
    BitsPerColor,
    BitsPerPixel,
    ComputePitch,
    ComputeScanlines,
    FormatDataType,
    HasAlpha,
    IsBGR,
    IsCompressed,
    IsDepthStencil,
    IsPacked,
    IsPalettized,
    IsPlanar,
    IsSRGB,
    IsTypeless,
    IsVideo,
    MakeLinear,
    MakeSRGB,
    MakeTypeless,
    MakeTypelessFLOAT,
    MakeTypelessUNORM,
    CP_FLAGS,
    FORMAT_TYPE,
};
#[cfg(feature = "windows")]
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R8G8B8A8_UNORM;
#[cfg(not(feature = "windows"))]
const DXGI_FORMAT_R8G8B8A8_UNORM: directxtex_sys::DXGI_FORMAT =
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R8G8B8A8_UNORM.0;

pub mod common;
use common::prelude::*;

#[test]
fn test_functions() {
    unsafe {
        assert_eq!(BitsPerColor(DXGI_FORMAT_R8G8B8A8_UNORM), 8);
        assert_eq!(BitsPerPixel(DXGI_FORMAT_R8G8B8A8_UNORM), 32);
        assert!(HasAlpha(DXGI_FORMAT_R8G8B8A8_UNORM));
        assert_eq!(
            FormatDataType(DXGI_FORMAT_R8G8B8A8_UNORM),
            FORMAT_TYPE::FORMAT_TYPE_UNORM
        );
        assert!(!IsBGR(DXGI_FORMAT_R8G8B8A8_UNORM));
        assert!(!IsCompressed(DXGI_FORMAT_R8G8B8A8_UNORM));
        assert!(!IsDepthStencil(DXGI_FORMAT_R8G8B8A8_UNORM));
        assert!(!IsPacked(DXGI_FORMAT_R8G8B8A8_UNORM));
        assert!(!IsPalettized(DXGI_FORMAT_R8G8B8A8_UNORM));
        assert!(!IsPlanar(DXGI_FORMAT_R8G8B8A8_UNORM));
        assert!(!IsSRGB(DXGI_FORMAT_R8G8B8A8_UNORM));
        assert!(IsSRGB(MakeSRGB(DXGI_FORMAT_R8G8B8A8_UNORM)));
        assert!(!IsTypeless(DXGI_FORMAT_R8G8B8A8_UNORM, false));
        let typeless = MakeTypeless(DXGI_FORMAT_R8G8B8A8_UNORM);
        assert!(IsTypeless(typeless, false));
        assert_eq!(MakeTypelessUNORM(typeless), DXGI_FORMAT_R8G8B8A8_UNORM);
        assert_ne!(MakeTypelessFLOAT(typeless), DXGI_FORMAT_R8G8B8A8_UNORM);
        assert!(!IsVideo(DXGI_FORMAT_R8G8B8A8_UNORM));
        assert_eq!(
            MakeLinear(DXGI_FORMAT_R8G8B8A8_UNORM),
            DXGI_FORMAT_R8G8B8A8_UNORM
        );
        let (mut row_pitch, mut slice_pitch): (usize, usize) = (0, 0);
        assert_eq!(
            ComputePitch(
                DXGI_FORMAT_R8G8B8A8_UNORM,
                32,
                32,
                &mut row_pitch,
                &mut slice_pitch,
                CP_FLAGS::default()
            ),
            S_OK
        );
        assert_eq!((row_pitch, slice_pitch), (32 * 4, 32 * 32 * 4));
        assert_eq!(
            ComputePitch(
                DXGI_FORMAT_R8G8B8A8_UNORM,
                32,
                32,
                &mut row_pitch,
                &mut slice_pitch,
                CP_FLAGS::CP_FLAGS_24BPP
            ),
            S_OK
        );
        assert_eq!((row_pitch, slice_pitch), (32 * 3, 32 * 32 * 3));
        assert_eq!(ComputeScanlines(DXGI_FORMAT_R8G8B8A8_UNORM, 32), 32);
    }
    // UNTESTED:
    // IsSupportedTexture
    // CaptureTexture
    // Compress
    // Compress1
    // Compress2
    // Compress3
    // ComputeMSE
    // ComputeNormalMap
    // ComputeNormalMap1
    // Convert
    // Convert1
    // ConvertToSinglePlane
    // ConvertToSinglePlane1
    // CopyRectangle
    // CreateShaderResourceView
    // CreateShaderResourceViewEx
    // CreateTexture
    // CreateTextureEx
    // Decompress
    // Decompress1
    // EncodeDDSHeader
    // EvaluateImage
    // EvaluateImage1
    // FlipRotate
    // FlipRotate1
    // GenerateMipMaps
    // GenerateMipMaps1
    // GenerateMipMaps3D
    // GenerateMipMaps3D1
    // GetMetadataFromEXRFile
    // GetMetadataFromHDRFile
    // GetMetadataFromHDRMemory
    // LoadFromEXRFile
    // LoadFromHDRFile
    // LoadFromHDRMemory
    // GetWICFactory
    // PremultiplyAlpha
    // PremultiplyAlpha1
    // Resize
    // Resize1
    // SaveToEXRFile
    // SaveToHDRFile
    // SaveToHDRMemory
    // ScaleMipMapsAlphaForCoverage
    // ScratchImage_GetImage
    // ScratchImage_Initialize
    // ScratchImage_Initialize1D
    // ScratchImage_Initialize2D
    // ScratchImage_Initialize3D
    // ScratchImage_Initialize3DFromImages
    // ScratchImage_InitializeArrayFromImages
    // ScratchImage_InitializeCube
    // ScratchImage_InitializeCubeFromImages
    // ScratchImage_InitializeFromImage
    // ScratchImage_IsAlphaAllOpaque
    // ScratchImage_OverrideFormat
    // ScratchImage_Release
    // SetWICFactory
    // TexMetadata_ComputeIndex
    // TransformImage
    // TransformImage1
    // XMFLOAT3X3_XMFLOAT3X3
    // XMFLOAT3X4_XMFLOAT3X4
    // XMFLOAT4X3_XMFLOAT4X3
    // XMFLOAT4X4_XMFLOAT4X4
    // XMMATRIX_XMMATRIX
    // XMMATRIX_XMMATRIX1
    // XMScalarACos
    // XMScalarACosEst
    // XMScalarASin
    // XMScalarASinEst
    // XMScalarCos
    // XMScalarCosEst
    // XMScalarModAngle
    // XMScalarNearEqual
    // XMScalarSin
    // XMScalarSinCos
    // XMScalarSinCosEst
    // XMScalarSinEst
    // XMVerifyCPUSupport
}
