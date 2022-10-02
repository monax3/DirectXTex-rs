use std::ffi::OsStr;
use std::path::Path;

use directxtex_sys::{self as sys, TexMetadata, DDS_FLAGS, TGA_FLAGS, WIC_FLAGS};

use crate::error::invalid_arg;
use crate::image::ScratchImage;
use crate::Result;

mod cwide;
pub use cwide::CWide;
pub mod dds;
pub mod exr;
pub mod hdr;
pub mod tga;
#[cfg(windows)] pub mod wic;

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
        _ => Err(invalid_arg()),
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
        _ => Err(invalid_arg()),
    }
}

pub fn save(file_name: impl AsRef<Path>, image: &sys::Image, metadata: &TexMetadata) -> Result<()> {
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
                Err(invalid_arg())
            }
        }
        _ => Err(invalid_arg()),
    }
}
