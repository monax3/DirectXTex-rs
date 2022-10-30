use std::ffi::OsStr;
use std::ptr;

use directxtex_sys::{self as sys, GetWICCodec, TexMetadata, WICCodecs, GUID, WIC_FLAGS};

use crate::error::hresult;
use crate::{Blob, CWide, Error, Image, Result, ScratchImage};

pub fn metadata<'file_name, CWIDE>(file_name: CWIDE, wic_flags: WIC_FLAGS) -> Result<TexMetadata>
where
    CWide<'file_name>: TryFrom<CWIDE>,
    Error: From<<CWide<'file_name> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    let mut metadata = TexMetadata::default();
    hresult(unsafe {
        sys::GetMetadataFromWICFile(
            file_name.as_ptr(),
            wic_flags,
            &mut metadata,
            None,
            std::ptr::null_mut(),
        )
    })?;
    Ok(metadata)
}

pub fn metadata_from_memory(buffer: &[u8], wic_flags: WIC_FLAGS) -> Result<TexMetadata> {
    let mut metadata = TexMetadata::default();
    hresult(unsafe {
        sys::GetMetadataFromWICMemory(
            buffer.as_ptr(),
            buffer.len(),
            wic_flags,
            &mut metadata,
            None,
            std::ptr::null_mut(),
        )
    })?;
    Ok(metadata)
}

pub fn load<'file_name, CWIDE>(file_name: CWIDE, wic_flags: WIC_FLAGS) -> Result<ScratchImage>
where
    CWide<'file_name>: TryFrom<CWIDE>,
    Error: From<<CWide<'file_name> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    let mut out = sys::ScratchImage::default();

    hresult(unsafe {
        sys::LoadFromWICFile(
            file_name.as_ptr(),
            wic_flags,
            ptr::null_mut(),
            &mut out,
            None,
            std::ptr::null_mut(),
        )
    })
    .map(|_| ScratchImage(out))
}

pub fn load_from_memory(buffer: &[u8], wic_flags: WIC_FLAGS) -> Result<ScratchImage> {
    let mut out = sys::ScratchImage::default();

    hresult(unsafe {
        sys::LoadFromWICMemory(
            buffer.as_ptr(),
            buffer.len(),
            wic_flags,
            ptr::null_mut(),
            &mut out,
            None,
            std::ptr::null_mut(),
        )
    })
    .map(|_| ScratchImage(out))
}

pub fn save<'file_name, CWIDE>(
    file_name: CWIDE,
    container: WICCodecs,
    image: &Image,
    wic_flags: WIC_FLAGS,
) -> Result<()>
where
    CWide<'file_name>: TryFrom<CWIDE>,
    Error: From<<CWide<'file_name> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    hresult(unsafe {
        sys::SaveToWICFile(
            image.as_ptr(),
            wic_flags,
            wic_guid(container),
            file_name.as_ptr(),
            ptr::null(),
            None,
            std::ptr::null_mut(),
        )
    })
}

pub fn save_slice<'file_name, CWIDE>(
    file_name: CWIDE,
    container: WICCodecs,
    images: &[Image],
    wic_flags: WIC_FLAGS,
) -> Result<()>
where
    CWide<'file_name>: TryFrom<CWIDE>,
    Error: From<<CWide<'file_name> as TryFrom<CWIDE>>::Error>,
{
    let file_name: CWide = file_name.try_into()?;
    hresult(unsafe {
        sys::SaveToWICFile1(
            Image::slice_as_ptr(images),
            images.len(),
            wic_flags,
            wic_guid(container),
            file_name.as_ptr(),
            ptr::null(),
            None,
            std::ptr::null_mut(),
        )
    })
}

pub fn save_to_memory(container: WICCodecs, image: &Image, wic_flags: WIC_FLAGS) -> Result<Blob> {
    let mut blob = sys::Blob::default();
    hresult(unsafe {
        sys::SaveToWICMemory(
            image.as_ptr(),
            wic_flags,
            wic_guid(container),
            &mut blob,
            ptr::null(),
            None,
            std::ptr::null_mut(),
        )
    })
    .map(|_| Blob(blob))
}

pub fn save_slice_to_memory(
    images: &[Image],
    container: WICCodecs,
    wic_flags: WIC_FLAGS,
) -> Result<Blob> {
    let mut blob = sys::Blob::default();
    hresult(unsafe {
        sys::SaveToWICMemory1(
            Image::slice_as_ptr(images),
            images.len(),
            wic_flags,
            wic_guid(container),
            &mut blob,
            ptr::null(),
            None,
            std::ptr::null_mut(),
        )
    })
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
