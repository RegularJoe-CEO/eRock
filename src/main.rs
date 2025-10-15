mod lexer;
mod parser;
mod interpreter;

fn main() {
    let input = "sum = 3.14 + (x - 2) * 10";
    let tokens = lexer::tokenize(input);
    println!("Tokens: {:?}", tokens);

    if let Some((arena, root_idx)) = parser::parse(tokens) {
        if let Some(root) = arena.get(root_idx) {
            println!("AST: {:?}", root);
        }
        
        let mut variables = std::collections::HashMap::new();
        variables.insert("x".to_string(), 5.0);
        
        let result = interpreter::interpret(root_idx, &arena, &mut variables);
        println!("Scalar Result: {}", result);
        println!("Variables: {:?}", variables);
        
        // Test Batch (unrolled for speed)
        let batch_indices = vec![root_idx, root_idx]; // Duplicate for demo
        let batch_results = interpreter::batch_interpret(&batch_indices, &arena, &mut variables);
        println!("Batch Results: {:?}", batch_results);
        
        // Test Legacy JIT (triggers for simple nodes)
        if let Some(jit_fn) = interpreter::jit_eval(root_idx, &arena) {
            let jit_result = jit_fn(&mut variables);
            println!("Legacy JIT Result: {}", jit_result);
        } else {
            println!("Legacy JIT: Complex AST, using interpreter");
        }
    } else {
        println!("Parse error!");
    }
}
