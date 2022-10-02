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
    pub const fn len(&self) -> usize {
        self.0.m_size
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.m_size == 0
    }
}

pub struct Image<'scratch> {
    inner:  sys::Image,
    marker: PhantomData<&'scratch crate::ScratchImage>,
}

impl<'scratch> Image<'scratch> {
    #[inline]
    #[must_use]
    pub unsafe fn from_sys(image: sys::Image) -> Image<'scratch> {
        debug_assert_eq!(std::mem::size_of::<Self>(), std::mem::size_of::<sys::Image>());
        Self {
            inner:  image,
            marker: PhantomData,
        }
    }

    pub unsafe fn slice_from_ptr(ptr: *const sys::Image, count: usize) -> &'scratch [Self] {
        unsafe { std::slice::from_raw_parts(ptr.cast::<Self>(), count) }
    }

    #[inline]
    #[must_use]
    pub fn as_ptr(&self) -> *const sys::Image {
        &self.inner
    }

    #[inline]
    #[must_use]
    pub const fn buffer(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.inner.pixels.cast::<u8>(), self.inner.slicePitch) }
    }

    pub fn width(&self) -> usize { self.inner.width }

    pub fn height(&self) -> usize { self.inner.height }

    pub fn row_pitch(&self) -> usize { self.inner.rowPitch }

    pub fn size(&self) -> usize { self.inner.slicePitch }

    pub fn slice_pitch(&self) -> usize { self.inner.slicePitch }

    pub fn format(&self) -> sys::DXGI_FORMAT { self.inner.format }

    pub(crate) fn slice_as_sys<'a>(images: &'a [Image<'_>]) -> &'a [sys::Image] {
        let (_, aligned, _) = unsafe { images.align_to() };
        debug_assert_eq!(images.len(), aligned.len());
        aligned
    }
}
