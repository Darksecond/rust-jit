use super::memory::*;

#[derive(Debug)]
pub struct Backend {
    mem: Memory
}

#[cfg(target_arch="x86_64")]
impl Backend {
    pub fn new() -> Backend {
        Backend {
            mem: Memory::new(1) //TODO Possibly allocate more pages
        }
    }

    // mov <u32> -> rax
    pub fn mov_rax_u32(&mut self, value: u32) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0x48);
        self.mem.push_u8(0xc7);
        self.mem.push_u8(0xc0);
        self.mem.push_u32(value);
        offset
    }

    // mov <u32> -> rdi
    pub fn mov_rdi_u32(&mut self, value: u32) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0x48);
        self.mem.push_u8(0xc7);
        self.mem.push_u8(0xc7);
        self.mem.push_u32(value);
        offset
    }

    // mov <u64> -> rax
    pub fn mov_rax_u64(&mut self, value: u64) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0x48);
        self.mem.push_u8(0xB8);
        self.mem.push_u64(value);
        offset
    }

    pub fn inc_rax(&mut self) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0x48);
        self.mem.push_u8(0xff);
        self.mem.push_u8(0xc0);
        offset
    }

    pub fn ret(&mut self) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0xc3);
        offset
    }

    //TODO Add debug feature to add runtime 16-byte stack alignment checks
    pub fn call(&mut self, dst: isize) -> isize {
        let offset = self.mem.offset();
        let src = self.mem.absolute_offset() + 5;
        let rel = dst - src;
        //Make sure this call is possible
        //TODO Make sure it isn't smalelr than possible either
        assert!((rel as i64) < (u32::max_value() as i64));
        self.mem.push_u8(0xe8);
        self.mem.push_u32(rel as u32);
        offset
    }

    pub fn push_rbp(&mut self) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0x55);
        offset
    }

    pub fn push_rax(&mut self) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0x50);
        offset
    }

    // sub <u8> from rsp
    pub fn sub_rsp_u8(&mut self, value: u8) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0x48);
        self.mem.push_u8(0x83);
        self.mem.push_u8(0xec);
        self.mem.push_u8(value);
        offset
    }

    // add <u8> to rsp
    pub fn add_rsp_u8(&mut self, value: u8) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0x48);
        self.mem.push_u8(0x83);
        self.mem.push_u8(0xc4);
        self.mem.push_u8(value);
        offset
    }
    
    //Move double from [rsp] to xmm0
    //movsd [rsp] -> xmm0
    pub fn movsd_xmm0_ptr_rsp(&mut self) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0xf2);
        self.mem.push_u8(0x0f);
        self.mem.push_u8(0x10);
        self.mem.push_u8(0x04);
        self.mem.push_u8(0x24);
        offset
    }

    pub fn movsd_xmm0_ptr_rdi(&mut self) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0xf2);
        self.mem.push_u8(0x0f);
        self.mem.push_u8(0x10);
        self.mem.push_u8(0x07);
        offset
    }

    pub fn movsd_xmm0_ptr_rdi_offset_u8(&mut self, value: u8) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0xf2);
        self.mem.push_u8(0x0f);
        self.mem.push_u8(0x10);
        self.mem.push_u8(0x47);
        self.mem.push_u8(value);
        offset
    }
    
    //mov rsp -> rbp
    pub fn mov_rsp_rbp(&mut self) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0x48);
        self.mem.push_u8(0x89);
        self.mem.push_u8(0xec);
        offset
    }

    pub fn leave(&mut self) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0xc9);
        offset
    }

    pub fn to_mem(self) -> Memory {
        let mut mem = self.mem;
        mem.protect();
        mem
    }
}
