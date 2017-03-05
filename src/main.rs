extern crate libc;

mod jit;

// This should probably be sysv64
extern "C" fn test(a: u64, b: f64) -> u64{
    println!("Hello, World! {:?}", b);
    a+5
}

//TODO Frontend, Bytecode
//TODO Context, Variables
fn main() {
    let mut backend = jit::Backend::new();

    backend.push_rbp();
    backend.mov_rsp_rbp();

    backend.movsd_xmm0_ptr_rdi_offset_u8(2*8);

    backend.mov_rdi_u32(14);
    backend.call(test as isize);
    backend.inc_rax();

    backend.leave(); //Clean up RSP, RBP
    backend.ret();

    println!("Backend, {:?}", backend);
    let jit = backend.to_mem();
    println!("Mem, {:?}", jit);

    let vars: *mut f64 = unsafe { std::mem::transmute(libc::malloc(32*8)) };
    unsafe {*vars.offset(0) = 1.1; }
    unsafe {*vars.offset(1) = 2.2; }
    unsafe {*vars.offset(2) = 3.3; }
    let value = jit.execute(vars);
    unsafe { libc::free(std::mem::transmute(vars)) }
    println!("Value, {:?}", value);
}
