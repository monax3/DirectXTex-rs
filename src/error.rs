#![cfg(feature = "windows")]

pub use windows::core::{Error, Result};
 use windows::Win32::Foundation::{E_FAIL, E_INVALIDARG};

pub fn invalid_arg() -> Error { E_INVALIDARG.into() }
pub fn fail() -> Error { E_FAIL.into() }

pub fn hresult(hr: windows::core::HRESULT) -> Result<()> {
    hr.ok()
}
