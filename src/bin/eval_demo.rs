/*
SPDX-FileCopyrightText: 2025 Eric Waller
SPDX-License-Identifier: LicenseRef-eRock-Business-1.0
*/

use erock::{interpreter, lexer, parser};
use std::collections::HashMap;

fn main() {
    // 1) Build an AST from a formula
    let input = "sum = 3.14 + (x - 2) * 10";
    let tokens = lexer::tokenize(input);
    let (arena, root) = parser::parse(tokens).expect("parse error");

    // 2) Scalar example (single x)
    let mut vars = HashMap::new();
    vars.insert("x".to_string(), 5.0);
    let y = interpreter::interpret(root, &arena, &mut vars);
    println!("scalar for x=5 -> {:.2}", y);

    // 3) SIMD batch example (evaluate over many x values)
    let xs: Vec<f64> = (0..20).map(|i| i as f64 * 0.5).collect(); // 0.0, 0.5, 1.0, ...
    let vars2: HashMap<String, f64> = HashMap::new();
    let ys = interpreter::simd_eval_over_x(root, &arena, &vars2, &xs);

    let pretty: Vec<String> = ys.iter().map(|v| format!("{:.2}", v)).collect();
    println!("SIMD over x (2dp): [{}]", pretty.join(", "));
}
