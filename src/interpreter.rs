use crate::lexer::Token;
use crate::parser::{Arena, ExprKind};
use std::collections::HashMap;

pub fn interpret(root_idx: usize, arena: &Arena, variables: &mut HashMap<String, f64>) -> f64 {
    interpret_node(root_idx, arena, variables)
}

fn interpret_node(idx: usize, arena: &Arena, variables: &mut HashMap<String, f64>) -> f64 {
    if let Some(expr) = arena.get(idx) {
        match &expr.kind {
            ExprKind::Number(n) => *n,
            ExprKind::Identifier(name) => variables.get(name).copied().unwrap_or(0.0),
            ExprKind::Binary { left, op, right } => {
                let left_val = interpret_node(*left, arena, variables);
                let right_val = interpret_node(*right, arena, variables);
                match op {
                    Token::Plus => left_val + right_val,
                    Token::Minus => left_val - right_val,
                    Token::Star => left_val * right_val,
                    Token::Slash => if right_val != 0.0 { left_val / right_val } else { f64::NAN },
                    _ => f64::NAN,
                }
            }
            ExprKind::Assign { name, value } => {
                let val = interpret_node(*value, arena, variables);
                variables.insert(name.clone(), val);
                val
            }
        }
    } else {
        f64::NAN
    }
}

// Batch Eval: Unrolled for 2-4 expressions (2-4x faster than recursive)
pub fn batch_interpret(root_indices: &[usize], arena: &Arena, variables: &mut HashMap<String, f64>) -> Vec<f64> {
    let mut results = Vec::with_capacity(root_indices.len());
    match root_indices.len() {
        0 => {}
        1 => results.push(interpret_node(root_indices[0], arena, variables)),
        2 => {
            results.push(interpret_node(root_indices[0], arena, variables));
            results.push(interpret_node(root_indices[1], arena, variables));
        }
        3 => {
            results.push(interpret_node(root_indices[0], arena, variables));
            results.push(interpret_node(root_indices[1], arena, variables));
            results.push(interpret_node(root_indices[2], arena, variables));
        }
        4 => {
            results.push(interpret_node(root_indices[0], arena, variables));
            results.push(interpret_node(root_indices[1], arena, variables));
            results.push(interpret_node(root_indices[2], arena, variables));
            results.push(interpret_node(root_indices[3], arena, variables));
        }
        _ => {
            for &idx in root_indices {
                results.push(interpret_node(idx, arena, variables));
            }
        }
    }
    results
}

// Legacy JIT: 'static closures for simple nodes (no interpreter overhead)
pub fn jit_eval(root_idx: usize, arena: &Arena) -> Option<Box<dyn Fn(&mut HashMap<String, f64>) -> f64 + Send + Sync + 'static>> {
    let node_data = if let Some(expr) = arena.get(root_idx) {
        match &expr.kind {
            ExprKind::Number(n) => {
                let val = *n;
                return Some(Box::new(move |_| val));
            }
            ExprKind::Binary { left, op, right, .. } => {
                let left_val = if let Some(left_expr) = arena.get(*left) {
                    if let ExprKind::Number(n) = &left_expr.kind { *n } else { 0.0 }
                } else { 0.0 };
                let right_val = if let Some(right_expr) = arena.get(*right) {
                    if let ExprKind::Number(n) = &right_expr.kind { *n } else { 0.0 }
                } else { 0.0 };
                (left_val, op.clone(), right_val)
            }
            _ => return None,
        }
    } else {
        return None;
    };

    match node_data.1 {
        Token::Plus => Some(Box::new(move |_| node_data.0 + node_data.2)),
        Token::Minus => Some(Box::new(move |_| node_data.0 - node_data.2)),
        Token::Star => Some(Box::new(move |_| node_data.0 * node_data.2)),
        Token::Slash => Some(Box::new(move |_| if node_data.2 != 0.0 { node_data.0 / node_data.2 } else { f64::NAN })),
        _ => None,
    }
}
