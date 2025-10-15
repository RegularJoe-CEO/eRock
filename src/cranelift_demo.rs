use cranelift::prelude::*;
use cranelift_codegen::ir::{Function, Signature};
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::settings::{Flags, OptLevel};
use cranelift_frontend::FunctionBuilder;
use cranelift_frontend::FunctionBuilderContext;
use cranelift_module::{Linkage, Module};
use cranelift_native;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup ISA and JIT Module (from Cranelift JIT demo)
    let isa_builder = cranelift_native::builder()?;
    let flags = Flags::new(OptLevel::Default);
    let isa = isa_builder.finish(flags)?;
    let libcall_names = Box::new(|_| Ok("".to_string()));
    let jit_builder = cranelift_jit::JITBuilder::new(libcall_names);
    let module = jit_builder.finish(isa)?;

    // Create context and builder
    let mut builder_ctx = FunctionBuilderContext::new();
    let mut ctx = module.make_context();

    // Signature: f64 input (x) -> f64 output
    let sig = Signature::new(CallConv::System);
    sig.params.push(AbiParam::new(types::F64));
    sig.returns.push(AbiRet(types::F64));
    ctx.func.signature = sig;

    // Build IR for "3.14 + x"
    let mut func = Function::new();
    let mut builder = FunctionBuilder::new(&mut func, &mut builder_ctx);
    let entry_block = builder.create_block();
    builder.switch_to_block(entry_block);
    builder.seal_block(entry_block);

    let param = builder.block_params(entry_block)[0]; // x param
    let const_3_14 = ctx.builder_mut().ins().f64const(3.14);
    let add = builder.ins().fadd(param, const_3_14);
    builder.ret(add);
    builder.finalize();

    // Compile to native
    let id = module.define_function("add_expr", Linkage::Export, &mut func)?;
    module.compile_function(&mut ctx)?;
    let code = module.get_finalized_function(id);

    // Execute
    type AddFn = extern "C" fn(f64) -> f64;
    let add_fn: AddFn = unsafe { std::mem::transmute(code) };
    let result = add_fn(2.0); // 3.14 + 2.0 = 5.14
    println!("Cranelift JIT Result: {}", result);

    // Benchmark speedup (10k evals)
    let mut interp_results = Vec::new();
    let start = Instant::now();
    for _ in 0..10000 {
        interp_results.push(3.14 + 2.0); // Interpreter sim
    }
    let interp_time = start.elapsed();
    println!("Interpreter 10k evals: {:?}", interp_time);

    let mut jit_results = Vec::new();
    let start = Instant::now();
    for _ in 0..10000 {
        jit_results.push(add_fn(2.0));
    }
    let jit_time = start.elapsed();
    println!("JIT 10k evals: {:?}", jit_time);
    let speedup = if jit_time.as_nanos() > 0 { interp_time.as_nanos() as f64 / jit_time.as_nanos() as f64 } else { 1.0 };
    println!("Speedup: ~{:.1}x", speedup);

    Ok(())
}
