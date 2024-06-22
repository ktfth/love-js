use ress::tokens::{Token, Token::*, Punct as Pct};
use std::string::String;
use std::str::FromStr;
use ress::prelude::TokenExt;

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
}

// Função de exemplo para construir AST a partir de tokens
pub fn parse_expression(tokens: Vec<Token<&str>>) -> Result<Expr, String> {
    if tokens.len() == 5 {
        if let (Number(left), op, Number(right)) = (&tokens[0], &tokens[1], &tokens[2]) {
            if op.matches_punct(Pct::Plus) {
                let left_value = i64::from_str(left.to_string().as_str()).map_err(|e| e.to_string())?;
                let right_value = i64::from_str(right.to_string().as_str()).map_err(|e| e.to_string())?;
                return Ok(Expr::Add(Box::new(Expr::Number(left_value)), Box::new(Expr::Number(right_value))));
            }
        }
    }
    Err("Invalid expression".to_string())
}
