use std::ptr;
use std::sync::atomic::Ordering;

use crate::ring::SharedHeader;

pub struct Consumer<T> {
    header: *mut SharedHeader,
    buffer: *mut T,
}

impl<T: Copy> Consumer<T> {
    pub unsafe fn new(header: *mut SharedHeader) -> Self {
        let buffer = header.add(1) as *mut T;
        Self { header, buffer }
    }

    pub fn try_pop(&self) -> Option<T> {
        unsafe {
            let header = &*self.header;

            let head = header.head.load(Ordering::Relaxed);
            let tail = header.tail.load(Ordering::Acquire);

            if head == tail {
                return None;
            }

            let index = head & header.mask();

            let value = ptr::read(self.buffer.add(index));

            header.head.store(head + 1, Ordering::Release);

            Some(value)
        }
    }
}
