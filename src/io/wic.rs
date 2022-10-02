use std::ffi::{c_void, OsStr};
use std::ptr;

use directxtex_sys::{self as sys, GetWICCodec, TexMetadata, WICCodecs, GUID, WIC_FLAGS};

use super::CWide;
use crate::Blob;
use crate::{Error, Result, ScratchImage};

pub fn metadata<'a, CWIDE>(file_name: CWIDE, wic_flags: WIC_FLAGS) -> Result<TexMetadata>
where
    CWide<'a>: TryFrom<CWIDE>,
    Error: From<<CWide<'a> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    let mut metadata = TexMetadata::default();
    unsafe { sys::GetMetadataFromWICFile(file_name.as_ptr(), wic_flags, &mut metadata, None) }
        .ok()?;
    Ok(metadata)
}

pub fn metadata_from_memory(buffer: &[u8], wic_flags: WIC_FLAGS) -> Result<TexMetadata> {
    let mut metadata = TexMetadata::default();
    unsafe {
        sys::GetMetadataFromWICMemory(
            buffer.as_ptr().cast::<c_void>(),
            buffer.len(),
            wic_flags,
            &mut metadata,
            None,
        )
    }
    .ok()?;
    Ok(metadata)
}

pub fn load<'a, CWIDE>(file_name: CWIDE, wic_flags: WIC_FLAGS) -> Result<ScratchImage>
where
    CWide<'a>: TryFrom<CWIDE>,
    Error: From<<CWide<'a> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    let mut out = sys::ScratchImage::default();

    unsafe {
        sys::LoadFromWICFile(
            file_name.as_ptr(),
            wic_flags,
            ptr::null_mut(),
            &mut out,
            None,
        )
    }
    .ok()
    .map(|_| ScratchImage(out))
}

pub fn load_from_memory(buffer: &[u8], wic_flags: WIC_FLAGS) -> Result<ScratchImage> {
    let mut out = sys::ScratchImage::default();

    unsafe {
        sys::LoadFromWICMemory(
            buffer.as_ptr().cast::<c_void>(),
            buffer.len(),
            wic_flags,
            ptr::null_mut(),
            &mut out,
            None,
        )
        .ok()
        .map(|_| ScratchImage(out))
    }
}

pub fn save<'a, CWIDE>(
    file_name: CWIDE,
    container: WICCodecs,
    image: &sys::Image,
    wic_flags: WIC_FLAGS,
) -> Result<()>
where
    CWide<'a>: TryFrom<CWIDE>,
    Error: From<<CWide<'a> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    unsafe {
        sys::SaveToWICFile(
            image,
            wic_flags,
            wic_guid(container),
            file_name.as_ptr(),
            ptr::null(),
            None,
        )
    }
    .ok()
}

pub fn save_slice<'a, CWIDE>(
    file_name: CWIDE,
    container: WICCodecs,
    images: &[sys::Image],
    wic_flags: WIC_FLAGS,
) -> Result<()>
where
    CWide<'a>: TryFrom<CWIDE>,
    Error: From<<CWide<'a> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    unsafe {
        sys::SaveToWICFile1(
            images.as_ptr(),
            images.len(),
            wic_flags,
            wic_guid(container),
            file_name.as_ptr(),
            ptr::null(),
            None,
        )
    }
    .ok()
}

pub fn save_to_memory(
    container: WICCodecs,
    image: &sys::Image,
    wic_flags: WIC_FLAGS,
) -> Result<Blob> {
    let mut blob = sys::Blob::default();
    unsafe {
        sys::SaveToWICMemory(
            image,
            wic_flags,
            wic_guid(container),
            &mut blob,
            ptr::null(),
            None,
        )
    }
    .ok()
    .map(|_| Blob(blob))
}

pub fn save_slice_to_memory(
    images: &[sys::Image],
    container: WICCodecs,
    wic_flags: WIC_FLAGS,
) -> Result<Blob> {
    let mut blob = sys::Blob::default();
    unsafe {
        sys::SaveToWICMemory1(
            images.as_ptr(),
            images.len(),
            wic_flags,
            wic_guid(container),
            &mut blob,
            ptr::null(),
            None,
        )
    }
    .ok()
    .map(|_| Blob(blob))
}

#[must_use]
#[inline]
pub fn wic_guid(codec: WICCodecs) -> &'static GUID {
    unsafe { GetWICCodec(codec).as_ref() }.expect("GetWICCodec returned NULL")
}

#[must_use]
#[inline]
pub fn wic_guid_by_ext(ext: &OsStr) -> Option<&'static GUID> { wic_codec_by_ext(ext).map(wic_guid) }

pub const WIC_CODEC_BMP: WICCodecs = WICCodecs::WIC_CODEC_BMP;
pub const WIC_CODEC_WMP: WICCodecs = WICCodecs::WIC_CODEC_WMP;
pub const WIC_CODEC_HEIF: WICCodecs = WICCodecs::WIC_CODEC_HEIF;
pub const WIC_CODEC_JPEG: WICCodecs = WICCodecs::WIC_CODEC_JPEG;
pub const WIC_CODEC_PNG: WICCodecs = WICCodecs::WIC_CODEC_PNG;
pub const WIC_CODEC_TIFF: WICCodecs = WICCodecs::WIC_CODEC_TIFF;

#[must_use]
#[inline]
pub fn wic_codec_by_ext(ext: impl AsRef<OsStr>) -> Option<WICCodecs> {
    let ext: &OsStr = ext.as_ref();
    match ext {
        ext if ext.eq_ignore_ascii_case("bmp") => Some(WIC_CODEC_BMP),
        ext if ext.eq_ignore_ascii_case("hdp") => Some(WIC_CODEC_WMP),
        ext if ext.eq_ignore_ascii_case("heic") => Some(WIC_CODEC_HEIF),
        ext if ext.eq_ignore_ascii_case("heif") => Some(WIC_CODEC_HEIF),
        ext if ext.eq_ignore_ascii_case("jpeg") => Some(WIC_CODEC_JPEG),
        ext if ext.eq_ignore_ascii_case("jpg") => Some(WIC_CODEC_JPEG),
        ext if ext.eq_ignore_ascii_case("jxr") => Some(WIC_CODEC_WMP),
        ext if ext.eq_ignore_ascii_case("png") => Some(WIC_CODEC_PNG),
        ext if ext.eq_ignore_ascii_case("tif") => Some(WIC_CODEC_TIFF),
        ext if ext.eq_ignore_ascii_case("tiff") => Some(WIC_CODEC_TIFF),
        ext if ext.eq_ignore_ascii_case("wdp") => Some(WIC_CODEC_WMP),
        _ => None,
    }
}
