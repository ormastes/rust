//! SimpleOS time — Wave-3 scaffold.
//! Uses libc clock_gettime (available via libsimpleos_c POSIX layer).
//! Wave-4: no changes expected; this should work as-is on SimpleOS.

use crate::time::Duration;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Instant(Duration);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct SystemTime(Duration);

pub const UNIX_EPOCH: SystemTime = SystemTime(Duration::ZERO);

fn clock_gettime(clock: libc::clockid_t) -> Duration {
    let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    // SAFETY: ts is a valid pointer; clock id is a known constant.
    unsafe { libc::clock_gettime(clock, &mut ts) };
    Duration::new(ts.tv_sec as u64, ts.tv_nsec as u32)
}

impl Instant {
    pub fn now() -> Instant {
        Instant(clock_gettime(libc::CLOCK_MONOTONIC))
    }

    pub fn checked_sub_instant(&self, other: &Instant) -> Option<Duration> {
        self.0.checked_sub(other.0)
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<Instant> {
        self.0.checked_add(*other).map(Instant)
    }

    pub fn checked_sub_duration(&self, other: &Duration) -> Option<Instant> {
        self.0.checked_sub(*other).map(Instant)
    }
}

impl SystemTime {
    pub fn now() -> SystemTime {
        SystemTime(clock_gettime(libc::CLOCK_REALTIME))
    }

    pub fn sub_time(&self, other: &SystemTime) -> Result<Duration, Duration> {
        self.0.checked_sub(other.0).ok_or_else(|| other.0.checked_sub(self.0).unwrap_or(Duration::ZERO))
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<SystemTime> {
        self.0.checked_add(*other).map(SystemTime)
    }

    pub fn checked_sub_duration(&self, other: &Duration) -> Option<SystemTime> {
        self.0.checked_sub(*other).map(SystemTime)
    }
}
