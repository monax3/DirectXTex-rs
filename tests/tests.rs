use std::env;
use std::path::PathBuf;

use directxtex::flags::{DDS_FLAGS, TEX_FILTER_FLAGS, TGA_FLAGS};
#[cfg(feature = "windows")] use directxtex::util::initialize_com;
use directxtex::{dds, tga, ScratchImage};
#[cfg(feature = "windows")] use directxtex::{flags::WIC_FLAGS, wic, wic::WIC_CODEC_JPEG};

#[cfg(feature = "windows")]
const RGBA8: directxtex_sys::DXGI_FORMAT =
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R8G8B8A8_UNORM;
#[cfg(not(feature = "windows"))]
const RGBA8: directxtex_sys::DXGI_FORMAT =
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R8G8B8A8_UNORM.0;

#[cfg(feature = "windows")]
const RGBAF32: directxtex_sys::DXGI_FORMAT =
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R32G32B32A32_FLOAT;
#[cfg(not(feature = "windows"))]
const RGBAF32: directxtex_sys::DXGI_FORMAT =
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R32G32B32A32_FLOAT.0;

#[cfg(feature = "hwaccel")]
const COMPRESSED: directxtex_sys::DXGI_FORMAT =
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_BC7_UNORM;
#[cfg(all(feature = "windows", not(feature = "hwaccel")))]
const COMPRESSED: directxtex_sys::DXGI_FORMAT =
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_BC1_UNORM;
#[cfg(not(feature = "windows"))]
const COMPRESSED: directxtex_sys::DXGI_FORMAT =
    windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_BC1_UNORM.0;

fn in_file(file_name: &str) -> PathBuf {
    PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .join("sys/tests")
        .join(file_name)
}

fn out_file(file_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(file_name)
}

#[test]
fn test_tga() {
    #[cfg(feature = "windows")]
    initialize_com().expect("Failed to initialize COM");

    let tga = tga::load(in_file("test.tga"), TGA_FLAGS::default()).expect("Failed to load TGA");
    tga.save_tga(0, out_file("test.tga"), TGA_FLAGS::default())
        .expect("Failed to save TGA");
}

#[test]
fn test_simple() {
    let compressed = dds::load(in_file("test_complex.dds"), DDS_FLAGS::default())
        .unwrap()
        .maybe_decompress()
        .unwrap()
        .compress(COMPRESSED, directxtex::flags::TEX_COMPRESS_FLAGS::default())
        .unwrap();
    assert!(compressed.is_compressed());
    compressed
        .save_dds(out_file("test.dds"), DDS_FLAGS::default())
        .expect("Failed to save as DDS");
}

#[cfg(feature = "windows")]
#[test]
fn test_wic() {
    initialize_com().expect("Failed to initialize COM");

    let png =
        wic::load(in_file("test.png"), WIC_FLAGS::default()).expect("Failed to load PNG via WIC");

    png.save_wic(
        0,
        out_file("test.jpg"),
        WIC_CODEC_JPEG,
        WIC_FLAGS::default(),
    )
    .expect("Failed to save as JPEG via WIC");
}

#[test]
fn test_hdr() {
    #[cfg(feature = "windows")]
    initialize_com().expect("Failed to initialize COM");

    ScratchImage::load(in_file("test.hdr"))
        .expect("Failed to load HDR")
        .convert(RGBA8, TEX_FILTER_FLAGS::default())
        .expect("Failed to convert from RGBAF32")
        .save(out_file("test.tga"), 0)
        .unwrap();

    ScratchImage::load(in_file("test.dds"))
        .expect("Failed to load DDS")
        .maybe_decompress()
        .unwrap()
        .convert(RGBAF32, TEX_FILTER_FLAGS::default())
        .expect("Failed to convert to RGBAF32")
        .save(out_file("test.hdr"), 0)
        .unwrap();
}

#[test]
fn test_step() {
    #[cfg(feature = "windows")]
    initialize_com().expect("Failed to initialize COM");

    let mut image =
        dds::load(in_file("test_complex.dds"), DDS_FLAGS::default()).expect("Failed to load DDS");

    let to_format = COMPRESSED;
    while image.format() != to_format {
        image = image
            .step_into_format(to_format)
            .expect("Compression step failed");
    }

    let to_format = RGBA8;
    while image.format() != to_format {
        image = image
            .step_into_format(to_format)
            .expect("RGBA u8 conversion step failed");
    }

    let to_format = RGBAF32;
    while image.format() != to_format {
        image = image
            .step_into_format(to_format)
            .expect("RGBA f32 step failed");
    }

    image
        .save_dds(out_file("test_complex.dds"), DDS_FLAGS::default())
        .expect("Failed to save as DDS");
}

#[test]
fn test_into_format() {
    #[cfg(feature = "windows")]
    initialize_com().expect("Failed to initialize COM");

    let image = dds::load(in_file("test_complex.dds"), DDS_FLAGS::default())
        .expect("Failed to load DDS")
        .into_format(RGBA8)
        .expect("Failed into_format(RGBA8)")
        .into_format(COMPRESSED)
        .expect("Failed into_format(BC7)")
        .into_format(RGBAF32)
        .expect("Failed into_format(RGBA32f")
        .into_format(RGBA8)
        .expect("Failed into_format(RGBA8)");

    let blob = image
        .save_tga_to_memory(0, TGA_FLAGS::default())
        .expect("Failed to save to memory as TGA");
    assert!(!blob.is_empty());

    #[cfg(feature = "windows")]
    image
        .save_wic(
            0,
            out_file("test.jpg"),
            WIC_CODEC_JPEG,
            WIC_FLAGS::default(),
        )
        .expect("Failed to save as JPEG");

    image
        .save_tga(0, out_file("test.tga"), TGA_FLAGS::default())
        .expect("Failed to save as TGA");
}

#[test]
fn test_buffers() {
    #[cfg(feature = "windows")]
    initialize_com().expect("Failed to initialize COM");

    let image = dds::load(in_file("test.dds"), DDS_FLAGS::default()).expect("Failed to load DDS");

    let buffer = image.buffer();

    let image2 = ScratchImage::new_2d(
        image.format(),
        image.width(),
        image.height(),
        image.array_size(),
        image.mip_levels(),
        buffer,
    )
    .expect("Failed to make new image from buffer");
    let image3 = image.clone();

    let out1 = image
        .save_dds_to_memory(DDS_FLAGS::default())
        .expect("Failed to save original as DDS");
    let out2 = image2
        .save_dds_to_memory(DDS_FLAGS::default())
        .expect("Failed to save copy as DDS");
    let out3 = image3
        .save_dds_to_memory(DDS_FLAGS::default())
        .expect("Failed to save clone as DDS");

    assert_eq!(out1.buffer(), out2.buffer());
    assert_eq!(out1.buffer(), out3.buffer());
}

#[test]
fn test_evaluate() {
    #[cfg(feature = "windows")]
    initialize_com().expect("Failed to initialize COM");

    let image = dds::load(in_file("test.dds"), DDS_FLAGS::default()).expect("Failed to load DDS");
    let images = image.images();
    assert_eq!(1, images.len());

    let mut count = 0;
    directxtex::ops::evaluate_images(images, image.metadata(), |_pixels, _y| {
        count += 1;
    })
    .unwrap();

    assert_eq!(count, image.height());
}
