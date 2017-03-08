extern crate libc;

mod jit;

//TODO Bytecode
//TODO Context, Variables
fn main() {
    let opcodes = vec![
        jit::Opcode::Nop,

        jit::Opcode::LoadVar(jit::Register::Param0, 1),
        jit::Opcode::LoadVar(jit::Register::Param1, 2),
        jit::Opcode::Test, //This returns a double
        jit::Opcode::SaveVar(3, jit::Register::Param0),
        jit::Opcode::LoadVar(jit::Register::Param1, 0),
        jit::Opcode::Test,
        jit::Opcode::LoadVar(jit::Register::Param0, 3),
        //jit::Opcode::LoadConst(jit::Register::Param0, 3.14),
        jit::Opcode::LoadVar(jit::Register::Param1, 0),
        jit::Opcode::Test,
    ];

    let frontend = jit::Frontend::new();
    let mem = frontend.jit(opcodes.as_slice());

    let vars: *mut f64 = unsafe { std::mem::transmute(libc::malloc(32*8)) };
    unsafe {*vars.offset(0) = 1.1; }
    unsafe {*vars.offset(1) = 2.2; }
    unsafe {*vars.offset(2) = 3.3; }
    let value = mem.execute(vars);
    unsafe { libc::free(std::mem::transmute(vars)) }
    println!("Value, {:?}", value);
}
