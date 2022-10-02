use directxtex_sys as sys;

use crate::flags::{TEX_FILTER_FLAGS, TEX_COMPRESS_FLAGS, TEX_PMALPHA_FLAGS};
use crate::{Result, ScratchImage, Image};

pub fn generate_mipmaps(
    images: &[sys::Image],
    metadata: &sys::TexMetadata,
    mipmaps: usize,
    flags: TEX_FILTER_FLAGS,
) -> Result<ScratchImage> {
    let mut out = sys::ScratchImage::default();

    unsafe {
        sys::GenerateMipMaps1(
            images.as_ptr(),
            images.len(),
            metadata,
            flags,
            mipmaps,
            &mut out,
        )
        .ok()
        .map(|_| ScratchImage(out))
    }
}

#[cfg(feature = "windows")]
pub fn compress_with_device(
    device: &windows::Win32::Graphics::Direct3D11::ID3D11Device,
    images: &[sys::Image],
    metadata: &sys::TexMetadata,
    to_format: sys::DXGI_FORMAT,
    flags: TEX_COMPRESS_FLAGS,
) -> Result<ScratchImage> {
    unsafe {
        let device: *mut sys::ID3D11Device = std::mem::transmute_copy(device);
        compress_with_device_ptr(device, images, metadata, to_format, flags)
    }
}

pub unsafe fn compress_with_device_ptr(
    device: *mut sys::ID3D11Device,
    images: &[sys::Image],
    metadata: &sys::TexMetadata,
    to_format: sys::DXGI_FORMAT,
    flags: TEX_COMPRESS_FLAGS,
) -> Result<ScratchImage> {
    let mut out = sys::ScratchImage::default();

    unsafe {
        sys::Compress3(
            device,
            images.as_ptr(),
            images.len(),
            metadata,
            to_format,
            flags,
            sys::TEX_THRESHOLD_DEFAULT,
            &mut out,
        )
        .ok()
        .map(|_| ScratchImage(out))
    }
}


pub fn compress(images: &[sys::Image], metadata: &sys::TexMetadata, to_format: sys::DXGI_FORMAT, flags: TEX_COMPRESS_FLAGS) -> Result<ScratchImage> {
    #[cfg(feature = "hwaccel")]
    if crate::hwaccel::should_accel(to_format) {
        let hwdevice = crate::hwaccel::hwdevice()?;
        return compress_with_device(&hwdevice, images, metadata, to_format, flags);
    }

    let mut out = sys::ScratchImage::default();

    unsafe {
        sys::Compress1(
            images.as_ptr(),
            images.len(),
            metadata,
            to_format,
            flags,
            sys::TEX_THRESHOLD_DEFAULT,
            &mut out,
        )
        .ok()
        .map(|_| ScratchImage(out))
    }
}

#[inline]
pub fn resize(images: &[sys::Image], metadata: &sys::TexMetadata, width: usize, height: usize, flags: TEX_FILTER_FLAGS) -> Result<ScratchImage> {
    let mut out = sys::ScratchImage::default();

    unsafe {
        sys::Resize1(
            images.as_ptr(),
            images.len(),
            metadata,
            width,
            height,
            flags,
            &mut out,
        )
        .ok()
        .map(|_| ScratchImage(out))
    }
}

#[inline]
pub fn decompress(images: &[sys::Image], metadata: &sys::TexMetadata) -> Result<ScratchImage> {
    let mut out = sys::ScratchImage::default();

    unsafe {
        sys::Decompress1(
            images.as_ptr(),
            images.len(),
            metadata,
            sys::DXGI_FORMAT::default(),
            &mut out,
        )
        .ok()
        .map(|_| ScratchImage(out))
    }
}

#[inline]
pub fn convert(images: &[Image], metadata: &sys::TexMetadata, to_format: sys::DXGI_FORMAT, flags: TEX_FILTER_FLAGS) -> Result<ScratchImage> {
    let mut out = sys::ScratchImage::default();

    unsafe {
        sys::Convert1(
            images.as_ptr().cast::<sys::Image>(),
            images.len(),
            metadata,
            to_format,
            flags,
            sys::TEX_THRESHOLD_DEFAULT,
            &mut out,
        )
        .ok()
        .map(|_| ScratchImage(out))
    }
}


#[inline]
pub fn premultiply_alpha(images: &[sys::Image], metadata: &sys::TexMetadata, flags: TEX_PMALPHA_FLAGS) -> Result<ScratchImage> {
    let mut out = sys::ScratchImage::default();

    unsafe {
        sys::PremultiplyAlpha1(
            images.as_ptr(),
            images.len(),
            metadata,
            flags,
            &mut out,
        )
        .ok()
        .map(|_| ScratchImage(out))
    }
}
