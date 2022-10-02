use std::ffi::c_void;
use std::ptr;

use directxtex_sys::{self as sys, TexMetadata};

use super::CWide;
use crate::Blob;
use crate::{Error, Result, ScratchImage};

pub fn metadata<'a, CWIDE>(file_name: CWIDE) -> Result<TexMetadata>
where
    CWide<'a>: TryFrom<CWIDE>,
    Error: From<<CWide<'a> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    let mut metadata = TexMetadata::default();
    unsafe { sys::GetMetadataFromHDRFile(file_name.as_ptr(), &mut metadata) }.ok()?;
    Ok(metadata)
}

pub fn metadata_from_memory(buffer: &[u8]) -> Result<TexMetadata> {
    let mut metadata = TexMetadata::default();
    unsafe {
        sys::GetMetadataFromHDRMemory(
            buffer.as_ptr().cast::<c_void>(),
            buffer.len(),
            &mut metadata,
        )
    }
    .ok()?;
    Ok(metadata)
}

pub fn load<'a, CWIDE>(file_name: CWIDE) -> Result<ScratchImage>
where
    CWide<'a>: TryFrom<CWIDE>,
    Error: From<<CWide<'a> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    let mut out = sys::ScratchImage::default();

    unsafe { sys::LoadFromHDRFile(file_name.as_ptr(), ptr::null_mut(), &mut out) }
        .ok()
        .map(|_| ScratchImage(out))
}

pub fn load_from_memory(buffer: &[u8]) -> Result<ScratchImage> {
    let mut out = sys::ScratchImage::default();

    unsafe {
        sys::LoadFromHDRMemory(
            buffer.as_ptr().cast::<c_void>(),
            buffer.len(),
            ptr::null_mut(),
            &mut out,
        )
        .ok()
        .map(|_| ScratchImage(out))
    }
}

pub fn save<'a, CWIDE>(file_name: CWIDE, image: &sys::Image) -> Result<()>
where
    CWide<'a>: TryFrom<CWIDE>,
    Error: From<<CWide<'a> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    unsafe { sys::SaveToHDRFile(image, file_name.as_ptr()) }.ok()
}

pub fn save_to_memory(image: &sys::Image) -> Result<Blob> {
    let mut blob = sys::Blob::default();
    unsafe { sys::SaveToHDRMemory(image, &mut blob) }
        .ok()
        .map(|_| Blob(blob))
}
