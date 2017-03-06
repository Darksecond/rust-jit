
//TODO variable enum-ish maybe
//TODO Rename enum or file to Bytecode

//TODO Rename this to something FPU related
//TODO Add more regs
//TODO Add some scratch regs
#[derive(Debug, Clone, Copy)]
pub enum Register {
    Reg0,
    Reg1,
}

#[derive(Debug, Clone, Copy)]
// Order is always dst, src
pub enum Opcode {
    Nop,
    LoadVar(Register, u8), //reg, var
    LoadConst(Register, f64),
    //TODO SaveVar(var: u8, Register)
    //TODO Add(Register, Register),
    //TODO Sub(Register, Register),
    //TODO Mul(Register, Register),
    //TODO Div(Register, Register),
    Test, //TODO REMOVE ME
}
