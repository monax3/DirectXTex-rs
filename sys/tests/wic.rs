use directxtex_sys::{
    GetMetadataFromWICFile,
    GetMetadataFromWICMemory,
    GetWICCodec,
    IPropertyBag2,
    IWICMetadataQueryReader,
    LoadFromWICFile,
    LoadFromWICMemory,
    SaveToWICFile,
    SaveToWICFile1,
    SaveToWICMemory,
    SaveToWICMemory1,
    TexMetadata,
    WICCodecs,
    WIC_FLAGS,
};

pub mod common;
use common::prelude::*;
use common::{initialize_com, DropBlob, DropScratch, FailureFlag, TempFile, PNG_FILE};

#[test]
fn test_wic() {
    static FLAG: FailureFlag = FailureFlag::new();

    extern "C" fn get_mqr(mqr: *mut IWICMetadataQueryReader, _userdata: *mut u8) {
        if mqr.is_null() {
            FLAG.fail();
        }
    }

    extern "C" fn set_custom_props(props: *mut IPropertyBag2, _userdata: *mut u8) {
        if props.is_null() {
            FLAG.fail();
        }
    }

    let png_mem_orig = std::fs::read(PNG_FILE).unwrap();
    let file_wide = HSTRING::from(PNG_FILE);

    initialize_com();

    let mut metadata_mem1 = TexMetadata::default();
    assert_eq!(
        unsafe {
            GetMetadataFromWICMemory(
                png_mem_orig.as_ptr(),
                png_mem_orig.len(),
                WIC_FLAGS::default(),
                &mut metadata_mem1,
                None,
                std::ptr::null_mut(),
            )
        },
        S_OK
    );

    let mut metadata_file1 = TexMetadata::default();
    assert_eq!(
        unsafe {
            GetMetadataFromWICFile(
                file_wide.as_ptr(),
                WIC_FLAGS::default(),
                &mut metadata_file1,
                Some(get_mqr),
                std::ptr::null_mut(),
            )
        },
        S_OK
    );
    FLAG.assert_success("GetMetadataFromWICFile");

    let mut metadata_mem2 = TexMetadata::default();
    let mut image_mem = DropScratch::default();
    assert_eq!(
        unsafe {
            LoadFromWICMemory(
                png_mem_orig.as_ptr(),
                png_mem_orig.len(),
                WIC_FLAGS::default(),
                &mut metadata_mem2,
                &mut *image_mem,
                Some(get_mqr),
                std::ptr::null_mut(),
            )
        },
        S_OK
    );
    FLAG.assert_success("LoadFromWICMemory");

    let mut metadata_file2 = TexMetadata::default();
    let mut image_file = DropScratch::default();
    assert_eq!(
        unsafe {
            LoadFromWICFile(
                file_wide.as_ptr(),
                WIC_FLAGS::default(),
                &mut metadata_file2,
                &mut *image_file,
                None,
                std::ptr::null_mut(),
            )
        },
        S_OK
    );

    assert_eq!(metadata_mem1, metadata_mem2);
    assert_eq!(metadata_mem2, metadata_file1);
    assert_eq!(metadata_file1, metadata_file2);

    let image = unsafe { image_file.GetImage(0, 0, 0) };
    assert!(!image.is_null());

    let mut blob = DropBlob::default();
    assert_eq!(
        unsafe {
            SaveToWICMemory(
                image,
                WIC_FLAGS::default(),
                GetWICCodec(WICCodecs::WIC_CODEC_PNG),
                &mut blob.0,
                null(),
                None,
                std::ptr::null_mut(),
            )
        },
        S_OK
    );
    blob.assert_has_data("SaveToWICMemory");

    let mut blob = DropBlob::default();
    assert_eq!(
        unsafe {
            SaveToWICMemory1(
                image,
                1,
                WIC_FLAGS::default(),
                GetWICCodec(WICCodecs::WIC_CODEC_BMP),
                &mut *blob,
                null(),
                Some(set_custom_props),
                std::ptr::null_mut(),
            )
        },
        S_OK
    );
    FLAG.assert_success("SaveToWICMemory1");
    blob.assert_has_data("SaveToWICMemory1");

    let image = unsafe { image_file.GetImage(0, 0, 0) };
    assert!(!image.is_null());

    let out = TempFile::new();
    assert_eq!(
        unsafe {
            SaveToWICFile(
                image,
                WIC_FLAGS::default(),
                GetWICCodec(WICCodecs::WIC_CODEC_JPEG),
                out.as_wideptr(),
                null(),
                Some(set_custom_props),
                std::ptr::null_mut(),
            )
        },
        S_OK
    );
    FLAG.assert_success("SaveToWICFile");
    out.assert_has_data("SaveToWICFile");

    let out = TempFile::new();
    assert_eq!(
        unsafe {
            SaveToWICFile1(
                image,
                1,
                WIC_FLAGS::default(),
                GetWICCodec(WICCodecs::WIC_CODEC_TIFF),
                out.as_wideptr(),
                null(),
                None,
                std::ptr::null_mut(),
            )
        },
        S_OK
    );
    out.assert_has_data("SaveToWICFile1");
}
