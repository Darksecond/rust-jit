use super::memory::*;

#[derive(Debug)]
pub struct Backend {
    mem: Memory
}

#[cfg(target_arch="x86_64")]
impl Backend {
    pub fn new() -> Backend {
        Backend {
            mem: Memory::new(1)
        }
    }

    // mov <u32> -> rax
    pub fn mov_rax_u32(&mut self, value: u32) {
        self.mem.push_u8(0x48);
        self.mem.push_u8(0xc7);
        self.mem.push_u8(0xc0);
        self.mem.push_u32(value);
    }

    // mov <u32> -> rdi
    pub fn mov_rdi_u32(&mut self, value: u32) {
        self.mem.push_u8(0x48);
        self.mem.push_u8(0xc7);
        self.mem.push_u8(0xc7);
        self.mem.push_u32(value);
    }

    // mov <u64> -> rax
    pub fn mov_rax_u64(&mut self, value: u64) {
        self.mem.push_u8(0x48);
        self.mem.push_u8(0xB8);
        self.mem.push_u64(value);
    }

    pub fn inc_rax(&mut self) {
        self.mem.push_u8(0x48);
        self.mem.push_u8(0xff);
        self.mem.push_u8(0xc0);
    }

    pub fn ret(&mut self) {
        self.mem.push_u8(0xc3);
    }

    pub fn call(&mut self, dst: isize) {
        let src = self.mem.absolute_offset() + 5;
        let rel = dst - src;
        //Make sure this call is possible
        //TODO Make sure it isn't smalelr than possible either
        assert!((rel as i64) < (u32::max_value() as i64));
        self.mem.push_u8(0xe8);
        self.mem.push_u32(rel as u32);
    }

    pub fn push_rbp(&mut self) {
        self.mem.push_u8(0x55);
    }

    pub fn push_rax(&mut self) {
        self.mem.push_u8(0x50);
    }
    
    //Move double from [rsp] to xmm0
    //movsd [rsp] -> xmm0
    pub fn movsd_xmm0_ptr_rsp(&mut self) {
        self.mem.push_u8(0xf2);
        self.mem.push_u8(0x0f);
        self.mem.push_u8(0x10);
        self.mem.push_u8(0x04);
        self.mem.push_u8(0x24);
    }
    
    //mov rsp -> rbp
    pub fn mov_rsp_rbp(&mut self) {
        self.mem.push_u8(0x48);
        self.mem.push_u8(0x89);
        self.mem.push_u8(0xec);
    }

    pub fn leave(&mut self) {
        self.mem.push_u8(0xc9);
    }

    //TODO Rename to to_mem
    pub fn mem(self) -> Memory {
        let mut mem = self.mem;
        mem.protect();
        mem
    }
}
