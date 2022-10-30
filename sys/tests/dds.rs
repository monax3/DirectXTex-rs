use directxtex_sys::{
    GetMetadataFromDDSFile,
    GetMetadataFromDDSMemory,
    LoadFromDDSFile,
    LoadFromDDSMemory,
    SaveToDDSFile,
    SaveToDDSFile1,
    SaveToDDSMemory,
    SaveToDDSMemory1,
    TexMetadata,
    DDS_FLAGS,
};
pub mod common;
use common::prelude::*;
use common::{DropBlob, DropScratch, TempFile, DDS_FILE};

#[test]
fn test_dds() {
    let buf = std::fs::read(DDS_FILE).unwrap();
    let file_wide = HSTRING::from(DDS_FILE);

    let mut metadata_mem1 = TexMetadata::default();
    assert_eq!(
        unsafe {
            GetMetadataFromDDSMemory(
                buf.as_ptr(),
                buf.len(),
                DDS_FLAGS::DDS_FLAGS_ALLOW_LARGE_FILES,
                &mut metadata_mem1,
            )
        },
        S_OK
    );

    let mut metadata_file1 = TexMetadata::default();
    assert_eq!(
        unsafe {
            GetMetadataFromDDSFile(
                file_wide.as_ptr(),
                DDS_FLAGS::DDS_FLAGS_ALLOW_LARGE_FILES,
                &mut metadata_file1,
            )
        },
        S_OK
    );

    let mut metadata_mem2 = TexMetadata::default();
    let mut image_mem = DropScratch::default();
    assert_eq!(
        unsafe {
            LoadFromDDSMemory(
                buf.as_ptr(),
                buf.len(),
                DDS_FLAGS::DDS_FLAGS_ALLOW_LARGE_FILES,
                &mut metadata_mem2,
                &mut *image_mem,
            )
        },
        S_OK
    );
    image_mem.assert_has_data("LoadFromDDSMemory");

    let mut metadata_file2 = TexMetadata::default();
    let mut image_file = DropScratch::default();
    assert_eq!(
        unsafe {
            LoadFromDDSFile(
                file_wide.as_ptr(),
                DDS_FLAGS::DDS_FLAGS_ALLOW_LARGE_FILES,
                &mut metadata_file2,
                &mut *image_file,
            )
        },
        S_OK
    );
    image_file.assert_has_data("LoadFromDDSFile");

    assert_eq!(metadata_mem1, metadata_mem2);
    assert_eq!(metadata_mem2, metadata_file1);
    assert_eq!(metadata_file1, metadata_file2);

    let image = unsafe { image_file.GetImage(0, 0, 0) };
    assert!(!image.is_null());

    let mut blob = DropBlob::default();
    assert_eq!(
        unsafe { SaveToDDSMemory(image, DDS_FLAGS::DDS_FLAGS_FORCE_DX10_EXT_MISC2, &mut *blob,) },
        S_OK
    );
    blob.assert_has_data("SaveToDDSMemory");

    let mut blob = DropBlob::default();
    assert_eq!(
        unsafe {
            SaveToDDSMemory1(
                image_file.m_image,
                image_file.m_nimages,
                &metadata_mem2,
                DDS_FLAGS::DDS_FLAGS_FORCE_DX10_EXT_MISC2,
                &mut *blob,
            )
        },
        S_OK
    );
    blob.assert_has_data("SaveToDDSMemory1");

    let image = unsafe { image_mem.GetImage(0, 0, 0) };
    assert!(!image.is_null());

    common::initialize_com();

    let out = TempFile::new();
    assert_eq!(
        unsafe { SaveToDDSFile(image, DDS_FLAGS::DDS_FLAGS_FORCE_DX10_EXT, out.as_wideptr(),) },
        S_OK
    );
    out.assert_has_data("SaveToDDSFile");

    let out = TempFile::new();
    assert_eq!(
        unsafe {
            SaveToDDSFile1(
                image_mem.m_image,
                image_mem.m_nimages,
                &metadata_file2,
                DDS_FLAGS::DDS_FLAGS_FORCE_DX10_EXT,
                out.as_wideptr(),
            )
        },
        S_OK
    );
    out.assert_has_data("SaveToDDSFile1");
}
