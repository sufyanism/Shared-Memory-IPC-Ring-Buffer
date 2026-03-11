use std::sync::atomic::AtomicUsize;

#[repr(C)]
pub struct SharedHeader {
    pub head: AtomicUsize,
    pub tail: AtomicUsize,
    pub capacity: usize,
}

impl SharedHeader {
    pub fn mask(&self) -> usize {
        self.capacity - 1
    }
}
