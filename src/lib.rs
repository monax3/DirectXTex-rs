#![allow(unsafe_code)]

use directxtex_sys as sys;
pub use sys::{
    Blob as RawBlob,
    Rect,
    Image as RawImage,
    ScratchImage as RawScratchImage,
    TexMetadata,
    WICCodecs,
    DXGI_FORMAT,
    TEX_ALPHA_MODE,
    TEX_DIMENSION,
};

pub(crate) mod error;
pub use error::{Error, Result};

mod types;
pub mod flags;
#[cfg(feature = "hwaccel")] pub(crate) mod hwaccel;
pub mod image;
pub mod io;
pub mod util;
pub mod ops;
pub use image::ScratchImage;
pub use types::{Blob, Image};

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
