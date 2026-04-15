//! SimpleOS stdio — Wave-3 scaffold.
//! Stdout/Stderr via libc write(1/2, ...), Stdin via libc read(0, ...).
//! Wave-4: replace with libsimpleos_c console_write/console_read if needed.

use crate::io;

pub struct Stdin;
pub struct Stdout;
pub struct Stderr;

impl Stdin {
    pub const fn new() -> Stdin {
        Stdin
    }
}

impl io::Read for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // SAFETY: buf is a valid slice; fd 0 is stdin.
        let ret = unsafe { libc::read(0, buf.as_mut_ptr() as *mut libc::c_void, buf.len()) };
        if ret < 0 {
            Err(io::Error::from_raw_os_error(unsafe { *libc::__errno_location() }))
        } else {
            Ok(ret as usize)
        }
    }
}

impl Stdout {
    pub const fn new() -> Stdout {
        Stdout
    }
}

impl io::Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // SAFETY: buf is a valid slice; fd 1 is stdout.
        let ret = unsafe { libc::write(1, buf.as_ptr() as *const libc::c_void, buf.len()) };
        if ret < 0 {
            Err(io::Error::from_raw_os_error(unsafe { *libc::__errno_location() }))
        } else {
            Ok(ret as usize)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Stderr {
    pub const fn new() -> Stderr {
        Stderr
    }
}

impl io::Write for Stderr {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // SAFETY: buf is a valid slice; fd 2 is stderr.
        let ret = unsafe { libc::write(2, buf.as_ptr() as *const libc::c_void, buf.len()) };
        if ret < 0 {
            Err(io::Error::from_raw_os_error(unsafe { *libc::__errno_location() }))
        } else {
            Ok(ret as usize)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub const STDIN_BUF_SIZE: usize = crate::sys_common::io::DEFAULT_BUF_SIZE;

pub fn is_ebadf(err: &io::Error) -> bool {
    err.raw_os_error() == Some(libc::EBADF)
}

pub fn panic_output() -> Option<impl io::Write> {
    Some(Stderr::new())
}
