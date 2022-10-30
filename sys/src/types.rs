#[cfg(not(feature = "windows"))]
mod compat {
    #[repr(C)]
    #[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
    pub struct GUID {
        pub data1: u32,
        pub data2: u16,
        pub data3: u16,
        pub data4: [u8; 8],
    }

    pub type HRESULT = i32;
    pub type DXGI_FORMAT = u32;
    pub type D3D11_USAGE = u32;
}
#[cfg(not(feature = "windows"))] pub use compat::{D3D11_USAGE, DXGI_FORMAT, GUID, HRESULT};

#[repr(C)]
pub struct ID3D11Resource([u8; 0]);

#[repr(C)]
pub struct ID3D11Device([u8; 0]);

#[repr(C)]
pub struct ID3D11DeviceContext([u8; 0]);

#[repr(C)]
pub struct ID3D11ShaderResourceView([u8; 0]);

#[repr(C)]
pub struct IWICImagingFactory([u8; 0]);

#[repr(C)]
pub struct IWICMetadataQueryReader([u8; 0]);

#[repr(C)]
pub struct IPropertyBag2([u8; 0]);

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct XMVECTORF32(f32, f32, f32, f32);

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct XMVECTORI32(i32, i32, i32, i32);

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct XMVECTORU32(u32, u32, u32, u32);

pub type XMVECTOR = XMVECTORF32;
