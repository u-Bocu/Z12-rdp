// An attribute to hide warnings for unused code.
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![warn(non_snake_case)]

use core::fmt;
use std::fmt::Debug;

// Allowed token types (for now):
pub const TT_INT: &str = "INT";
pub const TT_FLOAT: &str = "FLOAT";
pub const TT_PLUS: &str = "PLUS";
pub const TT_MINUS: &str = "MINUS";
pub const TT_MUL: &str = "MUL";
pub const TT_DIV: &str = "DIV";
pub const TT_LPARENT: &str = "LPARENT";
pub const TT_RPARENT: &str = "RPARENT";

/**
 * Empty struct used when a Token has no value.
 */
#[derive(Debug, Clone, Copy)]
pub struct Nothing {}

#[derive(Debug, Clone, Copy)]
pub enum token_value_types {
    Int(i32),
    Float(f32),
    NotAThing(Nothing),
}

/**
 * Token structure used by the tokenizer, consisting of a type name and a generic typed value.
 */

#[derive(Clone)]
pub struct Token<T: Debug> {
    _type: String,
    _value: Option<T>,
}

impl<T: Debug> Token<T> {
    pub fn new(__type: String, __value: Option<T>) -> Self {
        Token {
            _type: __type,
            _value: __value,
        }
    }

    #[inline]
    pub fn get_type(&self) -> String {
        self._type.clone()
    }
}

impl<T: Debug> fmt::Debug for Token<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str: String = match &self._value {
            Some(value) => self._type.clone() + ":" + &format!("{:?}", value).to_owned(),
            None => self._type.clone(),
        };
        write!(f, "{}", str)
    }
}
