pub mod common;
use common::prelude::*;
use directxtex_sys::*;
use windows::core::HSTRING;
#[cfg(feature = "windows")] use windows::Win32::Foundation::S_OK;
#[cfg(feature = "windows")] use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_BC7_UNORM;
#[cfg(feature = "windows")] use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R32G32B32A32_FLOAT;

#[cfg(not(feature = "windows"))]
const S_OK: HRESULT = 0;

#[cfg(not(feature = "windows"))]
const DXGI_FORMAT_BC7_UNORM: DXGI_FORMAT =
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_BC7_UNORM.0;
#[cfg(not(feature = "windows"))]
const DXGI_FORMAT_R32G32B32A32_FLOAT: DXGI_FORMAT =
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R32G32B32A32_FLOAT.0;

const PNG: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test.png");
const DDS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test.dds");
const DDS_COMPLEX: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test_complex.dds");
const DDS_1D: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test_1d.dds");
const TGA: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test.tga");
const EXR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test.exr");
const HDR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test.hdr");

#[ignore]
#[test]
fn generate_test_images() {
    common::initialize_com();

    let buf = std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/bastien-plu-tgsbkszPnJc-unsplash.jpg"
    ))
    .unwrap();
    let mut initial = ScratchImage::default();
    let mut scratch = ScratchImage::default();

    unsafe {
        assert_eq!(
            S_OK,
            LoadFromWICMemory(
                buf.as_ptr(),
                buf.len(),
                WIC_FLAGS::default(),
                std::ptr::null_mut(),
                &mut initial,
                None,
                std::ptr::null_mut(),
            )
        );

        assert_eq!(
            S_OK,
            Resize1(
                initial.m_image,
                initial.m_nimages,
                &initial.m_metadata,
                512,
                768,
                TEX_FILTER_FLAGS::default(),
                &mut scratch,
            )
        );

        let png = HSTRING::from(PNG);
        assert_eq!(
            S_OK,
            SaveToWICFile(
                scratch.GetImage(0, 0, 0),
                WIC_FLAGS::default(),
                GetWICCodec(WICCodecs::WIC_CODEC_PNG),
                png.as_ptr(),
                std::ptr::null(),
                None,
                std::ptr::null_mut(),
            )
        );

        let tga = HSTRING::from(TGA);
        assert_eq!(
            S_OK,
            SaveToTGAFile(
                scratch.GetImage(0, 0, 0),
                TGA_FLAGS::default(),
                tga.as_ptr(),
                &scratch.m_metadata,
            )
        );

        let mut hdr_scratch = ScratchImage::default();
        assert_eq!(
            S_OK,
            Convert1(
                scratch.m_image,
                scratch.m_nimages,
                &scratch.m_metadata,
                DXGI_FORMAT_R32G32B32A32_FLOAT,
                TEX_FILTER_FLAGS::default(),
                TEX_THRESHOLD_DEFAULT,
                &mut hdr_scratch,
            )
        );

        let exr = HSTRING::from(EXR);
        assert_eq!(
            S_OK,
            SaveToEXRFile(hdr_scratch.GetImage(0, 0, 0), exr.as_ptr())
        );

        let hdr = HSTRING::from(HDR);
        assert_eq!(
            S_OK,
            SaveToHDRFile(hdr_scratch.GetImage(0, 0, 0), hdr.as_ptr())
        );

        let dds = HSTRING::from(DDS);
        assert_eq!(
            S_OK,
            SaveToDDSFile(
                scratch.GetImage(0, 0, 0),
                DDS_FLAGS::default(),
                dds.as_ptr(),
            )
        );

        let mut oned = ScratchImage::default();
        assert_eq!(
            S_OK,
            Resize(
                scratch.GetImage(0, 0, 0),
                (*scratch.GetImage(0, 0, 0)).width,
                1,
                TEX_FILTER_FLAGS::default(),
                &mut oned,
            )
        );

        let dds_1d = HSTRING::from(DDS_1D);
        assert_eq!(
            S_OK,
            SaveToDDSFile(
                oned.GetImage(0, 0, 0),
                DDS_FLAGS::default(),
                dds_1d.as_ptr(),
            )
        );
        oned.Release();

        let image = scratch.GetImage(0, 0, 0).as_ref().unwrap();
        let image_buf = std::slice::from_raw_parts(image.pixels, image.slicePitch);

        let mut input = ScratchImage::default();

        const REPEATS: usize = 6;
        let mut array_buf = Vec::<u8>::with_capacity(image_buf.len() * REPEATS);
        for _ in 0 .. REPEATS {
            array_buf.extend(image_buf);
        }

        assert_eq!(
            S_OK,
            input.Initialize2D(
                scratch.m_metadata.format,
                scratch.m_metadata.width,
                scratch.m_metadata.height,
                REPEATS,
                1,
                CP_FLAGS::default(),
            )
        );

        scratch.Release();

        assert_eq!(array_buf.len(), input.m_size);

        let dst_buf = std::slice::from_raw_parts_mut(input.m_memory, input.m_size);
        dst_buf.copy_from_slice(&array_buf);
        let mut output = scratch;

        assert_eq!(
            S_OK,
            GenerateMipMaps1(
                input.m_image,
                input.m_nimages,
                &input.m_metadata,
                TEX_FILTER_FLAGS::default(),
                0,
                &mut output,
            )
        );

        std::mem::swap(&mut input, &mut output);

        assert_eq!(
            S_OK,
            Compress1(
                input.m_image,
                input.m_nimages,
                &input.m_metadata,
                DXGI_FORMAT_BC7_UNORM,
                TEX_COMPRESS_FLAGS::default(),
                TEX_THRESHOLD_DEFAULT,
                &mut output,
            )
        );

        // std::mem::swap(&mut input, &mut output);

        let dds_complex = HSTRING::from(DDS_COMPLEX);
        assert_eq!(
            S_OK,
            SaveToDDSFile1(
                output.m_image,
                output.m_nimages,
                &output.m_metadata,
                DDS_FLAGS::default(),
                dds_complex.as_ptr(),
            )
        );
    }
}
