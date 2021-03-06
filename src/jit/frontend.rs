use super::memory::*;
use super::backend::*;
use super::opcodes::*;

use std::mem;

extern "C" fn sin(input: f64) -> f64 {
    println!("sin {:?}", input);
    f64::sin(input)
}

extern "C" fn test(b: f64, c: f64) -> f64 {
    println!("Hello, World! {:?} {:?}", b, c);
    b*c
}

#[derive(Debug)]
pub struct Frontend {
    backend: Backend,
    offset: usize
}

impl Frontend {
    pub fn new() -> Frontend {
        Frontend {
            backend: Backend::new(),
            offset: 0
        }
    }

    //RAX is being treated as a scratch register
    //RDI is being treated as a scratch register
    //TODO var*8 might (very likely) be more than u8 -> Move to u16/u32
    pub fn jit(mut self, opcodes: &[Opcode]) -> Memory {
        self.backend.push_rbp();
        self.backend.mov_rbp_rsp();
        self.backend.push_rdi(); //FIRST PARAMETER ([rbp-8])
        self.backend.sub_rsp_u8(8); //ALIGN STACK
        
        while self.offset < opcodes.len() {
            let opcode = opcodes[self.offset];
            self.offset += 1;
            
            //TODO Move opcodes out into their own files, or at least functions
            match opcode {
                Opcode::Nop => (),
                Opcode::LoadVar(reg, var) => {
                    self.backend.mov_rax_ptr_rbp_offset_u8(-8);
                    match reg {
                        Register::Param0 => self.backend.movsd_xmm0_ptr_rax_offset_u8(var*8), //8 is sizeof f64
                        Register::Param1 => self.backend.movsd_xmm1_ptr_rax_offset_u8(var*8), //8 is sizeof f64
                    };
                },
                Opcode::LoadConst(reg, value) => {
                    unsafe { self.backend.mov_rax_u64(mem::transmute(value)); }
                    self.backend.push_rax();
                    match reg {
                        Register::Param0 => self.backend.movsd_xmm0_ptr_rsp(),
                        Register::Param1 => self.backend.movsd_xmm1_ptr_rsp(),
                    };
                    self.backend.add_rsp_u8(8);
                },
                Opcode::SaveVar(var, reg) => {
                    self.backend.mov_rax_ptr_rbp_offset_u8(-8); //Load address of vars into rax
                    self.backend.sub_rsp_u8(8); // Make space on stack
                    match reg {
                        Register::Param0 => self.backend.movsd_ptr_rsp_xmm0(),
                        Register::Param1 => self.backend.movsd_ptr_rsp_xmm1(),
                    };
                    self.backend.pop_ptr_rax_offset_u8(var*8);
                },
                Opcode::PushVar(var) => {
                    self.backend.mov_rax_ptr_rbp_offset_u8(-8); //Load address of vars into rax
                    self.backend.sub_rsp_u8(8); //Align stack
                    self.backend.push_ptr_rax_offset_u8(var*8);
                },
                Opcode::PopVar(var) => {
                    self.backend.mov_rax_ptr_rbp_offset_u8(-8); //Load address of vars into rax
                    self.backend.pop_ptr_rax_offset_u8(var*8);
                    self.backend.add_rsp_u8(8);
                },
                Opcode::PushReg(reg) => {
                    self.backend.sub_rsp_u8(16);
                    match reg {
                        Register::Param0 => self.backend.movsd_ptr_rsp_xmm0(),
                        Register::Param1 => self.backend.movsd_ptr_rsp_xmm1(),
                    };
                },
                Opcode::PopReg(reg) => {
                    match reg {
                        Register::Param0 => self.backend.movsd_xmm0_ptr_rsp(),
                        Register::Param1 => self.backend.movsd_xmm1_ptr_rsp(),
                    };
                    self.backend.add_rsp_u8(16);
                },
                Opcode::Sin => {
                    self.backend.call(sin as isize);
                },
                Opcode::Test => {
                    self.backend.call(test as isize);
                },
            }
        }

        self.backend.leave(); //Clean up RSP, RBP
        self.backend.ret();

        self.to_mem()
    }

    fn to_mem(self) -> Memory {
        self.backend.to_mem()
    }
}
