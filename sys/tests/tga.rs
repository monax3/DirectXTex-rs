use directxtex_sys::*;
pub mod common;
use common::prelude::*;
use common::{DropBlob, DropScratch, TempFile, TGA_FILE};

#[test]
fn test_tga() {
    let buf = std::fs::read(TGA_FILE).unwrap();
    let file_wide = HSTRING::from(TGA_FILE);

    let mut metadata_mem1 = TexMetadata::default();
    assert_eq!(
        unsafe {
            GetMetadataFromTGAMemory(
                buf.as_ptr(),
                buf.len(),
                TGA_FLAGS::default(),
                &mut metadata_mem1,
            )
        },
        S_OK
    );

    let mut metadata_file1 = TexMetadata::default();
    assert_eq!(
        unsafe {
            GetMetadataFromTGAFile(
                file_wide.as_ptr(),
                TGA_FLAGS::default(),
                &mut metadata_file1,
            )
        },
        S_OK
    );

    let mut metadata_mem2 = TexMetadata::default();
    let mut image_mem = DropScratch::default();
    assert_eq!(
        unsafe {
            LoadFromTGAMemory(
                buf.as_ptr(),
                buf.len(),
                TGA_FLAGS::default(),
                &mut metadata_mem2,
                &mut *image_mem,
            )
        },
        S_OK
    );

    let mut metadata_file2 = TexMetadata::default();
    let mut image_file = DropScratch::default();
    assert_eq!(
        unsafe {
            LoadFromTGAFile(
                file_wide.as_ptr(),
                TGA_FLAGS::default(),
                &mut metadata_file2,
                &mut *image_file,
            )
        },
        S_OK
    );

    // GetMetadataFromTGA and LoadFromTGA produce TexMetadata with different alpha
    // mode in miscFlags2
    assert_eq!(metadata_mem1, metadata_file1);
    assert_eq!(metadata_mem2, metadata_file2);

    let image = unsafe { image_file.GetImage(0, 0, 0) };
    assert!(!image.is_null());

    let mut blob = DropBlob::default();
    assert_eq!(
        unsafe { SaveToTGAMemory(image, TGA_FLAGS::default(), &mut *blob, &metadata_file1,) },
        S_OK
    );
    blob.assert_has_data("SaveToTGAMemory");

    let image = unsafe { image_mem.GetImage(0, 0, 0) };
    assert!(!image.is_null());

    let out = TempFile::new();
    assert_eq!(
        unsafe {
            SaveToTGAFile(
                image,
                TGA_FLAGS::default(),
                out.as_wideptr(),
                &metadata_mem1,
            )
        },
        S_OK
    );
    out.assert_has_data("SaveToTGAFile");
}
