use crate::lexer::{Token, TokenKind};



#[derive(Debug)]
pub struct Parser {
    cursor: usize,
    tokens: Vec<Token>
}

pub struct NodeExpr {
    pub int_lit: String
}
pub struct NodeExit {
    pub node_expr: NodeExpr,
}

impl Parser {
    pub fn from_tok(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            cursor: 0
        }
    }

    pub fn parse_expr(&mut self) -> Option<NodeExpr> {
        if self.peek().is_some() && self.peek().unwrap().token_kind == TokenKind::TIntlit {
            return Some(NodeExpr{int_lit: self.consume().value.unwrap()});
        } else {
            return None;
        }
    }

    pub fn parse(&mut self) -> Option<NodeExit> {
        let mut node_exit: Option<NodeExit> = Default::default();
        while self.peek().is_some() {
            if self.peek().unwrap().token_kind == TokenKind::TExit {
                self.consume();
                match self.parse_expr() {
                    Some(expr) => node_exit = Some(NodeExit { node_expr: expr }),
                    None => panic!("Invalid expression")
                }
            }
            if self.peek().unwrap().token_kind == TokenKind::TSemicolon {
                self.consume();
            }
        }
        node_exit
    }

    fn peek(&mut self) -> Option<Token> {
        if self.cursor + 1 > self.tokens.len() {
            return None;
        } else {
            return self.tokens.iter().nth(self.cursor).cloned();
        }
    }

    fn consume(&mut self) -> Token {
        self.cursor += 1;
        return self.tokens.iter().nth(self.cursor - 1).unwrap().clone();
    }
}