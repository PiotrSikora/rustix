//! # Safety
//!
//! This uses `CStr::from_bytes_with_nul_unchecked` on the buffer that
//! it filled itself.
#![allow(unsafe_code)]

use crate::ffi::ZStr;
use crate::imp::fd::{AsFd, AsRawFd};
use core::fmt::Write;
use itoa::{Buffer, Integer};
#[cfg(feature = "std")]
use std::ffi::{CStr, OsStr};
#[cfg(feature = "std")]
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
#[cfg(feature = "std")]
#[cfg(target_os = "wasi")]
use std::os::wasi::ffi::OsStrExt;
#[cfg(feature = "std")]
use std::path::Path;

/// Format an integer into a decimal `Path` component, without constructing a
/// temporary `PathBuf` or `String`.
///
/// This is used for opening paths such as `/proc/self/fd/<fd>` on Linux.
///
/// # Example
///
/// ```rust
/// use rustix::path::DecInt;
///
/// assert_eq!(
///     format!("hello {}", DecInt::new(9876).as_ref().display()),
///     "hello 9876"
/// );
/// ```
#[derive(Clone)]
pub struct DecInt {
    // 20 `u8`s is enough to hold the decimal ASCII representation of any
    // `u64`, and we add one for a NUL terminator for `as_c_str`.
    buf: [u8; 20 + 1],
    len: usize,
}

impl DecInt {
    /// Construct a new path component from an integer.
    #[inline]
    pub fn new<Int: Integer>(i: Int) -> Self {
        let mut me = DecIntWriter(Self {
            buf: [0; 20 + 1],
            len: 0,
        });
        let mut buf = Buffer::new();
        me.write_str(buf.format(i)).unwrap();
        me.0
    }

    /// Construct a new path component from a file descriptor.
    #[inline]
    pub fn from_fd<Fd: AsFd>(fd: &Fd) -> Self {
        Self::new(fd.as_fd().as_raw_fd())
    }

    /// Return the raw byte buffer.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }

    /// Return the raw byte buffer.
    #[inline]
    pub fn as_z_str(&self) -> &ZStr {
        let bytes_with_nul = &self.buf[..=self.len];
        debug_assert!(ZStr::from_bytes_with_nul(bytes_with_nul).is_ok());
        // Safety: `self.buf` holds a single decimal ASCII representation and
        // at least one extra NUL byte.
        unsafe { ZStr::from_bytes_with_nul_unchecked(bytes_with_nul) }
    }

    /// Return the raw byte buffer.
    #[cfg(not(feature = "rustc-dep-of-std"))]
    #[inline]
    pub fn as_c_str(&self) -> &CStr {
        let bytes_with_nul = &self.buf[..=self.len];
        debug_assert!(CStr::from_bytes_with_nul(bytes_with_nul).is_ok());
        // Safety: `self.buf` holds a single decimal ASCII representation and
        // at least one extra NUL byte.
        unsafe { CStr::from_bytes_with_nul_unchecked(bytes_with_nul) }
    }
}

struct DecIntWriter(DecInt);

impl core::fmt::Write for DecIntWriter {
    #[inline]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match self.0.buf.get_mut(self.0.len..self.0.len + s.len()) {
            Some(slice) => {
                slice.copy_from_slice(s.as_bytes());
                self.0.len += s.len();
                Ok(())
            }
            None => Err(core::fmt::Error),
        }
    }
}

#[cfg(not(feature = "rustc-dep-of-std"))]
impl AsRef<Path> for DecInt {
    #[inline]
    fn as_ref(&self) -> &Path {
        let as_os_str: &OsStr = OsStrExt::from_bytes(&self.buf[..self.len]);
        Path::new(as_os_str)
    }
}
