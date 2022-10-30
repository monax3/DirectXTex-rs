use directxtex_sys as sys;

use crate::error::hresult;
use crate::flags::{TEX_COMPRESS_FLAGS, TEX_FILTER_FLAGS, TEX_FR_FLAGS, TEX_PMALPHA_FLAGS};
use crate::{Image, Result, ScratchImage};

pub fn generate_mipmaps(
    images: &[Image],
    metadata: &sys::TexMetadata,
    mipmaps: usize,
    flags: TEX_FILTER_FLAGS,
) -> Result<ScratchImage> {
    let mut out = sys::ScratchImage::default();

    hresult(unsafe {
        sys::GenerateMipMaps1(
            Image::slice_as_ptr(images),
            images.len(),
            metadata,
            flags,
            mipmaps,
            &mut out,
        )
    })
    .map(|_| ScratchImage(out))
}

#[cfg(feature = "windows")]
pub fn compress_with_device(
    device: &windows::Win32::Graphics::Direct3D11::ID3D11Device,
    images: &[Image],
    metadata: &sys::TexMetadata,
    to_format: sys::DXGI_FORMAT,
    flags: TEX_COMPRESS_FLAGS,
) -> Result<ScratchImage> {
    use windows::core::Vtable;

    unsafe {
        let device: *mut sys::ID3D11Device = device.as_raw().cast::<sys::ID3D11Device>();
        compress_with_device_ptr(device, images, metadata, to_format, flags)
    }
}

#[cfg(windows)]
pub unsafe fn compress_with_device_ptr(
    device: *mut sys::ID3D11Device,
    images: &[Image],
    metadata: &sys::TexMetadata,
    to_format: sys::DXGI_FORMAT,
    flags: TEX_COMPRESS_FLAGS,
) -> Result<ScratchImage> {
    let mut out = sys::ScratchImage::default();

    unsafe {
        hresult(sys::Compress3(
            device,
            Image::slice_as_ptr(images),
            images.len(),
            metadata,
            to_format,
            flags,
            sys::TEX_THRESHOLD_DEFAULT,
            &mut out,
        ))
        .map(|_| ScratchImage(out))
    }
}

pub fn compress(
    images: &[Image],
    metadata: &sys::TexMetadata,
    to_format: sys::DXGI_FORMAT,
    flags: TEX_COMPRESS_FLAGS,
) -> Result<ScratchImage> {
    #[cfg(feature = "hwaccel")]
    if crate::hwaccel::should_accel(to_format) {
        let hwdevice = crate::hwaccel::hwdevice()?;
        return compress_with_device(&hwdevice, images, metadata, to_format, flags);
    }

    let mut out = sys::ScratchImage::default();

    unsafe {
        hresult(sys::Compress1(
            Image::slice_as_ptr(images),
            images.len(),
            metadata,
            to_format,
            flags,
            sys::TEX_THRESHOLD_DEFAULT,
            &mut out,
        ))
        .map(|_| ScratchImage(out))
    }
}

#[inline]
pub fn resize(
    images: &[Image],
    metadata: &sys::TexMetadata,
    width: usize,
    height: usize,
    flags: TEX_FILTER_FLAGS,
) -> Result<ScratchImage> {
    let mut out = sys::ScratchImage::default();

    unsafe {
        hresult(sys::Resize1(
            Image::slice_as_ptr(images),
            images.len(),
            metadata,
            width,
            height,
            flags,
            &mut out,
        ))
        .map(|_| ScratchImage(out))
    }
}

#[inline]
pub fn decompress(images: &[Image], metadata: &sys::TexMetadata) -> Result<ScratchImage> {
    let mut out = sys::ScratchImage::default();

    unsafe {
        hresult(sys::Decompress1(
            Image::slice_as_ptr(images),
            images.len(),
            metadata,
            sys::DXGI_FORMAT::default(),
            &mut out,
        ))
        .map(|_| ScratchImage(out))
    }
}

#[inline]
pub fn convert(
    images: &[Image],
    metadata: &sys::TexMetadata,
    to_format: sys::DXGI_FORMAT,
    flags: TEX_FILTER_FLAGS,
) -> Result<ScratchImage> {
    let mut out = sys::ScratchImage::default();

    unsafe {
        hresult(sys::Convert1(
            Image::slice_as_ptr(images),
            images.len(),
            metadata,
            to_format,
            flags,
            sys::TEX_THRESHOLD_DEFAULT,
            &mut out,
        ))
        .map(|_| ScratchImage(out))
    }
}

#[inline]
pub fn premultiply_alpha(
    images: &[Image],
    metadata: &sys::TexMetadata,
    flags: TEX_PMALPHA_FLAGS,
) -> Result<ScratchImage> {
    let mut out = sys::ScratchImage::default();

    unsafe {
        hresult(sys::PremultiplyAlpha1(
            Image::slice_as_ptr(images),
            images.len(),
            metadata,
            flags,
            &mut out,
        ))
        .map(|_| ScratchImage(out))
    }
}

#[inline]
pub fn flip_rotate(
    images: &[Image],
    metadata: &sys::TexMetadata,
    flags: TEX_FR_FLAGS,
) -> Result<ScratchImage> {
    let mut out = sys::ScratchImage::default();

    unsafe {
        hresult(sys::FlipRotate1(
            Image::slice_as_ptr(images),
            images.len(),
            metadata,
            flags,
            &mut out,
        ))
        .map(|_| ScratchImage(out))
    }
}

unsafe extern "C" fn evaluate_func_impl<EVALFN>(
    pixels: *const sys::XMVECTORF32,
    width: usize,
    y: usize,
    userdata: *mut u8,
) where
    EVALFN: FnMut(&[sys::XMVECTORF32], usize),
{
    let func: *mut EVALFN = userdata.cast();
    let pixels = std::slice::from_raw_parts(pixels, width);
    (*func)(pixels, y);
}

#[inline]
pub fn evaluate_images<EVALFN>(
    images: &[Image],
    metadata: &sys::TexMetadata,
    mut evaluate_func: EVALFN,
) -> Result<()>
where
    EVALFN: FnMut(&[sys::XMVECTORF32], usize),
{
    let func: *mut EVALFN = &mut evaluate_func;
    hresult(unsafe {
        sys::EvaluateImage1(
            Image::slice_as_ptr(images),
            images.len(),
            metadata,
            Some(evaluate_func_impl::<EVALFN>),
            func.cast(),
        )
    })
}

unsafe extern "C" fn transform_func_impl<TRANSFN>(
    out_pixels: *mut sys::XMVECTORF32,
    in_pixels: *const sys::XMVECTORF32,
    width: usize,
    y: usize,
    userdata: *mut u8,
) where
    TRANSFN: FnMut(&mut [sys::XMVECTORF32], &[sys::XMVECTORF32], usize),
{
    let func: *mut TRANSFN = userdata.cast();
    let in_pixels = std::slice::from_raw_parts(in_pixels, width);
    let out_pixels = std::slice::from_raw_parts_mut(out_pixels, width);
    (*func)(out_pixels, in_pixels, y);
}

#[inline]
pub fn transform_images<TRANSFN>(
    images: &[Image],
    metadata: &sys::TexMetadata,
    mut transform_func: TRANSFN,
) -> Result<ScratchImage>
where
    TRANSFN: FnMut(&mut [sys::XMVECTORF32], &[sys::XMVECTORF32], usize),
{
    let mut out = sys::ScratchImage::default();

    let func: *mut TRANSFN = &mut transform_func;
    hresult(unsafe {
        sys::TransformImage1(
            Image::slice_as_ptr(images),
            images.len(),
            metadata,
            Some(transform_func_impl::<TRANSFN>),
            func.cast(),
            &mut out,
        )
    })
    .map(|_| ScratchImage(out))
}

// TODO:
// ConvertToSinglePlane
// GenerateMipMaps3D
// ScaleMipMapsAlphaForCoverage
// ComputeNormalMap
// CopyRectangle
// ComputeMSE
