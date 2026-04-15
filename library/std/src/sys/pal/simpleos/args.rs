//! SimpleOS args — Wave-3 scaffold.
//! SimpleOS has no argv at this stage; returns an empty iterator.
//! Wave-4: wire up libsimpleos_c process_args() call.

pub struct Args(core::iter::Empty<crate::ffi::OsString>);

pub fn args() -> Args {
    Args(core::iter::empty())
}

impl Iterator for Args {
    type Item = crate::ffi::OsString;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(0))
    }
}

impl ExactSizeIterator for Args {
    fn len(&self) -> usize {
        0
    }
}

impl DoubleEndedIterator for Args {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}
