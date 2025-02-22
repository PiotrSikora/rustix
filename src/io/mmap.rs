//! The `mmap` API.
//!
//! # Safety
//!
//! `mmap` manipulates raw pointers and has special semantics and is
//! wildly unsafe.
#![allow(unsafe_code)]

use crate::{imp, io};
use core::ffi::c_void;
use imp::fd::AsFd;

#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
pub use imp::io::MlockFlags;
#[cfg(any(linux_raw, all(libc, target_os = "linux")))]
pub use imp::io::MremapFlags;
pub use imp::io::{MapFlags, MprotectFlags, ProtFlags};

/// `mmap(ptr, len, prot, flags, fd, offset)`—Create a file-backed memory
/// mapping.
///
/// For anonymous mappings (`MAP_ANON`/`MAP_ANONYMOUS`), see
/// [`mmap_anonymous`].
///
/// # Safety
///
/// Raw pointers and lots of special semantics.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/mmap.html
/// [Linux]: https://man7.org/linux/man-pages/man2/mmap.2.html
#[inline]
pub unsafe fn mmap<Fd: AsFd>(
    ptr: *mut c_void,
    len: usize,
    prot: ProtFlags,
    flags: MapFlags,
    fd: &Fd,
    offset: u64,
) -> io::Result<*mut c_void> {
    let fd = fd.as_fd();
    imp::syscalls::mmap(ptr, len, prot, flags, fd, offset)
}

/// `mmap(ptr, len, prot, MAP_ANONYMOUS | flags, -1, 0)`—Create an anonymous
/// memory mapping.
///
/// For file-backed mappings, see [`mmap`].
///
/// # Safety
///
/// Raw pointers and lots of special semantics.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/mmap.html
/// [Linux]: https://man7.org/linux/man-pages/man2/mmap.2.html
#[inline]
#[doc(alias = "mmap")]
pub unsafe fn mmap_anonymous(
    ptr: *mut c_void,
    len: usize,
    prot: ProtFlags,
    flags: MapFlags,
) -> io::Result<*mut c_void> {
    imp::syscalls::mmap_anonymous(ptr, len, prot, flags)
}

/// `munmap(ptr, len)`
///
/// # Safety
///
/// Raw pointers and lots of special semantics.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/munmap.html
/// [Linux]: https://man7.org/linux/man-pages/man2/munmap.2.html
#[inline]
pub unsafe fn munmap(ptr: *mut c_void, len: usize) -> io::Result<()> {
    imp::syscalls::munmap(ptr, len)
}

/// `mremap(old_address, old_size, new_size, flags)`—Resize, modify,
/// and/or move a memory mapping.
///
/// For moving a mapping to a fixed address (`MREMAP_FIXED`), see
/// [`mremap_fixed`].
///
/// # Safety
///
/// Raw pointers and lots of special semantics.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/mremap.2.html
#[cfg(any(linux_raw, all(libc, target_os = "linux")))]
#[inline]
pub unsafe fn mremap(
    old_address: *mut c_void,
    old_size: usize,
    new_size: usize,
    flags: MremapFlags,
) -> io::Result<*mut c_void> {
    imp::syscalls::mremap(old_address, old_size, new_size, flags)
}

/// `mremap(old_address, old_size, new_size, MREMAP_FIXED | flags)`—Resize,
/// modify, and/or move a memory mapping to a specific address.
///
/// For `mremap` without moving to a specific address, see [`mremap`].
/// [`mremap_fixed`].
///
/// # Safety
///
/// Raw pointers and lots of special semantics.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/mremap.2.html
#[cfg(any(linux_raw, all(libc, target_os = "linux")))]
#[inline]
#[doc(alias = "mremap")]
pub unsafe fn mremap_fixed(
    old_address: *mut c_void,
    old_size: usize,
    new_size: usize,
    flags: MremapFlags,
    new_address: *mut c_void,
) -> io::Result<*mut c_void> {
    imp::syscalls::mremap_fixed(old_address, old_size, new_size, flags, new_address)
}

/// `mprotect(ptr, len, flags)`
///
/// # Safety
///
/// Raw pointers and lots of special semantics.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/mprotect.html
/// [Linux]: https://man7.org/linux/man-pages/man2/mprotect.2.html
#[inline]
pub unsafe fn mprotect(ptr: *mut c_void, len: usize, flags: MprotectFlags) -> io::Result<()> {
    imp::syscalls::mprotect(ptr, len, flags)
}

/// `mlock(ptr, len)`—Lock memory into RAM.
///
/// # Safety
///
/// This function operates on raw pointers, but it should only be used on
/// memory which the caller owns. Technically, locking memory shouldn't violate
/// any invariants, but since unlocking it can violate invariants, this
/// function is also unsafe for symmetry.
///
/// Some implementations implicitly round the memory region out to the nearest
/// page boundaries, so this function may lock more memory than explicitly
/// requested if the memory isn't page-aligned.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/mlock.html
/// [Linux]: https://man7.org/linux/man-pages/man2/mlock.2.html
#[inline]
pub unsafe fn mlock(ptr: *mut c_void, len: usize) -> io::Result<()> {
    imp::syscalls::mlock(ptr, len)
}

/// `mlock2(ptr, len, flags)`—Lock memory into RAM, with
/// flags.
///
/// `mlock_with` is the same as [`mlock`] but adds an additional flags operand.
///
/// # Safety
///
/// This function operates on raw pointers, but it should only be used on
/// memory which the caller owns. Technically, locking memory shouldn't violate
/// any invariants, but since unlocking it can violate invariants, this
/// function is also unsafe for symmetry.
///
/// Some implementations implicitly round the memory region out to the nearest
/// page boundaries, so this function may lock more memory than explicitly
/// requested if the memory isn't page-aligned.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/mlock2.2.html
#[cfg(any(linux_raw, all(libc, any(target_os = "android", target_os = "linux"))))]
#[inline]
#[doc(alias = "mlock2")]
pub unsafe fn mlock_with(ptr: *mut c_void, len: usize, flags: MlockFlags) -> io::Result<()> {
    imp::syscalls::mlock_with(ptr, len, flags)
}

/// `munlock(ptr, len)`—Unlock memory.
///
/// # Safety
///
/// This function operates on raw pointers, but it should only be used on
/// memory which the caller owns, to avoid compromising the `mlock` invariants
/// of other unrelated code in the process.
///
/// Some implementations implicitly round the memory region out to the nearest
/// page boundaries, so this function may unlock more memory than explicitly
/// requested if the memory isn't page-aligned.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/munlock.html
/// [Linux]: https://man7.org/linux/man-pages/man2/munlock.2.html
#[inline]
pub unsafe fn munlock(ptr: *mut c_void, len: usize) -> io::Result<()> {
    imp::syscalls::munlock(ptr, len)
}
