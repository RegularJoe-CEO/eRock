use std::collections::HashMap;

#[derive(Debug, Clone)]
pub pub enum Token {
    Number(f64),
    Variable(String),
    Operator(String),
}

#[derive(Debug, Clone)]
pub pub enum ASTNode {
    Number(f64),
    Variable(String),
    Binary(Box<ASTNode>, String, Box<ASTNode>),
    Unary(String, Box<ASTNode>),
    Assignment(String, Box<ASTNode>),
    Paren(Box<ASTNode>),
}

pub pub fn eval(expression: &str, variables: &mut HashMap<String, Vec<f64>>) -> Result<f64, String> {
    println!("DEBUG eval: input '{}'", expression);
    match tokens(expression) {
        Ok(tokens) => {
            println!("DEBUG eval: tokens OK, len {}", tokens.len());
            match parse(&tokens) {
                Ok(ast) => {
                    println!("DEBUG eval: parse OK");
                    #[cfg(all(feature = "jit", target_arch = "x86_64"))]
                    {
                        match try_jit(&ast, variables) {
                            Ok(jit_result) => {
                                println!("DEBUG eval: JIT OK");
                                return Ok(jit_result);
                            }
                            Err(_) => println!("DEBUG eval: JIT fallback"),
                        }
                    }
                    match interpret(&ast, variables) {
                        Ok(result) => {
                            println!("DEBUG eval: interpret OK, result {}", result);
                            Ok(result)
                        }
                        Err(e) => {
                            println!("DEBUG eval: interpret Err '{}'", e);
                            Err(e)
                        }
                    }
                }
                Err(e) => {
                    println!("DEBUG eval: parse Err '{}'", e);
                    Err(e)
                }
            }
        }
        Err(e) => {
            println!("DEBUG eval: tokens Err '{}'", e);
            Err(e)
        }
    }
}

pub pub fn tokens(expr: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = expr.chars().peekable();
    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' => { chars.next(); continue; }
            '0'..='9' | '.' | '-' => {
                if let Some(num) = parse_number(&mut chars) {
                    tokens.push(Token::Number(num));
                } else {
                    return Err("Invalid number".to_string());
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                if let Some(var) = parse_variable(&mut chars) {
                    tokens.push(Token::Variable(var));
                } else {
                    return Err("Invalid variable".to_string());
                }
            }
            '+' | '-' | '*' | '/' | '^' | '(' | ')' | '=' => {
                tokens.push(Token::Operator(chars.next().unwrap().to_string()));
            }
            _ => return Err(format!("Unexpected character: {}", ch)),
        }
    }
    println!("DEBUG tokens: produced {} tokens", tokens.len());
    Ok(tokens)
}

pub fn parse_number(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<f64> {
    let mut num_str = String::new();
    if let Some(&ch) = chars.peek() {
        if ch == '-' {
            num_str.push(chars.next().unwrap());
        }
    }
    while let Some(&ch) = chars.peek() {
        if ('0'..='9').contains(&ch) || ch == '.' {
            num_str.push(chars.next().unwrap());
        } else {
            break;
        }
    }
    num_str.parse().ok()
}

pub fn parse_variable(chars: &mut std::iter::Peekable<std::str::Chars>) -> Option<String> {
    let mut var = String::new();
    while let Some(&ch) = chars.peek() {
        match ch {
            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => var.push(chars.next().unwrap()),
            _ => break,
        }
    }
    if var.is_empty() { None } else { Some(var) }
}

pub pub fn parse(tokens: &Vec<Token>) -> Result<ASTNode, String> {
    println!("DEBUG parse: tokens {:?}", tokens);
    let mut i = 0;
    match parse_expression(tokens, &mut i, tokens.len()) {
        Ok(node) => {
            if i != tokens.len() {
                println!("DEBUG parse: extra tokens after i={}", i);
                Err("Extra tokens after expression".to_string())
            } else {
                println!("DEBUG parse: OK");
                Ok(node)
            }
        }
        Err(e) => {
            println!("DEBUG parse: Err '{}' at i={}", e, i);
            Err(e)
        }
    }
}

pub fn parse_expression(tokens: &Vec<Token>, i: &mut usize, end: usize) -> Result<ASTNode, String> {
    let mut node = parse_term(tokens, i, end)?;
    while *i < end {
        if let Token::Operator(ref op) = tokens[*i] {
            if op == "+" || op == "-" {
                let op_str = op.clone();
                *i += 1;
                let right = parse_term(tokens, i, end)?;
                node = ASTNode::Binary(Box::new(node), op_str, Box::new(right));
            } else {
                break;
            }
        } else {
            break;
        }
    }
    Ok(node)
}

pub fn parse_term(tokens: &Vec<Token>, i: &mut usize, end: usize) -> Result<ASTNode, String> {
    let mut node = parse_factor(tokens, i, end)?;
    while *i < end {
        if let Token::Operator(ref op) = tokens[*i] {
            if op == "*" || op == "/" || op == "^" {
                let op_str = op.clone();
                *i += 1;
                let right = parse_factor(tokens, i, end)?;
                node = ASTNode::Binary(Box::new(node), op_str, Box::new(right));
            } else {
                break;
            }
        } else {
            break;
        }
    }
    Ok(node)
}

pub fn parse_factor(tokens: &Vec<Token>, i: &mut usize, end: usize) -> Result<ASTNode, String> {
    if *i >= end {
        return Err("Unexpected end of input".to_string());
    }
    println!("DEBUG parse_factor: at i={}, token {:?}", *i, tokens[*i]);
    match &tokens[*i] {
        Token::Number(n) => {
            *i += 1;
            Ok(ASTNode::Number(*n))
        }
        Token::Variable(v) => {
            let var_name = v.clone();
            *i += 1;
            if *i < end && matches!(tokens[*i], Token::Operator(ref o) if o == "=") {
                *i += 1;
                let expr = parse_expression(tokens, i, end)?;
                Ok(ASTNode::Assignment(var_name, Box::new(expr)))
            } else {
                Ok(ASTNode::Variable(var_name))
            }
        }
        Token::Operator(op) if op == "-" => {
            *i += 1;
            let child = parse_factor(tokens, i, end)?;
            Ok(ASTNode::Unary(op.clone(), Box::new(child)))
        }
        Token::Operator(op) if op == "(" => {
            *i += 1;
            let expr = parse_expression(tokens, i, end)?;
            if *i < end && matches!(tokens[*i], Token::Operator(ref o) if o == ")") {
                *i += 1;
                Ok(ASTNode::Paren(Box::new(expr)))
            } else {
                Err("Mismatched parentheses".to_string())
            }
        }
        _ => Err(format!("Invalid factor at i={}: {:?}", *i, tokens[*i])),
    }
}

pub pub fn interpret(ast: &ASTNode, variables: &mut HashMap<String, Vec<f64>>) -> Result<f64, String> {
    println!("DEBUG interpret: ast {:?}", ast);
    match ast {
        ASTNode::Number(n) => Ok(*n),
        ASTNode::Variable(ref v) => {
            match variables.get(v) {
                Some(vec) => match vec.first() {
                    Some(&val) => Ok(val),
                    None => Err(format!("Empty vec for variable: {}", v)),
                },
                None => Err(format!("Undefined variable: {}", v)),
            }
        }
        ASTNode::Binary(left, op, right) => {
            let l = interpret(left, variables)?;
            let r = interpret(right, variables)?;
            match op.as_str() {
                "+" => Ok(l + r),
                "-" => Ok(l - r),
                "*" => Ok(l * r),
                "/" => if r.abs() > f64::EPSILON { Ok(l / r) } else { Err("Division by zero".to_string()) },
                "^" => Ok(l.powf(r)),
                _ => Err(format!("Unknown operator: {}", op)),
            }
        }
        ASTNode::Unary(op, child) => {
            let c = interpret(child, variables)?;
            match op.as_str() {
                "-" => Ok(-c),
                _ => Err(format!("Unknown unary operator: {}", op)),
            }
        }
        ASTNode::Assignment(ref v, expr) => {
            let value = interpret(expr, variables)?;
            variables.insert(v.clone(), vec![value]);
            println!("DEBUG interpret: assigned {} = {}", v, value);
            Ok(value)
        }
        ASTNode::Paren(inner) => interpret(inner, variables),
    }
}

#[cfg(all(feature = "jit", target_arch = "x86_64"))]
pub fn try_jit(_ast: &ASTNode, _variables: &mut HashMap<String, Vec<f64>>) -> Result<f64, String> {
    Err("JIT not implemented".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_precedence() {
        let expr = "2 + 3 * 4";
        let mut variables = HashMap::new();
        match eval(expr, &mut variables) {
            Ok(result) => assert_eq!(result, 14.0),
            Err(e) => panic!("Eval failed: {}", e),
        }
    }

    #[test]
    pub fn test_variables() {
        let expr = "x + y";
        let mut variables = HashMap::new();
        variables.insert("x".to_string(), vec![5.0]);
        variables.insert("y".to_string(), vec![10.0]);
        match eval(expr, &mut variables) {
            Ok(result) => assert_eq!(result, 15.0),
            Err(e) => panic!("Eval failed: {}", e),
        }
    }

    #[test]
    pub fn test_batch_variables() {
        let expr = "x + y";
        let mut variables = HashMap::new();
        variables.insert("x".to_string(), vec![1.0, 2.0]);
        variables.insert("y".to_string(), vec![3.0, 4.0]);
        match eval(expr, &mut variables) {
            Ok(result) => assert_eq!(result, 4.0),
            Err(e) => panic!("Eval failed: {}", e),
        }
    }

    #[test]
    pub fn test_assignment() {
        let expr = "z = 7 + 3";
        let mut variables = HashMap::new();
        match eval(expr, &mut variables) {
            Ok(result) => {
                assert_eq!(result, 10.0);
                assert!(variables.contains_key("z"));
            }
            Err(e) => panic!("Eval failed: {}", e),
        }
    }
}
