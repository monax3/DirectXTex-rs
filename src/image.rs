use std::path::Path;

use directxtex_sys::{
    self as sys,
    TexMetadata,
    CP_FLAGS,
    DDS_FLAGS,
    DXGI_FORMAT,
    TEX_COMPRESS_FLAGS,
    TEX_FILTER_FLAGS,
    TEX_PMALPHA_FLAGS,
    TGA_FLAGS,
    WIC_FLAGS,
};

use crate::{Blob, Image, error};
use crate::io::{self, CWide};
use crate::util::is_compressed;
use crate::Result;
use crate::ops;

#[derive(Debug)]
pub struct ScratchImage(pub(crate) sys::ScratchImage);

impl Drop for ScratchImage {
    fn drop(&mut self) { unsafe { self.0.Release() }; }
}

impl ScratchImage {
    #[inline]
    #[must_use]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut sys::ScratchImage { &mut self.0 }

    #[inline]
    #[must_use]
    pub const unsafe fn as_ptr(&self) -> *const sys::ScratchImage { std::ptr::addr_of!(self.0) }

    #[inline]
    #[must_use]
    pub const fn format(&self) -> DXGI_FORMAT { self.metadata().format }

    #[inline]
    #[must_use]
    pub const fn metadata(&self) -> &TexMetadata { &self.0.m_metadata }

    #[inline]
    pub fn load(file_name: impl AsRef<Path>) -> Result<Self> { crate::io::load(file_name) }

    #[inline]
    pub fn save(&self, file_name: impl AsRef<Path>, item: usize) -> Result<()> {
        let image = self.image(0, item, 0).ok_or_else(error::invalid_arg)?;
        io::save(file_name, image, self.metadata())
    }

    #[inline]
    pub fn new(
        format: DXGI_FORMAT,
        width: usize,
        height: usize,
        array_size: usize,
        mipmaps: usize,
        data: &[u8],
    ) -> Result<Self> {
        if height > 1 {
            Self::new_2d(format, width, height, array_size, mipmaps, data)
        } else {
            Self::new_1d(format, width, array_size, mipmaps, data)
        }
    }

    #[inline]
    pub fn new_1d(
        format: DXGI_FORMAT,
        size: usize,
        array_size: usize,
        mipmaps: usize,
        data: &[u8],
    ) -> Result<Self> {
        let mut raw = sys::ScratchImage::default();

        let mut scratch =
            unsafe { raw.Initialize1D(format, size, array_size, mipmaps, CP_FLAGS::default()) }
                .ok()
                .map(|_| Self(raw))?;

        scratch.copy_from_slice(data)?;
        Ok(scratch)
    }

    #[inline]
    pub fn new_2d(
        format: DXGI_FORMAT,
        width: usize,
        height: usize,
        array_size: usize,
        mipmaps: usize,
        data: &[u8],
    ) -> Result<Self> {
        let mut raw = sys::ScratchImage::default();

        let mut scratch = unsafe {
            raw.Initialize2D(
                format,
                width,
                height,
                array_size,
                mipmaps,
                CP_FLAGS::default(),
            )
        }
        .ok()
        .map(|_| Self(raw))?;

        scratch.copy_from_slice(data)?;
        Ok(scratch)
    }

    #[inline]
    #[must_use]
    pub const fn width(&self) -> usize { self.metadata().width }

    #[inline]
    #[must_use]
    pub const fn height(&self) -> usize { self.metadata().height }

    #[inline]
    #[must_use]
    pub const fn array_size(&self) -> usize { self.metadata().arraySize }

    #[inline]
    #[must_use]
    pub const fn mip_levels(&self) -> usize { self.metadata().mipLevels }

    #[inline]
    pub fn copy_from_slice(&mut self, data: &[u8]) -> Result<()> {
        let buffer = self.buffer_mut();
        if buffer.len() == data.len() {
            buffer.copy_from_slice(data);
            Ok(())
        } else {
            Err(error::invalid_arg())
        }
    }

    // Helper for extracting intermediaries when chaining actions
    #[inline]
    pub fn inspect(self, func: impl FnOnce(&Self) -> Result<()>) -> Result<Self> {
        func(&self)?;
        Ok(self)
    }

    // Helper for chaining actions conditionally
    #[inline]
    pub fn map_if(self, condition: bool, func: impl FnOnce(&Self) -> Result<Self>) -> Result<Self> {
        if condition { func(&self) } else { Ok(self) }
    }

    #[inline]
    pub fn generate_mipmaps(&self, mipmaps: usize, flags: TEX_FILTER_FLAGS) -> Result<Self> {
        ops::generate_mipmaps(self.images(), self.metadata(), mipmaps, flags)
    }

    #[inline]
    pub fn override_format(&mut self, format: DXGI_FORMAT) -> Result<()> {
        unsafe { self.0.OverrideFormat(format).then_some(()).ok_or_else(fail) }
    }

    #[inline]
    #[must_use]
    pub fn image(&self, mip: usize, item: usize, slice: usize) -> Option<&sys::Image> {
        unsafe { self.0.GetImage(mip, item, slice).as_ref() }
    }

    #[cfg(feature = "windows")]
    pub fn compress_with_device(
        &self,
        device: &windows::Win32::Graphics::Direct3D11::ID3D11Device,
        to_format: DXGI_FORMAT,
        flags: TEX_COMPRESS_FLAGS,
    ) -> Result<Self> {
        unsafe {
            let device: *mut sys::ID3D11Device = std::mem::transmute_copy(device);
            ops::compress_with_device_ptr(device, self.images(), self.metadata(), to_format, flags)
        }
    }

    pub unsafe fn compress_with_device_ptr(
        &self,
        device: *mut sys::ID3D11Device,
        to_format: DXGI_FORMAT,
        flags: TEX_COMPRESS_FLAGS,
    ) -> Result<Self> {
        ops::compress_with_device_ptr(device, self.images(), self.metadata(), to_format, flags)
    }

    pub fn compress(&self, to_format: DXGI_FORMAT, flags: TEX_COMPRESS_FLAGS) -> Result<Self> {
        ops::compress(self.images(), self.metadata(), to_format, flags)
    }

    #[inline]
    pub fn resize(&self, width: usize, height: usize, flags: TEX_FILTER_FLAGS) -> Result<Self> {
        ops::resize(self.images(), self.metadata(), width, height, flags)
    }

    #[inline]
    pub fn decompress(&self) -> Result<Self> {
        ops::decompress(self.images(), self.metadata())
    }

    #[inline]
    pub fn convert(&self, to_format: DXGI_FORMAT, flags: TEX_FILTER_FLAGS) -> Result<Self> {
        ops::convert(self.images(), self.metadata(), to_format, flags)
    }

    pub fn step_into_format(self, to_format: DXGI_FORMAT) -> Result<Self> {
        let format = self.metadata().format;
        if format == to_format {
            Ok(self)
        } else if is_compressed(format) {
            self.decompress()
        } else if is_compressed(to_format) {
            self.compress(to_format, TEX_COMPRESS_FLAGS::default())
        } else {
            self.convert(to_format, TEX_FILTER_FLAGS::default())
        }
    }

    pub fn maybe_decompress(self) -> Result<Self> {
        if is_compressed(self.format()) {
            self.decompress()
        } else {
            Ok(self)
        }
    }

    pub fn convert_or_compress(self, to_format: DXGI_FORMAT) -> Result<Self> {
        if self.format() == to_format {
            Ok(self)
        } else if is_compressed(to_format) {
            self.compress(to_format, TEX_COMPRESS_FLAGS::default())
        } else {
            self.convert(to_format, TEX_FILTER_FLAGS::default())
        }
    }

    #[inline]
    pub fn into_format(self, to_format: DXGI_FORMAT) -> Result<Self> {
        self.maybe_decompress()?.convert_or_compress(to_format)
    }

    #[inline]
    pub fn into_converted(self, to_format: DXGI_FORMAT, flags: TEX_FILTER_FLAGS) -> Result<Self> {
        let metadata = self.metadata();
        if metadata.format == to_format {
            Ok(self)
        } else {
            self.convert(to_format, flags)
        }
    }

    #[inline]
    pub fn premultiply_alpha(&self, flags: TEX_PMALPHA_FLAGS) -> Result<Self> {
        ops::premultiply_alpha(self.images(), self.metadata(), flags)
    }

    // #[inline]
    // #[must_use]
    // pub const fn images(&self) -> &[sys::Image] {
    //     // Safety: These are always valid after the ScratchImage is initialized
    //     unsafe { std::slice::from_raw_parts(self.0.m_image, self.0.m_nimages) }
    // }

    #[inline]
    #[must_use]
    pub const fn images(&self) -> &[Image] {
        // Safety: These are always valid after the ScratchImage is initialized
        unsafe { Image::slice_from_ptr(self.0.m_image, self.0.m_nimages) }
    }

    #[must_use]
    #[inline]
    pub const fn buffer_size(&self) -> usize { self.0.m_size }

    #[inline]
    #[must_use]
    pub fn buffer(&self) -> &[u8] {
        // Safety: These are always valid after the ScratchImage is initialized
        assert!(!self.0.m_memory.is_null());
        unsafe { std::slice::from_raw_parts(self.0.m_memory, self.0.m_size) }
    }

    #[inline]
    #[must_use]
    pub fn buffer_mut(&mut self) -> &mut [u8] {
        // Safety: These are always valid after the ScratchImage is initialized
        assert!(!self.0.m_memory.is_null());
        unsafe { std::slice::from_raw_parts_mut(self.0.m_memory, self.0.m_size) }
    }

    #[inline]
    #[must_use]
    pub const fn num_images(&self) -> usize { self.0.m_nimages }

    #[inline]
    #[must_use]
    pub fn image_size(&self, mip: usize, item: usize, slice: usize) -> Option<usize> {
        self.image(mip, item, slice).map(|image| image.slicePitch)
    }

    #[inline]
    #[must_use]
    pub fn image_buffer(&self, mip: usize, item: usize, slice: usize) -> Option<&[u8]> {
        self.image(mip, item, slice)
            .map(|image| unsafe { std::slice::from_raw_parts(image.pixels, image.slicePitch) })
    }

    pub fn save_dds<'a, CWIDE>(&self, file_name: CWIDE, dds_flags: DDS_FLAGS) -> Result<()>
    where
        CWide<'a>: TryFrom<CWIDE>,
        crate::Error: From<<CWide<'a> as TryFrom<CWIDE>>::Error>,
    {
        io::dds::save_slice(file_name, self.images(), self.metadata(), dds_flags)
    }

    pub fn save_dds_to_memory(&self, dds_flags: DDS_FLAGS) -> Result<Blob> {
        io::dds::save_slice_to_memory(self.images(), self.metadata(), dds_flags)
    }

    pub fn save_tga<'a, CWIDE>(
        &self,
        item: usize,
        file_name: CWIDE,
        tga_flags: TGA_FLAGS,
    ) -> Result<()>
    where
        CWide<'a>: TryFrom<CWIDE>,
        crate::Error: From<<CWide<'a> as TryFrom<CWIDE>>::Error>,
    {
        let image = self.images().get(item).ok_or_else(invalid_arg)?;
        io::tga::save(file_name, image, self.metadata(), tga_flags)
    }

    pub fn save_tga_to_memory(&self, item: usize, tga_flags: TGA_FLAGS) -> Result<Blob> {
        let image = self.images().get(item).ok_or_else(invalid_arg)?;
        io::tga::save_to_memory(image, self.metadata(), tga_flags)
    }

    pub fn save_hdr<'a, CWIDE>(&self, item: usize, file_name: CWIDE) -> Result<()>
    where
        CWide<'a>: TryFrom<CWIDE>,
        crate::Error: From<<CWide<'a> as TryFrom<CWIDE>>::Error>,
    {
        let image = self.images().get(item).ok_or_else(invalid_arg)?;
        io::hdr::save(file_name, image)
    }

    pub fn save_hdr_to_memory(&self, item: usize) -> Result<Blob> {
        let image = self.images().get(item).ok_or_else(invalid_arg)?;
        io::hdr::save_to_memory(image)
    }

    pub fn save_exr<'a, CWIDE>(&self, item: usize, file_name: CWIDE) -> Result<()>
    where
        CWide<'a>: TryFrom<CWIDE>,
        crate::Error: From<<CWide<'a> as TryFrom<CWIDE>>::Error>,
    {
        let image = self.images().get(item).ok_or_else(invalid_arg)?;
        io::exr::save(file_name, image)
    }

    pub fn save_wic<'a, CWIDE>(
        &self,
        item: usize,
        file_name: CWIDE,
        container: crate::WICCodecs,
        wic_flags: WIC_FLAGS,
    ) -> Result<()>
    where
        CWide<'a>: TryFrom<CWIDE>,
        crate::Error: From<<CWide<'a> as TryFrom<CWIDE>>::Error>,
    {
        let image = self.images().get(item).ok_or_else(invalid_arg)?;
        io::wic::save(file_name, container, image, wic_flags)
    }

    pub fn save_wic_to_memory(
        &self,
        item: usize,
        container: crate::WICCodecs,
        wic_flags: WIC_FLAGS,
    ) -> Result<Blob> {
        let image = self.images().get(item).ok_or_else(invalid_arg)?;
        io::wic::save_to_memory(container, image, wic_flags)
    }
}

impl Clone for ScratchImage {
    fn clone(&self) -> Self {
        Self::new(
            self.format(),
            self.width(),
            self.height(),
            self.array_size(),
            self.mip_levels(),
            self.buffer(),
        )
        .expect("Clone failed")
    }
}
