use std::ptr;

use directxtex_sys::{self as sys, TexMetadata};

use super::CWide;
use crate::{Error, Result, ScratchImage};

pub fn metadata<'a, CWIDE>(file_name: CWIDE) -> Result<TexMetadata>
where
    CWide<'a>: TryFrom<CWIDE>,
    Error: From<<CWide<'a> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    let mut metadata = TexMetadata::default();
    unsafe { sys::GetMetadataFromEXRFile(file_name.as_ptr(), &mut metadata) }.ok()?;
    Ok(metadata)
}

pub fn load<'a, CWIDE>(file_name: CWIDE) -> Result<ScratchImage>
where
    CWide<'a>: TryFrom<CWIDE>,
    Error: From<<CWide<'a> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    let mut out = sys::ScratchImage::default();

    unsafe { sys::LoadFromEXRFile(file_name.as_ptr(), ptr::null_mut(), &mut out) }
        .ok()
        .map(|_| ScratchImage(out))
}

pub fn save<'a, CWIDE>(file_name: CWIDE, image: &sys::Image) -> Result<()>
where
    CWide<'a>: TryFrom<CWIDE>,
    Error: From<<CWide<'a> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    unsafe { sys::SaveToEXRFile(image, file_name.as_ptr()) }.ok()
}
