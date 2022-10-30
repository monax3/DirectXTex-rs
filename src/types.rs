use std::marker::PhantomData;

use directxtex_sys as sys;

#[derive(Debug)]
pub struct Blob(pub(crate) sys::Blob);
impl Drop for Blob {
    fn drop(&mut self) { unsafe { self.0.Release() }; }
}

impl Blob {
    #[inline]
    #[must_use]
    pub const unsafe fn from_sys(blob: sys::Blob) -> Self { Self(blob) }

    #[inline]
    #[must_use]
    pub const fn buffer(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.0.m_buffer.cast::<u8>(), self.0.m_size) }
    }

    #[inline]
    #[must_use]
    pub const fn len(&self) -> usize { self.0.m_size }

    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.0.m_size == 0 }
}

#[repr(C)]
pub struct Image<'scratch> {
    inner:  sys::Image,
    marker: PhantomData<&'scratch crate::ScratchImage>,
}

impl<'scratch> Image<'scratch> {
    #[inline]
    #[must_use]
    pub const unsafe fn from_sys(image: sys::Image) -> Image<'scratch> {
        Self {
            inner:  image,
            marker: PhantomData,
        }
    }

    #[inline]
    #[must_use]
    pub unsafe fn from_ptr(ptr: *const sys::Image) -> Option<&'scratch Self> {
        ptr.cast::<Self>().as_ref()
    }

    #[inline]
    #[must_use]
    pub const unsafe fn slice_from_ptr(ptr: *const sys::Image, count: usize) -> &'scratch [Self] {
        std::slice::from_raw_parts(ptr.cast::<Self>(), count)
    }

    #[inline]
    #[must_use]
    pub const unsafe fn as_ptr(&self) -> *const sys::Image { &self.inner }

    #[inline]
    #[must_use]
    pub const unsafe fn slice_as_ptr(from: &[Image]) -> *const sys::Image {
        from.as_ptr().cast::<sys::Image>()
    }

    #[inline]
    #[must_use]
    pub const fn buffer(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.inner.pixels.cast::<u8>(), self.inner.slicePitch) }
    }

    #[inline]
    #[must_use]
    pub const fn width(&self) -> usize { self.inner.width }

    #[inline]
    #[must_use]
    pub const fn height(&self) -> usize { self.inner.height }

    #[inline]
    #[must_use]
    pub const fn row_pitch(&self) -> usize { self.inner.rowPitch }

    #[inline]
    #[must_use]
    pub const fn size(&self) -> usize { self.slice_pitch() }

    #[inline]
    #[must_use]
    pub const fn slice_pitch(&self) -> usize { self.inner.slicePitch }

    #[inline]
    #[must_use]
    pub const fn format(&self) -> sys::DXGI_FORMAT { self.inner.format }
}

#[allow(dead_code)]
const IMAGE_SIZE_IS_SYS_IMAGE_SIZE: [(); std::mem::size_of::<Image>()] =
    [(); std::mem::size_of::<sys::Image>()];
