use std::ptr;

use directxtex_sys::{self as sys, TexMetadata, TGA_FLAGS};

use crate::error::hresult;
use crate::{Blob, CWide, Error, Image, Result, ScratchImage};

pub fn metadata<'file_name, CWIDE>(file_name: CWIDE, tga_flags: TGA_FLAGS) -> Result<TexMetadata>
where
    CWide<'file_name>: TryFrom<CWIDE>,
    Error: From<<CWide<'file_name> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    let mut metadata = TexMetadata::default();
    hresult(unsafe { sys::GetMetadataFromTGAFile(file_name.as_ptr(), tga_flags, &mut metadata) })?;
    Ok(metadata)
}

pub fn metadata_from_memory(buffer: &[u8], tga_flags: TGA_FLAGS) -> Result<TexMetadata> {
    let mut metadata = TexMetadata::default();
    hresult(unsafe {
        sys::GetMetadataFromTGAMemory(buffer.as_ptr(), buffer.len(), tga_flags, &mut metadata)
    })?;
    Ok(metadata)
}

pub fn load<'file_name, CWIDE>(file_name: CWIDE, tga_flags: TGA_FLAGS) -> Result<ScratchImage>
where
    CWide<'file_name>: TryFrom<CWIDE>,
    Error: From<<CWide<'file_name> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    let mut out = sys::ScratchImage::default();

    hresult(unsafe {
        sys::LoadFromTGAFile(file_name.as_ptr(), tga_flags, ptr::null_mut(), &mut out)
    })
    .map(|_| ScratchImage(out))
}

pub fn load_from_memory(buffer: &[u8], tga_flags: TGA_FLAGS) -> Result<ScratchImage> {
    let mut out = sys::ScratchImage::default();

    hresult(unsafe {
        sys::LoadFromTGAMemory(
            buffer.as_ptr(),
            buffer.len(),
            tga_flags,
            ptr::null_mut(),
            &mut out,
        )
    })
    .map(|_| ScratchImage(out))
}

pub fn save<'file_name, CWIDE>(
    file_name: CWIDE,
    image: &Image,
    metadata: &TexMetadata,
    tga_flags: TGA_FLAGS,
) -> Result<()>
where
    CWide<'file_name>: TryFrom<CWIDE>,
    Error: From<<CWide<'file_name> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    hresult(unsafe { sys::SaveToTGAFile(image.as_ptr(), tga_flags, file_name.as_ptr(), metadata) })
}

pub fn save_to_memory(image: &Image, metadata: &TexMetadata, tga_flags: TGA_FLAGS) -> Result<Blob> {
    let mut blob = sys::Blob::default();
    hresult(unsafe { sys::SaveToTGAMemory(image.as_ptr(), tga_flags, &mut blob, metadata) })
        .map(|_| Blob(blob))
}
