use std::ptr;
use std::sync::atomic::Ordering;

use crate::ring::SharedHeader;

pub struct Producer<T> {
    header: *mut SharedHeader,
    buffer: *mut T,
}

impl<T: Copy> Producer<T> {
    pub unsafe fn new(header: *mut SharedHeader) -> Self {
        let buffer = header.add(1) as *mut T;
        Self { header, buffer }
    }

    pub fn try_push(&self, value: T) -> bool {
        unsafe {
            let header = &*self.header;

            let tail = header.tail.load(Ordering::Relaxed);
            let head = header.head.load(Ordering::Acquire);

            if tail - head == header.capacity {
                return false;
            }

            let index = tail & header.mask();

            ptr::write(self.buffer.add(index), value);

            header.tail.store(tail + 1, Ordering::Release);

            true
        }
    }
}
