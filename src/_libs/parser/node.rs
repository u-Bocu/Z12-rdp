use crate::tokenizer::token::{token_value_types, Token};
use std::fmt;

pub trait Node {}

pub struct NumberNode {
    _token: Token<token_value_types>,
}

pub struct BinOpNode<T: fmt::Debug>
where
    T: Node,
{
    _left_node: T,
    _right_node: T,
    _op_token: Token<token_value_types>,
}

impl NumberNode {
    pub fn new(__token: Token<token_value_types>) -> Self {
        NumberNode { _token: __token }
    }
}

impl fmt::Debug for NumberNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self._token)
    }
}

impl Node for NumberNode {}
impl<T: fmt::Debug> Node for BinOpNode<T> where T: Node {}

impl<T: fmt::Debug> BinOpNode<T>
where
    T: Node,
{
    pub fn new(left_node: T, __token: Token<token_value_types>, right_node: T) -> Self {
        BinOpNode {
            _left_node: left_node,
            _right_node: right_node,
            _op_token: __token,
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for BinOpNode<T>
where
    T: Node,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({:?}{:?}{:?})",
            self._left_node, self._op_token, self._right_node
        )
    }
}
