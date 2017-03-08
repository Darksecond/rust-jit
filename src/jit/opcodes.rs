
//TODO Add Scratch0, Scratch1, etc
//TODO Add Param2, Param3, etc
#[derive(Debug, Clone, Copy)]
pub enum Register {
    Param0,
    Param1,
}

//TODO Should this be a enum or struct?
//TODO Should this be a u8, u16 or u32?
pub type Variable = u8;

//TODO Rename enum or file to Bytecode
// Order is always dst, src
// Right now the idea is that functions _will_ clobber the registers
// This includes operations like sin/cos/tan
// The exception are some math operations like Add/Sub/Mul/Div
// You can use PushReg/PopReg to save the registers you are using to the stack
// You can also use SaveVar instead
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Nop,
    LoadVar(Register, Variable),
    LoadConst(Register, f64),
    SaveVar(Variable, Register),
    PushVar(Variable),
    PopVar(Variable),
    PushReg(Register),
    PopReg(Register),
    Sin, //This will input and output to/from Param0
    //TODO Add(Register, Register),
    //TODO Sub(Register, Register),
    //TODO Mul(Register, Register),
    //TODO Div(Register, Register),
    //TODO Move(Register, Register),
    Test, //TODO REMOVE ME
}
