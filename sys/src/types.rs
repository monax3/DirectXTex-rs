#[cfg(not(feature = "windows"))]
mod compat {
    #[repr(C)]
    pub struct GUID([u8; 0]);

    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct HRESULT(pub i32);

    impl HRESULT {
        pub fn ok(self) -> std::result::Result<(), Self> {
            match self.0 {
                0 => Ok(()),
                error => Err(HRESULT(error)),
            }
        }
    }

    impl From<std::convert::Infallible> for HRESULT {
        fn from(_: std::convert::Infallible) -> Self { unimplemented!() }
    }

    impl std::error::Error for HRESULT {}

    impl std::fmt::Debug for HRESULT {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "HRESULT({:08x})", self.0 as u32)
        }
    }

    impl std::fmt::Display for HRESULT {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "HRESULT({:08x})", self.0 as u32)
        }
    }

    #[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct DXGI_FORMAT(pub u32);

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
