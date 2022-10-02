#![allow(unsafe_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::items_after_statements)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::excessive_precision)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::used_underscore_binding)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::use_self)]
#![allow(clippy::transmute_ptr_to_ptr)]

mod impls;
mod types;

pub use types::{
    ID3D11Device,
    ID3D11DeviceContext,
    ID3D11Resource,
    ID3D11ShaderResourceView,
    IPropertyBag2,
    IWICImagingFactory,
    IWICMetadataQueryReader,
    XMVECTOR,
    XMVECTORF32,
    XMVECTORI32,
    XMVECTORU32,
};
#[cfg(not(feature = "windows"))] pub use types::{D3D11_USAGE, DXGI_FORMAT, GUID, HRESULT};
#[cfg(feature = "windows")]
pub use windows::{
    core::{GUID, HRESULT},
    Win32::Graphics::Direct3D11::D3D11_USAGE,
    Win32::Graphics::Dxgi::Common::DXGI_FORMAT,
};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
