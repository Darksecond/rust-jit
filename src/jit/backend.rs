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

    // mov rax, <u32>
    pub fn mov_rax_u32(&mut self, value: u32) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0x48);
        self.mem.push_u8(0xc7);
        self.mem.push_u8(0xc0);
        self.mem.push_u32(value);
        offset
    }

    // rdi, <u32>
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

    // mov <u64> -> rdi
    pub fn mov_rdi_u64(&mut self, value: u64) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0x48);
        self.mem.push_u8(0xBF);
        self.mem.push_u64(value);
        offset
    }

    pub fn mov_rax_ptr_rbp_offset_u8(&mut self, offset: i8) -> isize {
        let opcode_offset = self.mem.offset();
        self.mem.push_u8(0x48);
        self.mem.push_u8(0x8b);
        self.mem.push_u8(0x45);
        self.mem.push_u8(offset as u8);
        opcode_offset
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

    pub fn push_rdi(&mut self) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0x57);
        offset
    }

    //sub rsp, <u8>
    pub fn sub_rsp_u8(&mut self, value: u8) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0x48);
        self.mem.push_u8(0x83);
        self.mem.push_u8(0xec);
        self.mem.push_u8(value);
        offset
    }

    // add rsp, <u8>
    pub fn add_rsp_u8(&mut self, value: u8) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0x48);
        self.mem.push_u8(0x83);
        self.mem.push_u8(0xc4);
        self.mem.push_u8(value);
        offset
    }
    
    //movsd xmm0, [rsp]
    pub fn movsd_xmm0_ptr_rsp(&mut self) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0xf2);
        self.mem.push_u8(0x0f);
        self.mem.push_u8(0x10);
        self.mem.push_u8(0x04);
        self.mem.push_u8(0x24);
        offset
    }

    //movsd xmm1, [rsp]
    pub fn movsd_xmm1_ptr_rsp(&mut self) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0xf2);
        self.mem.push_u8(0x0f);
        self.mem.push_u8(0x10);
        self.mem.push_u8(0x0c);
        self.mem.push_u8(0x24);
        offset
    }

    //movsd xmm0, [rdi]
    pub fn movsd_xmm0_ptr_rdi(&mut self) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0xf2);
        self.mem.push_u8(0x0f);
        self.mem.push_u8(0x10);
        self.mem.push_u8(0x07);
        offset
    }

    //movsd xmm0, [rdi+offset]
    pub fn movsd_xmm0_ptr_rdi_offset_u8(&mut self, value: u8) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0xf2);
        self.mem.push_u8(0x0f);
        self.mem.push_u8(0x10);
        self.mem.push_u8(0x47);
        self.mem.push_u8(value);
        offset
    }

    //movsd xmm1, [rdi+offset]
    pub fn movsd_xmm1_ptr_rdi_offset_u8(&mut self, value: u8) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0xf2);
        self.mem.push_u8(0x0f);
        self.mem.push_u8(0x10);
        self.mem.push_u8(0x4f);
        self.mem.push_u8(value);
        offset
    }

    //movsd xmm0, [rax+offset]
    pub fn movsd_xmm0_ptr_rax_offset_u8(&mut self, value: u8) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0xf2);
        self.mem.push_u8(0x0f);
        self.mem.push_u8(0x10);
        self.mem.push_u8(0x40);
        self.mem.push_u8(value);
        offset
    }

    //movsd xmm1, [rax+offset]
    pub fn movsd_xmm1_ptr_rax_offset_u8(&mut self, value: u8) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0xf2);
        self.mem.push_u8(0x0f);
        self.mem.push_u8(0x10);
        self.mem.push_u8(0x48);
        self.mem.push_u8(value);
        offset
    }

    //movsd xmm1, [rsp+offset]
    pub fn movsd_xmm1_ptr_rsp_offset_u8(&mut self, value: i8) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0xf2);
        self.mem.push_u8(0x0f);
        self.mem.push_u8(0x10);
        self.mem.push_u8(0x4c);
        self.mem.push_u8(0x24);
        self.mem.push_u8(value as u8);
        offset
    }

    //movsd xmm0, [rsp+offset]
    pub fn movsd_xmm0_ptr_rsp_offset_u8(&mut self, value: i8) -> isize {
        let offset = self.mem.offset();
        self.mem.push_u8(0xf2);
        self.mem.push_u8(0x0f);
        self.mem.push_u8(0x10);
        self.mem.push_u8(0x44);
        self.mem.push_u8(0x24);
        self.mem.push_u8(value as u8);
        offset
    }
    
    //mov rbp, rsp
    pub fn mov_rbp_rsp(&mut self) -> isize {
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

    pub fn offset(&self) -> isize {
        self.mem.offset()
    }
}
