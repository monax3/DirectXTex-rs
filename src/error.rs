#![allow(overflowing_literals)]
#![allow(clippy::useless_conversion)]

#[cfg(feature = "windows")]
#[cfg(feature = "windows")]
mod error_impl {
    pub use windows::core::{Error, Result};
    pub(super) use windows::Win32::Foundation::{E_FAIL, E_INVALIDARG};
}

#[cfg(not(feature = "windows"))]
#[allow(overflowing_literals)]
mod error_impl {
    use directxtex_sys::HRESULT;
    pub use directxtex_sys::HRESULT as Error;

    pub type Result<T> = std::result::Result<T, Error>;

    pub(super) const E_INVALIDARG: HRESULT = HRESULT(0x80070057);
    pub(super) const E_FAIL: HRESULT = HRESULT(0x800040005);
}

pub use error_impl::{Error, Result};
use error_impl::{E_FAIL, E_INVALIDARG};

pub fn invalid_arg() -> Error { E_INVALIDARG.into() }
pub fn fail() -> Error { E_FAIL.into() }
