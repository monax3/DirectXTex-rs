use directxtex_sys::{Blob, ScratchImage};

pub const BUFSIZE: usize = 1_048_576;
pub mod prelude {
    pub use std::ptr::null;
    pub use std::sync::atomic::{AtomicBool, Ordering};

    pub use windows::core::HSTRING;
    #[cfg(feature = "windows")] pub use windows::Win32::Foundation::S_OK;

    #[cfg(not(feature = "windows"))]
    pub const S_OK: directxtex_sys::HRESULT = 0;
}

use prelude::*;
use windows::Win32::System::Com::{CoInitializeEx, COINIT_DISABLE_OLE1DDE, COINIT_MULTITHREADED};

pub const PNG_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test.png");

pub const DDS_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test_complex.dds");

pub const TGA_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test.tga");

pub const EXR_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test.exr");

pub const HDR_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test.hdr");

#[derive(Default)]
pub struct DropBlob(pub Blob);

impl DropBlob {
    pub fn assert_has_data(&self, ctx: &str) {
        assert!(!self.0.m_buffer.is_null(), "{ctx} produced a null pointer");
        let len = self.0.m_size;
        assert_ne!(len, 0, "{ctx} produced an empty blob");
        eprintln!("{ctx} produced a {len} byte blob");
    }
}

impl std::ops::Deref for DropBlob {
    type Target = Blob;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl std::ops::DerefMut for DropBlob {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl Drop for DropBlob {
    fn drop(&mut self) { unsafe { self.0.Release() }; }
}

#[derive(Default)]
pub struct DropScratch(pub ScratchImage);

impl DropScratch {
    pub fn assert_has_data(&self, ctx: &str) {
        assert!(!self.0.m_memory.is_null(), "{ctx} produced a null pointer");
        let len = self.0.m_size;
        assert_ne!(len, 0, "{ctx} produced an empty ScratchImage");
        eprintln!("{ctx} produced a {len} byte ScratchImage");
    }
}

impl std::ops::Deref for DropScratch {
    type Target = ScratchImage;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl std::ops::DerefMut for DropScratch {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl Drop for DropScratch {
    fn drop(&mut self) { unsafe { self.0.Release() }; }
}

pub struct TempFile {
    pub tempfile: tempfile::TempPath,
    pub wide:     HSTRING,
}

impl TempFile {
    pub fn new() -> Self {
        let tempfile = tempfile::Builder::new()
            .tempfile_in(env!("CARGO_TARGET_TMPDIR"))
            .expect("Failed to open temporary file")
            .into_temp_path();
        let wide = HSTRING::from(tempfile.as_os_str());
        Self { tempfile, wide }
    }

    pub fn assert_has_data(&self, ctx: &str) {
        let len = self
            .tempfile
            .metadata()
            .expect("Failed to get temp file metadata")
            .len();
        assert_ne!(len, 0, "{ctx} produced an empty file");
        eprintln!("{ctx} produced a {len} byte file");
    }

    pub fn as_wideptr(&self) -> *const u16 { self.wide.as_ptr() }
}

impl Default for TempFile {
    fn default() -> Self { Self::new() }
}

pub struct FailureFlag(AtomicBool);

impl FailureFlag {
    pub const fn new() -> Self { Self(AtomicBool::new(false)) }

    pub fn assert_success(&self, ctx: &str) {
        let failed = self.0.swap(false, Ordering::Relaxed);
        assert!(!failed, "{ctx} signalled failure in FFI callback");
    }

    pub fn fail(&self) { self.0.store(true, Ordering::Relaxed); }
}

pub fn initialize_com() {
    unsafe { CoInitializeEx(None, COINIT_MULTITHREADED | COINIT_DISABLE_OLE1DDE) }
        .expect("Failed to initialize COM");
}
