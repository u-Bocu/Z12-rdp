// An attribute to hide warnings for unused code.
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![warn(non_snake_case)]

use crate::tokenizer::token::{token_value_types, Token, TT_DIV, TT_FLOAT, TT_INT, TT_MUL};

use self::node::{BinOpNode, NumberNode};

mod node;

pub struct Parser {
    _tokens: Vec<Token<token_value_types>>,
    _current_token: Token<token_value_types>,
}

impl Parser {
    pub fn new(tokens: Vec<Token<token_value_types>>) -> Self {
        let mut __tokens = tokens;
        let __current_token = __tokens.remove(0usize);

        Parser {
            _tokens: __tokens,
            _current_token: __current_token,
        }
    }

    pub fn advance(&mut self) -> bool {
        if self._tokens.len() > 0 {
            self._current_token = self._tokens.remove(0usize);
            true
        } else {
            false
        }
    }

    pub fn term(&mut self) -> Option<BinOpNode<NumberNode>> {
        let left_node: NumberNode = self.factor().unwrap();

        let mut op_token;
        let mut right_node;

        loop {
            op_token = self._current_token.clone();
            right_node = self.factor();

            if !(self._current_token.get_type().as_str() == TT_MUL
                || self._current_token.get_type().as_str() == TT_DIV)
            {
                break;
            }
        }

        Some(BinOpNode::new(left_node, op_token, right_node.unwrap()))
    }

    pub fn factor(&mut self) -> Option<NumberNode> {
        let token: Token<token_value_types> = self._current_token.clone();

        match token.get_type().as_str() {
            TT_INT => Some(NumberNode::new(token)),
            TT_FLOAT => Some(NumberNode::new(token)),
            _ => None,
        }
    }

    pub fn expression() {}
}
