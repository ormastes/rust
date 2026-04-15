//! SimpleOS os — Wave-3 scaffold.
//! getenv/setenv delegate to libc (available via libsimpleos_c POSIX layer).
//! current_exe is Unsupported at scaffold stage; Wave-4 wires libsimpleos_c.

use crate::ffi::{CStr, CString, OsStr, OsString};
use crate::io;
use crate::os::unix::ffi::OsStringExt;
use crate::path::PathBuf;

use super::unsupported;

pub fn errno() -> i32 {
    unsafe { *libc::__errno_location() }
}

pub fn error_string(errno: i32) -> String {
    // SAFETY: strerror_r writes into buf; we cap at 128 bytes.
    let mut buf = [0i8; 128];
    let _r = unsafe { libc::strerror_r(errno, buf.as_mut_ptr(), buf.len()) };
    let s = unsafe { CStr::from_ptr(buf.as_ptr()) };
    s.to_string_lossy().into_owned()
}

pub fn getcwd() -> io::Result<PathBuf> {
    let mut buf = vec![0u8; 512];
    loop {
        let ret = unsafe { libc::getcwd(buf.as_mut_ptr() as *mut libc::c_char, buf.len()) };
        if !ret.is_null() {
            let s = unsafe { CStr::from_ptr(ret as *const libc::c_char) };
            return Ok(PathBuf::from(OsString::from_vec(s.to_bytes().to_vec())));
        }
        let err = errno();
        if err == libc::ERANGE {
            let new_cap = buf.len() * 2;
            buf.resize(new_cap, 0);
        } else {
            return Err(io::Error::from_raw_os_error(err));
        }
    }
}

pub fn chdir(p: &crate::path::Path) -> io::Result<()> {
    let c = CString::new(p.as_os_str().as_encoded_bytes())
        .map_err(|_| io::Error::from_raw_os_error(libc::EINVAL))?;
    let ret = unsafe { libc::chdir(c.as_ptr()) };
    if ret == 0 { Ok(()) } else { Err(io::Error::from_raw_os_error(errno())) }
}

pub fn getenv(k: &OsStr) -> Option<OsString> {
    let key = CString::new(k.as_encoded_bytes()).ok()?;
    let val = unsafe { libc::getenv(key.as_ptr()) };
    if val.is_null() {
        None
    } else {
        let s = unsafe { CStr::from_ptr(val) };
        Some(OsString::from_vec(s.to_bytes().to_vec()))
    }
}

pub unsafe fn setenv(k: &OsStr, v: &OsStr) -> io::Result<()> {
    let key = CString::new(k.as_encoded_bytes())
        .map_err(|_| io::Error::from_raw_os_error(libc::EINVAL))?;
    let val = CString::new(v.as_encoded_bytes())
        .map_err(|_| io::Error::from_raw_os_error(libc::EINVAL))?;
    let ret = unsafe { libc::setenv(key.as_ptr(), val.as_ptr(), 1) };
    if ret == 0 { Ok(()) } else { Err(io::Error::from_raw_os_error(errno())) }
}

pub unsafe fn unsetenv(k: &OsStr) -> io::Result<()> {
    let key = CString::new(k.as_encoded_bytes())
        .map_err(|_| io::Error::from_raw_os_error(libc::EINVAL))?;
    let ret = unsafe { libc::unsetenv(key.as_ptr()) };
    if ret == 0 { Ok(()) } else { Err(io::Error::from_raw_os_error(errno())) }
}

/// Wave-4: implement via libsimpleos_c process_exe() or /proc/self/exe equivalent.
pub fn current_exe() -> io::Result<PathBuf> {
    unsupported()
}

pub struct Env(core::iter::Empty<(OsString, OsString)>);

pub fn env() -> Env {
    // Wave-4: iterate environ[] via libc.
    Env(core::iter::empty())
}

impl Iterator for Env {
    type Item = (OsString, OsString);
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
