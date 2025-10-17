/*
SPDX-FileCopyrightText: 2025 Eric Waller
SPDX-License-Identifier: LicenseRef-eRock-Business-1.0
*/

use crate::lexer::Token;
use crate::parser::{Arena, ExprKind};
use std::collections::HashMap;

// ========== Scalar interpreter ==========
pub fn interpret(root_idx: usize, arena: &Arena, variables: &mut HashMap<String, f64>) -> f64 {
    interpret_node(root_idx, arena, variables)
}

fn interpret_node(idx: usize, arena: &Arena, variables: &mut HashMap<String, f64>) -> f64 {
    if let Some(expr) = arena.get(idx) {
        match &expr.kind {
            ExprKind::Number(n) => *n,
            ExprKind::Identifier(name) => variables.get(name).copied().unwrap_or(0.0),
            ExprKind::Binary { left, op, right } => {
                let l = interpret_node(*left, arena, variables);
                let r = interpret_node(*right, arena, variables);
                match op {
                    Token::Plus  => l + r,
                    Token::Minus => l - r,
                    Token::Star  => l * r,
                    Token::Slash => if r != 0.0 { l / r } else { f64::NAN },
                    _ => f64::NAN,
                }
            }
            ExprKind::Assign { name, value } => {
                let v = interpret_node(*value, arena, variables);
                variables.insert(name.clone(), v);
                v
            }
        }
    } else {
        f64::NAN
    }
}

// ========== Batch (simple) ==========
pub fn batch_interpret(root_indices: &[usize], arena: &Arena, variables: &mut HashMap<String, f64>) -> Vec<f64> {
    let mut out = Vec::with_capacity(root_indices.len());
    for &idx in root_indices {
        out.push(interpret_node(idx, arena, variables));
    }
    out
}

// ========== Cranelift JIT (demo) ==========
use cranelift::prelude::*;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};
use cranelift_codegen::isa::CallConv;

#[inline]
fn target_callconv() -> CallConv {
    #[cfg(all(target_arch = "aarch64", target_vendor = "apple"))]
    { CallConv::AppleAarch64 }
    #[cfg(not(all(target_arch = "aarch64", target_vendor = "apple")))]
    { CallConv::SystemV }
}

pub struct ExprJit {
    _module: JITModule,
    func: extern "C" fn() -> f64,
}

impl ExprJit {
    pub fn new(left: f64, op: Token, right: f64) -> Result<Self, String> {
        let builder = JITBuilder::new(cranelift_module::default_libcall_names())
            .map_err(|e| e.to_string())?;
        let mut module = JITModule::new(builder);

        let mut ctx = module.make_context();
        ctx.func.signature = Signature::new(target_callconv());
        ctx.func.signature.returns.push(AbiParam::new(types::F64));

        let mut fctx = FunctionBuilderContext::new();
        let mut fb = FunctionBuilder::new(&mut ctx.func, &mut fctx);
        let entry = fb.create_block();
        fb.switch_to_block(entry);
        fb.seal_block(entry);

        let l = fb.ins().f64const(left);
        let r = fb.ins().f64const(right);
        let res = match op {
            Token::Plus  => fb.ins().fadd(l, r),
            Token::Minus => fb.ins().fsub(l, r),
            Token::Star  => fb.ins().fmul(l, r),
            Token::Slash => fb.ins().fdiv(l, r),
            _ => l,
        };
        fb.ins().return_(&[res]);
        fb.finalize();

        let func_id = module
            .declare_function("expr_jit", Linkage::Export, &ctx.func.signature)
            .map_err(|e| e.to_string())?;
        module.define_function(func_id, &mut ctx).map_err(|e| e.to_string())?;
        module.clear_context(&mut ctx);
        module.finalize_definitions().map_err(|e| e.to_string())?;

        let code_ptr = module.get_finalized_function(func_id);
        let func: extern "C" fn() -> f64 =
            unsafe { std::mem::transmute::<*const u8, extern "C" fn() -> f64>(code_ptr) };

        Ok(ExprJit { _module: module, func })
    }

    pub fn eval(&self) -> f64 { (self.func)() }
}

// ========== Stable SIMD using wide::f64x4 ==========
use wide::f64x4;
type Vf64 = f64x4;

// SIMD evaluator for a given x vector. Other identifiers are splats.
fn interpret_node_simd(idx: usize, arena: &Arena, variables: &HashMap<String, f64>, x: Vf64) -> Vf64 {
    if let Some(expr) = arena.get(idx) {
        match &expr.kind {
            ExprKind::Number(n) => Vf64::splat(*n),
            ExprKind::Identifier(name) => {
                if name == "x" { x } else { Vf64::splat(*variables.get(name).unwrap_or(&0.0)) }
            }
            ExprKind::Binary { left, op, right } => {
                let l = interpret_node_simd(*left, arena, variables, x);
                let r = interpret_node_simd(*right, arena, variables, x);
                match op {
                    Token::Plus  => l + r,
                    Token::Minus => l - r,
                    Token::Star  => l * r,
                    Token::Slash => l / r,
                    _ => l,
                }
            }
            ExprKind::Assign { value, .. } => {
                // Pure-eval (no mutation) for throughput
                interpret_node_simd(*value, arena, variables, x)
            }
        }
    } else {
        Vf64::splat(f64::NAN)
    }
}

// Evaluate across a slice of x values using 4‑wide lanes.
pub fn simd_eval_over_x(root_idx: usize, arena: &Arena, variables: &HashMap<String, f64>, xs: &[f64]) -> Vec<f64> {
    let n = xs.len();
    let mut out = Vec::with_capacity(n);

    let mut i = 0;
    while i < n {
        let mut buf = [0.0f64; 4];
        let mut count = 0;
        while count < 4 && (i + count) < n {
            buf[count] = xs[i + count];
            count += 1;
        }
        if count < 4 {
            let pad = if count > 0 { buf[count - 1] } else { *xs.last().unwrap_or(&0.0) };
            for j in count..4 { buf[j] = pad; }
        }

        let x = Vf64::from(buf);
        let v = interpret_node_simd(root_idx, arena, variables, x);
        let arr: [f64; 4] = v.into(); // wide 0.7 supports Into<[f64;4]>
        for j in 0..count { out.push(arr[j]); }
        i += count;
    }
    out
}

// ========== Legacy micro‑JIT (const fold to closure) ==========
pub fn jit_eval(root_idx: usize, arena: &Arena) -> Option<Box<dyn Fn(&mut HashMap<String, f64>) -> f64 + Send + Sync + 'static>> {
    let expr = arena.get(root_idx)?;
    match &expr.kind {
        ExprKind::Number(n) => {
            let v = *n;
            Some(Box::new(move |_| v))
        }
        ExprKind::Binary { left, op, right } => {
            let l = arena.get(*left)?;
            let r = arena.get(*right)?;
            if let (ExprKind::Number(a), ExprKind::Number(b)) = (&l.kind, &r.kind) {
                let (a, b) = (*a, *b);
                match op {
                    Token::Plus  => Some(Box::new(move |_| a + b)),
                    Token::Minus => Some(Box::new(move |_| a - b)),
                    Token::Star  => Some(Box::new(move |_| a * b)),
                    Token::Slash => Some(Box::new(move |_| if b != 0.0 { a / b } else { f64::NAN })),
                    _ => None,
                }
            } else {
                None
            }
        }
        _ => None,
    }
}
