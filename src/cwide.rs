use std::{borrow::Cow, ffi::OsStr};
use std::iter;
use std::ops::Deref;

use crate::error::invalid_arg;
use crate::Error;

pub struct CWide<'str>(Cow<'str, [u16]>);

impl<'str> Deref for CWide<'str> {
    type Target = [u16];

    #[inline]
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[cfg(windows)]
impl From<&OsStr> for CWide<'static> {
    #[inline]
    fn from(value: &OsStr) -> Self {
        Self(Cow::Owned(
            std::os::windows::ffi::OsStrExt::encode_wide(value)
                .chain(iter::once(0))
                .collect(),
        ))
    }
}

#[cfg(unix)]
impl From<&std::ffi::OsStr> for CWide<'static> {
    #[inline]
    fn from(value: &core::ffi::OsStr) -> Result<Self, Self::Error> {
        CWide::from(value.to_string_lossy().as_ref())
    }
}

impl From<&std::path::Path> for CWide<'static> {
    #[inline]
    fn from(value: &std::path::Path) -> Self { CWide::from(value.as_os_str()) }
}


impl From<std::path::PathBuf> for CWide<'static> {
    #[inline]
    fn from(value: std::path::PathBuf) -> Self { CWide::from(value.as_os_str()) }
}

impl From<&str> for CWide<'static> {
    #[inline]
    fn from(from: &str) -> Self {
        Self(Cow::Owned(
            from.encode_utf16().chain(iter::once(0)).collect(),
        ))
    }
}

#[cfg(feature = "windows")]
impl<'str> From<&'str windows::core::HSTRING> for CWide<'str> {
    #[inline]
    fn from(hstr: &'str windows::core::HSTRING) -> Self { Self(Cow::Borrowed(hstr.as_wide())) }
}

impl<'str> TryFrom<&'str [u16]> for CWide<'str> {
    type Error = Error;

    #[inline]
    fn try_from(value: &'str [u16]) -> Result<Self, Self::Error> {
        if value.last() == Some(&0) {
            Ok(Self(Cow::Borrowed(value)))
        } else {
            Err(invalid_arg())
        }
    }
}

impl TryFrom<Vec<u16>> for CWide<'static> {
    type Error = Error;

    #[inline]
    fn try_from(value: Vec<u16>) -> Result<Self, Self::Error> {
        if value.last() == Some(&0) {
            Ok(Self(Cow::Owned(value)))
        } else {
            Err(invalid_arg())
        }
    }
}
