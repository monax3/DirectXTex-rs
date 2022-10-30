//! Missing:
//! `directxtex_bindgen::FormatDataType`

use directxtex_sys::{self as sys, CP_FLAGS, DXGI_FORMAT};

use crate::error::hresult;
use crate::Result;

#[inline]
#[must_use]
pub fn is_compressed(format: DXGI_FORMAT) -> bool { unsafe { sys::IsCompressed(format) } }

#[inline]
#[must_use]
pub fn is_srgb(format: DXGI_FORMAT) -> bool { unsafe { sys::IsSRGB(format) } }

#[inline]
#[must_use]
pub fn has_alpha(format: DXGI_FORMAT) -> bool { unsafe { sys::HasAlpha(format) } }

#[inline]
#[must_use]
pub fn is_bgr(format: DXGI_FORMAT) -> bool { unsafe { sys::IsBGR(format) } }

#[inline]
#[must_use]
pub fn is_depth_stencil(format: DXGI_FORMAT) -> bool { unsafe { sys::IsDepthStencil(format) } }

#[inline]
#[must_use]
pub fn is_packed(format: DXGI_FORMAT) -> bool { unsafe { sys::IsPacked(format) } }

#[inline]
#[must_use]
pub fn is_palettized(format: DXGI_FORMAT) -> bool { unsafe { sys::IsPalettized(format) } }

#[inline]
#[must_use]
pub fn is_planar(format: DXGI_FORMAT) -> bool { unsafe { sys::IsPlanar(format) } }

#[inline]
#[must_use]
pub fn is_typeless(format: DXGI_FORMAT, partial: bool) -> bool {
    unsafe { sys::IsTypeless(format, partial) }
}

#[inline]
#[must_use]
pub fn is_video(format: DXGI_FORMAT) -> bool { unsafe { sys::IsVideo(format) } }

#[inline]
#[must_use]
pub fn make_linear(format: DXGI_FORMAT) -> DXGI_FORMAT { unsafe { sys::MakeLinear(format) } }

#[inline]
#[must_use]
pub fn make_srgb(format: DXGI_FORMAT) -> DXGI_FORMAT { unsafe { sys::MakeSRGB(format) } }

#[inline]
#[must_use]
pub fn make_typelesss(format: DXGI_FORMAT) -> DXGI_FORMAT { unsafe { sys::MakeTypeless(format) } }

#[inline]
#[must_use]
pub fn make_typelesss_float(format: DXGI_FORMAT) -> DXGI_FORMAT {
    unsafe { sys::MakeTypelessFLOAT(format) }
}

#[inline]
#[must_use]
pub fn make_typelesss_unorm(format: DXGI_FORMAT) -> DXGI_FORMAT {
    unsafe { sys::MakeTypelessUNORM(format) }
}

#[inline]
#[must_use]
pub fn bits_per_pixel(format: DXGI_FORMAT) -> usize { unsafe { sys::BitsPerPixel(format) } }

#[inline]
#[must_use]
pub fn bits_per_color(format: DXGI_FORMAT) -> usize { unsafe { sys::BitsPerColor(format) } }

#[inline]
pub fn compute_pitch(
    format: DXGI_FORMAT,
    width: usize,
    height: usize,
    cp_flags: CP_FLAGS,
) -> Result<(usize, usize)> {
    let mut row_pitch: usize = 0;
    let mut slice_pitch: usize = 0;

    hresult(unsafe {
        sys::ComputePitch(
            format,
            width,
            height,
            &mut row_pitch,
            &mut slice_pitch,
            cp_flags,
        )
    })
    .map(|_| (row_pitch, slice_pitch))
}

#[inline]
#[must_use]
pub fn compute_scanlines(format: DXGI_FORMAT, height: usize) -> usize {
    unsafe { sys::ComputeScanlines(format, height) }
}

#[inline]
pub fn expected_buffer(
    format: DXGI_FORMAT,
    width: usize,
    height: usize,
    array_size: usize,
    mipmaps: usize,
) -> Result<usize> {
    let mut raw = sys::ScratchImage::default();

    hresult(unsafe {
        raw.Initialize2D(
            format,
            width,
            height,
            array_size,
            mipmaps,
            CP_FLAGS::default(),
        )
    })
    // Make sure our temporary ScratchImage has its drop handler
    .map(|_| crate::ScratchImage(raw))
    // ...and return only buffer size that was allocated in it
    .map(|scratch| scratch.0.m_size)
}

#[cfg(feature = "windows")]
pub fn initialize_com() -> Result<()> {
    use windows::Win32::System::Com::{
        CoInitializeEx,
        COINIT_DISABLE_OLE1DDE,
        COINIT_MULTITHREADED,
    };

    unsafe { CoInitializeEx(None, COINIT_MULTITHREADED | COINIT_DISABLE_OLE1DDE) }
}
