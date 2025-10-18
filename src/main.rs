/*
SPDX-FileCopyrightText: 2025 Eric Waller
SPDX-License-Identifier: LicenseRef-eRock-Business-1.0
*/

mod interpreter;
mod lexer;
mod parser;

use crate::lexer::Token;
use std::collections::HashMap;

fn main() {
    let input = "sum = 3.14 + (x - 2) * 10";
    let tokens = lexer::tokenize(input);
    println!("Tokens: {:?}", tokens);

    if let Some((arena, root_idx)) = parser::parse(tokens) {
        if let Some(root) = arena.get(root_idx) {
            println!("AST: {:?}", root);
        }

        let mut variables = HashMap::new();
        variables.insert("x".to_string(), 5.0);

        let result = interpreter::interpret(root_idx, &arena, &mut variables);
        println!("Scalar Result: {}", result);
        println!("Variables: {:?}", variables);

        let batch_indices = vec![root_idx, root_idx];
        let batch_results = interpreter::batch_interpret(&batch_indices, &arena, &mut variables);
        println!("Batch Results: {:?}", batch_results);

        if let Some(jit_fn) = interpreter::jit_eval(root_idx, &arena) {
            let jit_result = jit_fn(&mut variables);
            println!("Legacy JIT Result: {}", jit_result);
        } else {
            println!("Legacy JIT: Complex AST, using interpreter");
        }

        match interpreter::ExprJit::new(3.14, Token::Plus, 30.0) {
            Ok(j) => println!("Cranelift JIT demo (3.14 + 30.0) = {}", j.eval()),
            Err(e) => eprintln!("Cranelift JIT error: {}", e),
        }

        // SIMD demo with pretty-print to 2 decimals
        let xs = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let simd_results = interpreter::simd_eval_over_x(root_idx, &arena, &variables, &xs);
        let pretty = simd_results
            .iter()
            .map(|v| format!("{:.2}", v))
            .collect::<Vec<_>>()
            .join(", ");
        println!("SIMD over x (2dp): [{}]", pretty);
    } else {
        println!("Parse error!");
    }
}
