#![allow(unsafe_code)]

pub mod common;

use common::prelude::*;
use common::{initialize_com, DropScratch, FailureFlag, TempFile, PNG_FILE};
use directxtex_sys::{
    Convert,
    EvaluateImage,
    IWICMetadataQueryReader,
    LoadFromWICFile,
    SaveToDDSFile1,
    TexMetadata,
    DDS_FLAGS,
    TEX_FILTER_FLAGS,
    TEX_THRESHOLD_DEFAULT,
    WIC_FLAGS,
    XMVECTOR,
};
#[cfg(feature = "windows")]
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R32G32B32A32_FLOAT;

#[cfg(not(feature = "windows"))]
pub const DXGI_FORMAT_R32G32B32A32_FLOAT: directxtex_sys::DXGI_FORMAT =
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R32G32B32A32_FLOAT.0;

#[test]
fn test_operations() {
    static FLAG: FailureFlag = FailureFlag::new();

    unsafe extern "C" fn get_mqr(mqr: *mut IWICMetadataQueryReader, _userdata: *mut u8) {
        if mqr.is_null() {
            FLAG.fail();
        }
    }

    unsafe extern "C" fn evaluate_func(
        pixels: *const XMVECTOR,
        _width: usize,
        _y: usize,
        _userdata: *mut u8,
    ) {
        if pixels.is_null() {
            FLAG.fail();
        }
    }

    initialize_com();

    assert!(std::path::Path::new(PNG_FILE).is_file());
    let file_wstr: Vec<u16> = PNG_FILE.encode_utf16().chain(std::iter::once(0)).collect();

    let mut metadata = TexMetadata::default();
    let mut scratch = DropScratch::default();

    unsafe {
        assert_eq!(
            LoadFromWICFile(
                file_wstr.as_ptr(),
                WIC_FLAGS::WIC_FLAGS_NONE,
                &mut metadata,
                &mut *scratch,
                Some(get_mqr),
                std::ptr::null_mut(),
            ),
            S_OK
        );
        FLAG.assert_success("LoadFromWICFile");
        scratch.assert_has_data("LoadFromWICFile");

        let image = scratch.GetImage(0, 0, 0);
        assert!(!image.is_null());

        assert_eq!(
            EvaluateImage(image, Some(evaluate_func), std::ptr::null_mut()),
            S_OK
        );
        FLAG.assert_success("EvaluateImage");

        let mut converted = DropScratch::default();
        assert_eq!(
            Convert(
                image,
                DXGI_FORMAT_R32G32B32A32_FLOAT,
                TEX_FILTER_FLAGS::default(),
                TEX_THRESHOLD_DEFAULT,
                &mut *converted
            ),
            S_OK
        );
        converted.assert_has_data("Convert");

        let out = TempFile::new();
        assert_eq!(
            SaveToDDSFile1(
                image,
                1,
                &metadata,
                DDS_FLAGS::DDS_FLAGS_NONE,
                out.as_wideptr(),
            ),
            S_OK
        );
        out.assert_has_data("SaveToDDSFile1");
    }
}
