use std::ptr;

use directxtex_sys::{self as sys, TexMetadata, DDS_FLAGS};

use crate::error::hresult;
use crate::{Blob, CWide, Error, Image, Result, ScratchImage};

#[inline]
pub fn metadata<'file_name, CWIDE>(file_name: CWIDE, dds_flags: DDS_FLAGS) -> Result<TexMetadata>
where
    CWide<'file_name>: TryFrom<CWIDE>,
    Error: From<<CWide<'file_name> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    let mut metadata = TexMetadata::default();
    hresult(unsafe { sys::GetMetadataFromDDSFile(file_name.as_ptr(), dds_flags, &mut metadata) })?;
    Ok(metadata)
}

#[inline]
pub fn metadata_from_memory(buffer: &[u8], dds_flags: DDS_FLAGS) -> Result<TexMetadata> {
    let mut metadata = TexMetadata::default();
    hresult(unsafe {
        sys::GetMetadataFromDDSMemory(buffer.as_ptr(), buffer.len(), dds_flags, &mut metadata)
    })?;
    Ok(metadata)
}

#[inline]
pub fn load<'file_name, CWIDE>(file_name: CWIDE, dds_flags: DDS_FLAGS) -> Result<ScratchImage>
where
    CWide<'file_name>: TryFrom<CWIDE>,
    Error: From<<CWide<'file_name> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    let mut out = sys::ScratchImage::default();

    hresult(unsafe {
        sys::LoadFromDDSFile(file_name.as_ptr(), dds_flags, ptr::null_mut(), &mut out)
    })
    .map(|_| ScratchImage(out))
}

#[inline]
pub fn load_from_memory(buffer: &[u8], dds_flags: DDS_FLAGS) -> Result<ScratchImage> {
    let mut out = sys::ScratchImage::default();

    unsafe {
        hresult(sys::LoadFromDDSMemory(
            buffer.as_ptr(),
            buffer.len(),
            dds_flags,
            ptr::null_mut(),
            &mut out,
        ))
        .map(|_| ScratchImage(out))
    }
}

#[inline]
pub fn save<'file_name, CWIDE>(file_name: CWIDE, image: &Image, dds_flags: DDS_FLAGS) -> Result<()>
where
    CWide<'file_name>: TryFrom<CWIDE>,
    Error: From<<CWide<'file_name> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    hresult(unsafe { sys::SaveToDDSFile(image.as_ptr(), dds_flags, file_name.as_ptr()) })
}

#[inline]
pub fn save_slice<'file_name, CWIDE>(
    file_name: CWIDE,
    images: &[Image],
    metadata: &TexMetadata,
    dds_flags: DDS_FLAGS,
) -> Result<()>
where
    CWide<'file_name>: TryFrom<CWIDE>,
    Error: From<<CWide<'file_name> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    hresult(unsafe {
        sys::SaveToDDSFile1(
            Image::slice_as_ptr(images),
            images.len(),
            metadata,
            dds_flags,
            file_name.as_ptr(),
        )
    })
}

#[inline]
pub fn save_to_memory(image: &Image, dds_flags: DDS_FLAGS) -> Result<Blob> {
    let mut blob = sys::Blob::default();
    hresult(unsafe { sys::SaveToDDSMemory(image.as_ptr(), dds_flags, &mut blob) })
        .map(|_| Blob(blob))
}

#[inline]
pub fn save_slice_to_memory(
    images: &[Image],
    metadata: &TexMetadata,
    dds_flags: DDS_FLAGS,
) -> Result<Blob> {
    let mut blob = sys::Blob::default();
    hresult(unsafe {
        sys::SaveToDDSMemory1(
            Image::slice_as_ptr(images),
            images.len(),
            metadata,
            dds_flags,
            &mut blob,
        )
    })
    .map(|_| Blob(blob))
}
