//! SimpleOS thread — Wave-3 scaffold (cooperative scheduler stub).
//! All entry points return `Unsupported` io::Error or are no-ops.
//! Wave-4 plan: wire `Thread::new` to the SimpleOS cooperative scheduler via
//! `scheduler.clone_task` (interface IF-02); `sleep` and `yield_now` will
//! delegate to scheduler yield / nanosleep primitives.

use crate::ffi::CStr;
use crate::io;
use crate::num::NonZero;
use crate::thread::ThreadInit;
use crate::time::Duration;

// Silence dead code warnings for the otherwise unused ThreadInit::init() call.
#[expect(dead_code)]
fn dummy_init_call(init: Box<ThreadInit>) {
    drop(init.init());
}

pub struct Thread(!);

pub const DEFAULT_MIN_STACK_SIZE: usize = 64 * 1024;

impl Thread {
    // unsafe: see thread::Builder::spawn_unchecked for safety requirements
    /// Wave-4: delegate to SimpleOS cooperative scheduler via `scheduler.clone_task` (IF-02).
    pub unsafe fn new(_stack: usize, _init: Box<ThreadInit>) -> io::Result<Thread> {
        Err(io::Error::UNSUPPORTED_PLATFORM)
    }

    pub fn join(self) {
        self.0
    }
}

/// Wave-4: query scheduler task count or libc sysconf(_SC_NPROCESSORS_ONLN).
pub fn available_parallelism() -> io::Result<NonZero<usize>> {
    Err(io::Error::UNKNOWN_THREAD_COUNT)
}

pub fn current_os_id() -> Option<u64> {
    None
}

/// Wave-4: delegate to scheduler yield primitive.
pub fn yield_now() {
    // Wave-3: no-op; Wave-4 calls scheduler.yield_current()
}

/// Wave-4: wire to scheduler nanosleep or libc nanosleep.
pub fn set_name(_name: &CStr) {
    // Wave-3: no-op
}

/// Wave-4: wire to libc nanosleep or scheduler sleep primitive.
pub fn sleep(_dur: Duration) {
    panic!("can't sleep");
}
