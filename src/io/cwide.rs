use std::borrow::Cow;

use crate::error::invalid_arg;
use crate::Error;

pub struct CWide<'a>(Cow<'a, [u16]>);

impl<'a> std::ops::Deref for CWide<'a> {
    type Target = [u16];

    fn deref(&self) -> &Self::Target { &self.0 }
}

#[cfg(windows)]
impl From<&std::ffi::OsStr> for CWide<'static> {
    fn from(value: &std::ffi::OsStr) -> Self {
        Self(Cow::Owned(
            std::os::windows::ffi::OsStrExt::encode_wide(value)
                .chain(std::iter::once(0))
                .collect(),
        ))
    }
}

#[cfg(unix)]
impl From<&std::ffi::OsStr> for CWide<'static> {
    fn from(value: &std::ffi::OsStr) -> Result<Self, Self::Error> {
        CWide::from(value.to_string_lossy().as_ref())
    }
}

impl From<&std::path::Path> for CWide<'static> {
    fn from(value: &std::path::Path) -> Self { CWide::from(value.as_os_str()) }
}

impl From<&str> for CWide<'static> {
    fn from(from: &str) -> Self {
        Self(Cow::Owned(
            from.encode_utf16().chain(std::iter::once(0)).collect(),
        ))
    }
}

#[cfg(feature = "windows")]
impl<'a> From<&'a windows::core::HSTRING> for CWide<'a> {
    fn from(hstr: &'a windows::core::HSTRING) -> Self { Self(Cow::Borrowed(hstr.as_wide())) }
}

impl<'a> TryFrom<&'a [u16]> for CWide<'a> {
    type Error = Error;

    fn try_from(value: &'a [u16]) -> Result<Self, Self::Error> {
        if value.last() == Some(&0) {
            Ok(Self(Cow::Borrowed(value)))
        } else {
            Err(invalid_arg())
        }
    }
}

impl TryFrom<Vec<u16>> for CWide<'static> {
    type Error = Error;

    fn try_from(value: Vec<u16>) -> Result<Self, Self::Error> {
        if value.last() == Some(&0) {
            Ok(Self(Cow::Owned(value)))
        } else {
            Err(invalid_arg())
        }
    }
}
