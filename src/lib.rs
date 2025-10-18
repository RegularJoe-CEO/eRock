/*
SPDX-FileCopyrightText: 2025 Eric Waller
SPDX-License-Identifier: LicenseRef-eRock-Business-1.0
*/

pub mod interpreter;
pub mod lexer;
pub mod parser;

#[cfg(test)]
mod tests {
    use super::*; // Import things from the parent module (lib.rs)
    use crate::interpreter::interpret;
    use crate::lexer::tokenize;
    use crate::parser::parse;
    use std::collections::HashMap;

    #[test]
    fn test_simple_addition() {
        let input = "y = 10 + 5;";
        let tokens = tokenize(input);
        let (arena, root_idx) = parse(tokens).expect("Parsing failed");

        let mut variables = HashMap::new();
        let result = interpret(root_idx, &arena, &mut variables);

        // Assert that the result of "10 + 5" is 15.0
        assert_eq!(result, 15.0);
    }

    #[test]
    fn test_variable_evaluation() {
        let input = "y = a + b;";
        let tokens = tokenize(input);
        let (arena, root_idx) = parse(tokens).expect("Parsing failed");

        let mut variables = HashMap::new();
        variables.insert("a".to_string(), 7.0);
        variables.insert("b".to_string(), 3.0);

        let result = interpret(root_idx, &arena, &mut variables);

        // Assert that the result of "a + b" with a=7, b=3 is 10.0
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_operator_precedence() {
        let input = "y = 2 + 3 * 4;";
        let tokens = tokenize(input);
        let (arena, root_idx) = parse(tokens).expect("Parsing failed");

        let mut variables = HashMap::new();
        let result = interpret(root_idx, &arena, &mut variables);

        // Assert that the result of "2 + 3 * 4" is 14.0 (not 20.0)
        assert_eq!(result, 14.0);
    }

    #[test]
    fn test_parentheses() {
        let input = "y = (2 + 3) * 4;";
        let tokens = tokenize(input);
        let (arena, root_idx) = parse(tokens).expect("Parsing failed");

        let mut variables = HashMap::new();
        let result = interpret(root_idx, &arena, &mut variables);

        // Assert that the result of "(2 + 3) * 4" is 20.0
        assert_eq!(result, 20.0);
    }
}

mod jit_gate;
pub use jit_gate::{health_fields, jit_available, jit_enabled, jit_reason, license_valid};
