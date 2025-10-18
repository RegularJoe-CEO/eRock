/*
SPDX-FileCopyrightText: 2025 Eric Waller
SPDX-License-Identifier: LicenseRef-eRock-Business-1.0
*/

use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum ExprKind {
    Number(f64),
    Identifier(String),
    Binary {
        left: usize,
        op: Token,
        right: usize,
    },
    Assign {
        name: String,
        value: usize,
    },
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
}

pub struct Arena {
    nodes: Vec<Expr>,
}

impl Arena {
    pub fn new() -> Self {
        Arena { nodes: Vec::new() }
    }

    pub fn alloc(&mut self, kind: ExprKind) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(Expr { kind });
        idx
    }

    pub fn get(&self, idx: usize) -> Option<&Expr> {
        self.nodes.get(idx)
    }
}

struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            Some(&self.tokens[self.pos])
        } else {
            None
        }
    }

    fn next(&mut self) -> Option<Token> {
        if self.pos < self.tokens.len() {
            let token = self.tokens[self.pos].clone();
            self.pos += 1;
            Some(token)
        } else {
            None
        }
    }

    fn save_pos(&self) -> usize {
        self.pos
    }

    fn restore_pos(&mut self, pos: usize) {
        self.pos = pos;
    }

    fn eat(&mut self, expected: Token) -> bool {
        if let Some(token) = self.peek() {
            if *token == expected {
                self.next();
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Option<(Arena, usize)> {
    let mut arena = Arena::new();
    let mut parser = Parser::new(&tokens);
    let root = parse_assignment(&mut parser, &mut arena)?;
    Some((arena, root))
}

fn parse_assignment(parser: &mut Parser, arena: &mut Arena) -> Option<usize> {
    let saved_pos = parser.save_pos();

    if let Some(first) = parser.peek() {
        if let Token::Identifier(name) = first.clone() {
            parser.next(); // Consume identifier

            if let Some(second) = parser.peek() {
                if let Token::Unknown('=') = second.clone() {
                    parser.next(); // Consume '='

                    if let Some(value_idx) = parse_expr(parser, arena) {
                        let idx = arena.alloc(ExprKind::Assign {
                            name,
                            value: value_idx,
                        });
                        return Some(idx);
                    }
                }
            }

            // Not an assignment—restore and parse expression
            parser.restore_pos(saved_pos);
        }
    }

    parse_expr(parser, arena)
}

fn parse_expr(parser: &mut Parser, arena: &mut Arena) -> Option<usize> {
    parse_term(parser, arena)
}

fn parse_term(parser: &mut Parser, arena: &mut Arena) -> Option<usize> {
    let mut left_idx = parse_factor(parser, arena)?;

    while let Some(token) = parser.peek().cloned() {
        match token {
            Token::Plus | Token::Minus => {
                let op = parser.next().unwrap();
                let right_idx = parse_factor(parser, arena)?;
                left_idx = arena.alloc(ExprKind::Binary {
                    left: left_idx,
                    op,
                    right: right_idx,
                });
            }
            _ => break,
        }
    }
    Some(left_idx)
}

fn parse_factor(parser: &mut Parser, arena: &mut Arena) -> Option<usize> {
    let mut left_idx = parse_primary(parser, arena)?;

    while let Some(token) = parser.peek().cloned() {
        match token {
            Token::Star | Token::Slash => {
                let op = parser.next().unwrap();
                let right_idx = parse_primary(parser, arena)?;
                left_idx = arena.alloc(ExprKind::Binary {
                    left: left_idx,
                    op,
                    right: right_idx,
                });
            }
            _ => break,
        }
    }
    Some(left_idx)
}

fn parse_primary(parser: &mut Parser, arena: &mut Arena) -> Option<usize> {
    match parser.next()? {
        Token::Number(n) => Some(arena.alloc(ExprKind::Number(n))),
        Token::Identifier(name) => Some(arena.alloc(ExprKind::Identifier(name))),
        Token::LParen => {
            let expr_idx = parse_expr(parser, arena);
            if parser.eat(Token::RParen) {
                expr_idx
            } else {
                None
            }
        }
        _ => None,
    }
}
