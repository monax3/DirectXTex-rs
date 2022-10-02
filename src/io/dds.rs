use std::ffi::c_void;
use std::ptr;

use directxtex_sys::{self as sys, TexMetadata, DDS_FLAGS};

use super::CWide;
use crate::Blob;
use crate::{Error, Result, ScratchImage};

pub fn metadata<'a, CWIDE>(file_name: CWIDE, dds_flags: DDS_FLAGS) -> Result<TexMetadata>
where
    CWide<'a>: TryFrom<CWIDE>,
    Error: From<<CWide<'a> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    let mut metadata = TexMetadata::default();
    unsafe { sys::GetMetadataFromDDSFile(file_name.as_ptr(), dds_flags, &mut metadata) }.ok()?;
    Ok(metadata)
}

pub fn metadata_from_memory(buffer: &[u8], dds_flags: DDS_FLAGS) -> Result<TexMetadata> {
    let mut metadata = TexMetadata::default();
    unsafe {
        sys::GetMetadataFromDDSMemory(
            buffer.as_ptr().cast::<c_void>(),
            buffer.len(),
            dds_flags,
            &mut metadata,
        )
    }
    .ok()?;
    Ok(metadata)
}

pub fn load<'a, CWIDE>(file_name: CWIDE, dds_flags: DDS_FLAGS) -> Result<ScratchImage>
where
    CWide<'a>: TryFrom<CWIDE>,
    Error: From<<CWide<'a> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    let mut out = sys::ScratchImage::default();

    unsafe { sys::LoadFromDDSFile(file_name.as_ptr(), dds_flags, ptr::null_mut(), &mut out) }
        .ok()
        .map(|_| ScratchImage(out))
}

pub fn load_from_memory(buffer: &[u8], dds_flags: DDS_FLAGS) -> Result<ScratchImage> {
    let mut out = sys::ScratchImage::default();

    unsafe {
        sys::LoadFromDDSMemory(
            buffer.as_ptr().cast::<c_void>(),
            buffer.len(),
            dds_flags,
            ptr::null_mut(),
            &mut out,
        )
        .ok()
        .map(|_| ScratchImage(out))
    }
}

pub fn save<'a, CWIDE>(file_name: CWIDE, image: &sys::Image, dds_flags: DDS_FLAGS) -> Result<()>
where
    CWide<'a>: TryFrom<CWIDE>,
    Error: From<<CWide<'a> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    unsafe { sys::SaveToDDSFile(image, dds_flags, file_name.as_ptr()) }.ok()
}

pub fn save_slice<'a, CWIDE>(
    file_name: CWIDE,
    images: &[sys::Image],
    metadata: &TexMetadata,
    dds_flags: DDS_FLAGS,
) -> Result<()>
where
    CWide<'a>: TryFrom<CWIDE>,
    Error: From<<CWide<'a> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    unsafe {
        sys::SaveToDDSFile1(
            images.as_ptr(),
            images.len(),
            metadata,
            dds_flags,
            file_name.as_ptr(),
        )
    }
    .ok()
}

pub fn save_to_memory(image: &sys::Image, dds_flags: DDS_FLAGS) -> Result<Blob> {
    let mut blob = sys::Blob::default();
    unsafe { sys::SaveToDDSMemory(image, dds_flags, &mut blob) }
        .ok()
        .map(|_| Blob(blob))
}

pub fn save_slice_to_memory(
    images: &[sys::Image],
    metadata: &TexMetadata,
    dds_flags: DDS_FLAGS,
) -> Result<Blob> {
    let mut blob = sys::Blob::default();
    unsafe {
        sys::SaveToDDSMemory1(
            images.as_ptr(),
            images.len(),
            metadata,
            dds_flags,
            &mut blob,
        )
    }
    .ok()
    .map(|_| Blob(blob))
}
