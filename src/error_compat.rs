use directxtex_sys::HRESULT;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[allow(overflowing_literals)]
const E_INVALIDARG: HRESULT = 0x8007_0057;
#[allow(overflowing_literals)]
const E_FAIL: HRESULT = 0x8000_40005;

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Error(pub HRESULT);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error code 0x{:08x}", self.0)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HRESULT(0x{:08x})", self.0)
    }
}

impl From<HRESULT> for Error {
    fn from(hr: HRESULT) -> Self { Self(hr) }
}

impl From<std::convert::Infallible> for Error {
    fn from(_: std::convert::Infallible) -> Self { unreachable!() }
}

pub fn invalid_arg() -> Error { E_INVALIDARG.into() }
pub fn fail() -> Error { E_FAIL.into() }

pub const fn hresult(hr: directxtex_sys::HRESULT) -> Result<(), Error> {
    if hr == 0 { Ok(()) } else { Err(Error(hr)) }
}
