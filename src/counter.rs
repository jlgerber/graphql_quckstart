
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub struct Counter {
    cnt: AtomicUsize
}

impl Counter {
    pub fn new() -> Self {
        Self {
            cnt: AtomicUsize::new(0)
        }
    }

    pub fn inc(&self) -> usize {
        self.cnt.fetch_add(1, Ordering::AcqRel)
    }
}
