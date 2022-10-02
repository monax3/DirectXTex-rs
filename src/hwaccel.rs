use directxtex_sys::DXGI_FORMAT;
use once_cell::sync::OnceCell;
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::Graphics::Direct3D::{
    D3D_DRIVER_TYPE_HARDWARE,
    D3D_FEATURE_LEVEL,
    D3D_FEATURE_LEVEL_10_0,
    D3D_FEATURE_LEVEL_10_1,
    D3D_FEATURE_LEVEL_11_0,
};
use windows::Win32::Graphics::Direct3D11::{
    D3D11CreateDevice,
    ID3D11Device,
    D3D11_CREATE_DEVICE_FLAG,
    D3D11_SDK_VERSION,
};
use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_FORMAT_BC6H_SF16,
    DXGI_FORMAT_BC6H_TYPELESS,
    DXGI_FORMAT_BC6H_UF16,
    DXGI_FORMAT_BC7_TYPELESS,
    DXGI_FORMAT_BC7_UNORM,
    DXGI_FORMAT_BC7_UNORM_SRGB,
};

use crate::Result;

static HW_FORMATS: &[DXGI_FORMAT] = &[
    DXGI_FORMAT_BC6H_TYPELESS,
    DXGI_FORMAT_BC6H_UF16,
    DXGI_FORMAT_BC6H_SF16,
    DXGI_FORMAT_BC7_TYPELESS,
    DXGI_FORMAT_BC7_UNORM,
    DXGI_FORMAT_BC7_UNORM_SRGB,
];

static FEATURE_LEVELS: &[D3D_FEATURE_LEVEL] = &[
    D3D_FEATURE_LEVEL_11_0,
    D3D_FEATURE_LEVEL_10_1,
    D3D_FEATURE_LEVEL_10_0,
];

pub fn should_accel(format: DXGI_FORMAT) -> bool { HW_FORMATS.contains(&format) }

static HWDEVICE: OnceCell<ID3D11Device> = OnceCell::new();

fn try_create_device() -> Result<ID3D11Device> {
    let mut device = std::mem::MaybeUninit::zeroed();

    let device = unsafe {
        D3D11CreateDevice(
            None,
            D3D_DRIVER_TYPE_HARDWARE,
            None,
            D3D11_CREATE_DEVICE_FLAG::default(),
            Some(FEATURE_LEVELS),
            D3D11_SDK_VERSION,
            Some(device.as_mut_ptr()),
            None,
            None,
        )?;
        device.assume_init()
    };

    device.ok_or_else(|| E_FAIL.into())
}

pub fn hwdevice() -> Result<ID3D11Device> {
    let device = HWDEVICE.get_or_try_init(try_create_device)?;
    Ok(device.clone())
}
