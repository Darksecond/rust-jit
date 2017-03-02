extern crate libc;

use std::ptr;
use std::mem;

#[derive(Debug)]
pub struct Memory {
    size: u32,
    offset: isize,
    mem: *mut u8
}

//TODO Disallow pushing beyond it's size
#[cfg(unix)]
impl Memory {
    pub fn new(pages: u32) -> Memory {
        unsafe {
            Memory {
                size: pages*4096,
                offset: 0,
                mem: mem::transmute(libc::mmap(ptr::null_mut(), 4096*pages as usize, libc::PROT_READ|libc::PROT_WRITE, libc::MAP_ANON|libc::MAP_SHARED, -1, 0))
            }
        }
    }

    //TODO push_u{16}
    pub fn push_u8(&mut self, value: u8) {
        unsafe {
            *self.mem.offset(self.offset) = value;
        }
        self.offset += 1;
    }

    pub fn push_u32(&mut self, value: u32) {
        self.push_u8(((value >> 0) & 0xff) as u8);
        self.push_u8(((value >> 8) & 0xff) as u8);
        self.push_u8(((value >> 16) & 0xff) as u8);
        self.push_u8(((value >> 24) & 0xff) as u8);
    }

    pub fn push_u64(&mut self, value: u64) {
        self.push_u8(((value >> 0) & 0xff) as u8);
        self.push_u8(((value >> 8) & 0xff) as u8);
        self.push_u8(((value >> 16) & 0xff) as u8);
        self.push_u8(((value >> 24) & 0xff) as u8);

        self.push_u8(((value >> 32) & 0xff) as u8);
        self.push_u8(((value >> 40) & 0xff) as u8);
        self.push_u8(((value >> 48) & 0xff) as u8);
        self.push_u8(((value >> 56) & 0xff) as u8);
    }

    //TODO This should disallow any usage of push_*
    pub fn protect(&mut self) {
        unsafe {
            libc::mprotect(self.mem as *mut _, self.size as usize, libc::PROT_EXEC|libc::PROT_READ);
        }
    }

    pub fn offset(&self) -> isize {
        self.offset
    }

    pub fn absolute_offset(&self) -> isize {
        unsafe {
            self.mem.offset(self.offset) as isize
        }
    }

    //TODO Better params and return value, obviously
    //TODO This should not work when unprotected
    pub fn execute(&self, vars: *mut f64) -> i64 {
        self.to_fn()(vars)
    }

    //TODO Better params and return value, obviously
    //first param in rdi, second param in rsi
    fn to_fn(&self) -> (extern "C" fn(*mut f64) -> i64) {
        unsafe {
            mem::transmute(self.mem)
        }
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        unsafe {
            libc::munmap(self.mem as *mut _, self.size as usize);
        }
    }
}
