use std::ffi::c_void;
use std::ptr;

use directxtex_sys::{self as sys, TexMetadata, TGA_FLAGS};

use super::CWide;
use crate::Blob;
use crate::{Error, Result, ScratchImage};

pub fn metadata<'a, CWIDE>(file_name: CWIDE, tga_flags: TGA_FLAGS) -> Result<TexMetadata>
where
    CWide<'a>: TryFrom<CWIDE>,
    Error: From<<CWide<'a> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    let mut metadata = TexMetadata::default();
    unsafe { sys::GetMetadataFromTGAFile(file_name.as_ptr(), tga_flags, &mut metadata) }.ok()?;
    Ok(metadata)
}

pub fn metadata_from_memory(buffer: &[u8], tga_flags: TGA_FLAGS) -> Result<TexMetadata> {
    let mut metadata = TexMetadata::default();
    unsafe {
        sys::GetMetadataFromTGAMemory(
            buffer.as_ptr().cast::<c_void>(),
            buffer.len(),
            tga_flags,
            &mut metadata,
        )
    }
    .ok()?;
    Ok(metadata)
}

pub fn load<'a, CWIDE>(file_name: CWIDE, tga_flags: TGA_FLAGS) -> Result<ScratchImage>
where
    CWide<'a>: TryFrom<CWIDE>,
    Error: From<<CWide<'a> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    let mut out = sys::ScratchImage::default();

    unsafe { sys::LoadFromTGAFile(file_name.as_ptr(), tga_flags, ptr::null_mut(), &mut out) }
        .ok()
        .map(|_| ScratchImage(out))
}

pub fn load_from_memory(buffer: &[u8], tga_flags: TGA_FLAGS) -> Result<ScratchImage> {
    let mut out = sys::ScratchImage::default();

    unsafe {
        sys::LoadFromTGAMemory(
            buffer.as_ptr().cast::<c_void>(),
            buffer.len(),
            tga_flags,
            ptr::null_mut(),
            &mut out,
        )
        .ok()
        .map(|_| ScratchImage(out))
    }
}

pub fn save<'a, CWIDE>(
    file_name: CWIDE,
    image: &sys::Image,
    metadata: &TexMetadata,
    tga_flags: TGA_FLAGS,
) -> Result<()>
where
    CWide<'a>: TryFrom<CWIDE>,
    Error: From<<CWide<'a> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    unsafe { sys::SaveToTGAFile(image, tga_flags, file_name.as_ptr(), metadata) }.ok()
}

pub fn save_to_memory(
    image: &sys::Image,
    metadata: &TexMetadata,
    tga_flags: TGA_FLAGS,
) -> Result<Blob> {
    let mut blob = sys::Blob::default();
    unsafe { sys::SaveToTGAMemory(image, tga_flags, &mut blob, metadata) }
        .ok()
        .map(|_| Blob(blob))
}
