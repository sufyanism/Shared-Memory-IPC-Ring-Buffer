use libc::*;
use std::ffi::CString;
use std::ptr;
use std::io;

pub struct SharedMem {
    pub ptr: *mut u8,
    pub size: usize,
    fd: i32,
    name: CString,
}

impl SharedMem {
    pub fn create(name: &str, size: usize) -> io::Result<Self> {
        let cname = CString::new(name).unwrap();

        unsafe {
            let fd = shm_open(cname.as_ptr(), O_CREAT | O_RDWR, 0o600);
            if fd < 0 {
                return Err(io::Error::last_os_error());
            }

            if ftruncate(fd, size as i64) != 0 {
                return Err(io::Error::last_os_error());
            }

            let ptr = mmap(
                ptr::null_mut(),
                size,
                PROT_READ | PROT_WRITE,
                MAP_SHARED,
                fd,
                0,
            );

            if ptr == MAP_FAILED {
                return Err(io::Error::last_os_error());
            }

            Ok(Self {
                ptr: ptr as *mut u8,
                size,
                fd,
                name: cname,
            })
        }
    }
}

impl Drop for SharedMem {
    fn drop(&mut self) {
        unsafe {
            munmap(self.ptr as *mut _, self.size);
            shm_unlink(self.name.as_ptr());
        }
    }
}
