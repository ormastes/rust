//! SimpleOS thread — Wave-4: pthread trampoline over libsimpleos_c shim.
//! Thread::new/join/yield_now wired to pthread_create/join/sched_yield.
//! Thread::sleep, set_name remain stubs (Wave-5 scope).

use crate::ffi::CStr;
use crate::io;
use crate::num::NonZero;
use crate::thread::ThreadInit;
use crate::time::Duration;

use core::ffi::c_void;
use core::ptr::null_mut;

extern "C" {
    fn pthread_create(
        th: *mut usize,
        attr: *const c_void,
        start: extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> i32;
    fn pthread_join(th: usize, ret: *mut *mut c_void) -> i32;
    fn pthread_self() -> usize;
    fn pthread_detach(th: usize) -> i32;
    fn sched_yield() -> i32;
}

/// C-ABI trampoline: unboxes the `Box<dyn FnOnce() + Send>` and calls it.
extern "C" fn thread_start(arg: *mut c_void) -> *mut c_void {
    // Safety: arg is a Box<dyn FnOnce() + Send> leaked via Box::into_raw in Thread::new.
    unsafe {
        let f: Box<Box<dyn FnOnce() + Send>> =
            Box::from_raw(arg as *mut Box<dyn FnOnce() + Send>);
        (*f)();
    }
    null_mut()
}

pub struct Thread(usize);

pub const DEFAULT_MIN_STACK_SIZE: usize = 64 * 1024;

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    pub unsafe fn new(_stack: usize, init: Box<ThreadInit>) -> io::Result<Thread> {
        // init.init() sets up TLS current-thread state and returns the user closure.
        let f: Box<dyn FnOnce() + Send> = init.init();
        // Double-box for a stable fat-pointer round-trip through *mut c_void.
        let raw = Box::into_raw(Box::new(f)) as *mut c_void;

        let mut tid: usize = 0;
        let ret = pthread_create(&mut tid, null_mut(), thread_start, raw);
        if ret == 0 {
            Ok(Thread(tid))
        } else {
            // Reclaim the allocation on failure so we don't leak.
            drop(Box::from_raw(raw as *mut Box<dyn FnOnce() + Send>));
            Err(io::Error::from_raw_os_error(ret))
        }
    }

    pub fn join(self) {
        unsafe {
            pthread_join(self.0, null_mut());
        }
    }
}

/// Wave-5: wire to libc sysconf(_SC_NPROCESSORS_ONLN).
pub fn available_parallelism() -> io::Result<NonZero<usize>> {
    Err(io::Error::UNKNOWN_THREAD_COUNT)
}

pub fn current_os_id() -> Option<u64> {
    Some(unsafe { pthread_self() } as u64)
}

pub fn yield_now() {
    unsafe {
        sched_yield();
    }
}

/// Wave-5: wire to pthread_setname_np.
pub fn set_name(_name: &CStr) {
    // no-op
}

/// Wave-5: wire to nanosleep.
pub fn sleep(_dur: Duration) {
    panic!("thread::sleep not yet implemented on SimpleOS");
}
