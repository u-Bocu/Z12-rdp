// An attribute to hide warnings for unused code.
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![warn(non_snake_case)]

use lazy_static::lazy_static;
use regex::Regex;

mod token;

use token::{
    token_value_types::{self, Float, Int},
    Token,
};

pub struct Tokenizer {
    _text: String,
    _current_char: char,
}

impl Tokenizer {
    pub fn new(text: String) -> Self {
        let mut __text = text;
        let __current_char = __text.remove(0usize);

        Tokenizer {
            _text: __text,
            _current_char: __current_char,
        }
    }

    pub fn advance(&mut self) -> bool {
        if self._text.len() > 0 {
            self._current_char = self._text.remove(0usize);
            true
        } else {
            false
        }
    }

    pub fn create_tokens(&mut self) -> Vec<Token<token_value_types>> {
        let mut tokens: Vec<Token<token_value_types>> = Vec::new();

        loop {
            if is_a_number(self._current_char.to_string()) {
                tokens.push(self.make_number())
            }

            match self._current_char {
                '+' => tokens.push(Token::new(token::TT_PLUS.to_string(), None)),
                '-' => tokens.push(Token::new(token::TT_MINUS.to_string(), None)),
                '*' => tokens.push(Token::new(token::TT_MUL.to_string(), None)),
                '/' => tokens.push(Token::new(token::TT_DIV.to_string(), None)),
                '(' => tokens.push(Token::new(token::TT_LPARENT.to_string(), None)),
                ')' => tokens.push(Token::new(token::TT_RPARENT.to_string(), None)),

                _ => {}
            }

            if !self.advance() {
                break;
            }
        }

        tokens
    }

    pub fn make_number(&mut self) -> Token<token_value_types> {
        let mut number_str: String = String::new();
        let mut dot_found: bool = false;

        loop {
            number_str.push(self._current_char);

            match self._current_char {
                '.' => {
                    if dot_found {
                        break;
                    } else {
                        dot_found = true;
                    }
                }

                _ => {}
            }

            if !self.advance()
                || (!is_a_number(self._current_char.to_string()) && !(self._current_char == '.'))
            {
                break;
            }
        }

        if dot_found {
            Token::new(
                token::TT_FLOAT.to_string(),
                Some(Float(number_str.parse::<f32>().unwrap())),
            )
        } else {
            Token::new(
                token::TT_INT.to_string(),
                Some(Int(number_str.parse::<i32>().unwrap())),
            )
        }
    }
}

fn is_a_number(s: String) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d").unwrap();
    }

    RE.is_match(&s)
}
