use std::ptr;

use directxtex_sys::{self as sys, TexMetadata};

use crate::error::hresult;
use crate::{CWide, Error, Image, Result, ScratchImage};

pub fn metadata<'file_name, CWIDE>(file_name: CWIDE) -> Result<TexMetadata>
where
    CWide<'file_name>: TryFrom<CWIDE>,
    Error: From<<CWide<'file_name> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    let mut metadata = TexMetadata::default();
    hresult(unsafe { sys::GetMetadataFromEXRFile(file_name.as_ptr(), &mut metadata) })?;
    Ok(metadata)
}

pub fn load<'file_name, CWIDE>(file_name: CWIDE) -> Result<ScratchImage>
where
    CWide<'file_name>: TryFrom<CWIDE>,
    Error: From<<CWide<'file_name> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    let mut out = sys::ScratchImage::default();

    hresult(unsafe { sys::LoadFromEXRFile(file_name.as_ptr(), ptr::null_mut(), &mut out) })
        .map(|_| ScratchImage(out))
}

pub fn save<'file_name, CWIDE>(file_name: CWIDE, image: &Image) -> Result<()>
where
    CWide<'file_name>: TryFrom<CWIDE>,
    Error: From<<CWide<'file_name> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    hresult(unsafe { sys::SaveToEXRFile(image.as_ptr(), file_name.as_ptr()) })
}
