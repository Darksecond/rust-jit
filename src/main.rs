extern crate libc;

mod jit;

// This should probably be sysv64
extern "C" fn test(a: u64, b: f64) -> u64{
    println!("Hello, World! {:?}", b);
    a+5
}

//TODO Frontend, Bytecode
//TODO Context, Variables
//TODO Figure out floating point (xmm0 onwards using SSE it seems)
fn main() {
    let mut backend = jit::Backend::new();

    backend.push_rbp();
    backend.mov_rsp_rbp();

    backend.mov_rax_u64(0x3ff0000000000006);
    backend.push_rax();
    backend.movsd_xmm0_ptr_rsp();
    backend.push_rax(); // hack to align stack to 16-bytes

    backend.mov_rdi_u32(14);
    backend.call(test as isize);
    backend.inc_rax();

    backend.leave(); //Clean up RSP, RBP
    backend.ret();

    println!("Backend, {:?}", backend);
    let jit = backend.mem();
    println!("Mem, {:?}", jit);

    let value = jit.execute();
    println!("Value, {:?}", value);
}
