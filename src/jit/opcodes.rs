
//TODO Add Scratch0, Scratch1, etc
#[derive(Debug, Clone, Copy)]
pub enum Register {
    Param0,
    Param1,
}

//TODO Rename enum or file to Bytecode
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
    //TODO PushVar
    //TODO PopVar
    Test, //TODO REMOVE ME
}
