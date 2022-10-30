use directxtex_sys::{Blob, Blob_Initialize, Blob_Release, Blob_Resize, Blob_Trim};

pub mod common;
use common::prelude::*;
use common::BUFSIZE;

#[test]
fn test_blob() {
    fn assert_blob_valid(blob: &Blob, expected: Option<usize>) {
        if let Some(expected) = expected {
            assert!(!blob.m_buffer.is_null());
            assert_eq!(blob.m_size, expected);
        } else {
            assert!(blob.m_buffer.is_null());
            assert_eq!(blob.m_size, 0);
        }
    }

    let mut blob = Blob::default();
    assert_blob_valid(&blob, None);

    assert_eq!(unsafe { Blob_Initialize(&mut blob, BUFSIZE) }, S_OK);
    assert_blob_valid(&blob, Some(BUFSIZE));
    assert_eq!(unsafe { Blob_Resize(&mut blob, BUFSIZE * 2) }, S_OK);
    assert_blob_valid(&blob, Some(BUFSIZE * 2));
    assert_eq!(unsafe { Blob_Trim(&mut blob, BUFSIZE) }, S_OK);
    assert_blob_valid(&blob, Some(BUFSIZE));
    // The actual error (E_INVALIDARG) is an implementation detail
    assert_ne!(unsafe { Blob_Trim(&mut blob, BUFSIZE * 2) }, S_OK);
    assert_blob_valid(&blob, Some(BUFSIZE));
    unsafe { Blob_Release(&mut blob) };
    assert_blob_valid(&blob, None);
}
