//! SimpleOS PAL — Wave-3 scaffold (os/args/env/time/stdio modules added)
//! Wave-4 replaces Unsupported stubs with real libsimpleos_c calls.

#![deny(unsafe_op_in_unsafe_fn)]

mod common;
pub use common::*;

pub mod args;
pub mod env;
pub mod os;
pub mod process;
pub mod stdio;
pub mod thread;
pub mod time;
