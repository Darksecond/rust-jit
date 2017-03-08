
//TODO Add Scratch0, Scratch1, etc
#[derive(Debug, Clone, Copy)]
pub enum Register {
    Param0,
    Param1,
}

pub type Variable = u8;

//TODO Rename enum or file to Bytecode
// Order is always dst, src
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Nop,
    LoadVar(Register, Variable),
    LoadConst(Register, f64),
    SaveVar(Variable, Register),
    //TODO Add(Register, Register),
    //TODO Sub(Register, Register),
    //TODO Mul(Register, Register),
    //TODO Div(Register, Register),
    //TODO PushVar(Variable),
    //TODO PopVar(Variable),
    //TODO PushReg(Register),
    //TODO PopReg(Register),
    Test, //TODO REMOVE ME
}
