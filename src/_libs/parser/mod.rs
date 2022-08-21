// An attribute to hide warnings for unused code.
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![warn(non_snake_case)]

use crate::tokenizer::token::{token_value_types, Token};

mod node;

pub struct Parser {
    _tokens: Vec<Token<token_value_types>>,
    _token_idx: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token<token_value_types>>) -> Self {
        Parser {
            _tokens: tokens,
            _token_idx: 0usize,
        }
    }

    pub fn term() {}

    pub fn factor() {}

    pub fn expression() {}
}
