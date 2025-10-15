pub fn tokenize(input: &str) -> Vec<&str> {
    // Dummy lexer: split input by spaces
    input.split_whitespace().collect()
}
