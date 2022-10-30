#![allow(unsafe_code)]

use std::ffi::OsStr;
use std::path::Path;

use directxtex_sys as sys;
pub use sys::{
    Blob as RawBlob,
    Image as RawImage,
    Rect,
    ScratchImage as RawScratchImage,
    TexMetadata,
    WICCodecs,
    DXGI_FORMAT,
    TEX_ALPHA_MODE,
    TEX_DIMENSION,
};

#[cfg_attr(not(feature = "windows"), path = "error_compat.rs")] pub(crate) mod error;

pub use error::{Error, Result};

mod cwide;
pub mod flags;
#[cfg(feature = "hwaccel")] pub(crate) mod hwaccel;
pub mod ops;
pub mod scratch;
mod types;
pub mod util;
pub use cwide::CWide;
pub use scratch::ScratchImage;
pub use types::{Blob, Image};
mod formats;
use flags::{DDS_FLAGS, TGA_FLAGS, WIC_FLAGS};
#[cfg(windows)] pub use formats::wic;
pub use formats::{dds, exr, hdr, tga};

pub fn compress_texture(
    format: DXGI_FORMAT,
    width: usize,
    height: usize,
    array_size: usize,
    mipmaps: usize,
    data: &[u8],
    flags: crate::flags::TEX_COMPRESS_FLAGS,
) -> Result<Vec<u8>> {
    let uncompressed = ScratchImage::new_2d(format, width, height, array_size, 1, data)?;
    let temp = if mipmaps > 1 {
        uncompressed.generate_mipmaps(mipmaps, flags::TEX_FILTER_FLAGS::default())?
    } else {
        uncompressed
    };

    let compressed = temp.compress(format, flags)?;

    Ok(compressed.buffer().to_vec())
}

pub fn decompress_texture(
    format: DXGI_FORMAT,
    width: usize,
    height: usize,
    array_size: usize,
    mipmaps: usize,
    data: &[u8],
) -> Result<Vec<u8>> {
    let compressed = ScratchImage::new_2d(format, width, height, array_size, mipmaps, data)?;
    let decompressed = compressed.decompress()?;

    Ok(decompressed.buffer().to_vec())
}

pub fn metadata(file_name: impl AsRef<Path>) -> Result<TexMetadata> {
    let file_name = file_name.as_ref();

    match file_name.extension().and_then(OsStr::to_str) {
        Some(ext) if ext.eq_ignore_ascii_case("dds") => {
            dds::metadata(file_name, DDS_FLAGS::default())
        }
        Some(ext) if ext.eq_ignore_ascii_case("tga") => {
            tga::metadata(file_name, TGA_FLAGS::default())
        }
        Some(ext) if ext.eq_ignore_ascii_case("hdr") => hdr::metadata(file_name),
        Some(ext) if ext.eq_ignore_ascii_case("exr") => exr::metadata(file_name),
        #[cfg(windows)]
        Some(_) => wic::metadata(file_name, WIC_FLAGS::default()),
        _ => Err(error::invalid_arg()),
    }
}

pub fn load(file_name: impl AsRef<Path>) -> Result<ScratchImage> {
    let file_name = file_name.as_ref();

    match file_name.extension().and_then(OsStr::to_str) {
        Some(ext) if ext.eq_ignore_ascii_case("dds") => dds::load(file_name, DDS_FLAGS::default()),
        Some(ext) if ext.eq_ignore_ascii_case("tga") => tga::load(file_name, TGA_FLAGS::default()),
        Some(ext) if ext.eq_ignore_ascii_case("hdr") => hdr::load(file_name),
        Some(ext) if ext.eq_ignore_ascii_case("exr") => exr::load(file_name),
        #[cfg(windows)]
        Some(_) => wic::load(file_name, WIC_FLAGS::default()),
        _ => Err(error::invalid_arg()),
    }
}

pub fn save(file_name: impl AsRef<Path>, image: &Image, metadata: &TexMetadata) -> Result<()> {
    let file_name = file_name.as_ref();

    match file_name.extension().and_then(OsStr::to_str) {
        Some(ext) if ext.eq_ignore_ascii_case("dds") => {
            dds::save(file_name, image, DDS_FLAGS::default())
        }
        Some(ext) if ext.eq_ignore_ascii_case("tga") => {
            tga::save(file_name, image, metadata, TGA_FLAGS::default())
        }
        Some(ext) if ext.eq_ignore_ascii_case("hdr") => hdr::save(file_name, image),
        Some(ext) if ext.eq_ignore_ascii_case("exr") => exr::save(file_name, image),
        #[cfg(windows)]
        Some(ext) => {
            if let Some(codec) = wic::wic_codec_by_ext(ext) {
                wic::save(file_name, codec, image, WIC_FLAGS::default())
            } else {
                Err(error::invalid_arg())
            }
        }
        _ => Err(error::invalid_arg()),
    }
}
