extern crate libc;

mod jit;

//TODO Bytecode
//TODO Context, Variables
//TODO Add JIT checking (uninitialized register usage, etc)
fn main() {
    let opcodes = vec![
        jit::Opcode::LoadVar(jit::Register::Param0, 0),
        jit::Opcode::Sin,
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
