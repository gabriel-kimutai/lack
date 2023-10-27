#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenKind {
    TExit,
    TIntlit,
    TSemicolon,
    TEof
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Token {
    pub token_kind: TokenKind,
    pub value: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Lexer {
    tokens: Vec<Token>,
    source: String,
    cursor: usize,
    line: usize,
}



impl Lexer {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            source: String::new(),
            cursor: 0,
            line: 1,
        }
    }

    pub fn from_str(source: String) -> Self {
        Self {
            tokens: Vec::new(),
            source,
            cursor: 0,
            line: 1,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut _tok: &char = &' ';
        let mut buf: String = String::new();
        // while self.peek().is_some() {
        //     _tok = &self.peek().unwrap();
        //     if self.peek().unwrap().is_alphanumeric() {
        //         if self.peek().unwrap().is_alphabetic() {
        //             while self.peek().unwrap().is_alphabetic() {
        //                 buf.push(self.consume());
        //                 continue;
        //             }
        //             if buf == "exit" {
        //                 self.tokens.push(Token {
        //                     token_kind: TokenKind::TExit,
        //                     value: None,
        //                 });
        //                 buf.clear();
        //                 continue;
        //             }
        //         }
        //         if self.peek().unwrap().is_numeric() {
        //             while self.peek().unwrap().is_numeric() {
        //                 buf.push(self.consume());
        //                 continue;
        //             }
        //             if buf.chars().all(char::is_numeric) {
        //                 self.tokens.push(Token {
        //                     token_kind: TokenKind::TIntlit,
        //                     value: Some(buf.to_string()),
        //                 });
        //                 buf.clear();
        //                 continue;
        //             }
        //         }
        //     }
        //     if self.peek().unwrap() == ';' {
        //         self.tokens.push(Token {
        //             token_kind: TokenKind::TSemicolon,
        //             value: None,
        //         });
        //         self.consume();
        //         continue;
        //     }
        //     if self.peek().unwrap() == ' ' {
        //         self.consume();
        //         continue;
        //     }
        //     if self.peek().unwrap() == '\n' {
        //         self.consume();
        //         continue;
        //     } else {
        //         println!("Error at line: {}, {}", self.line, self.cursor);
        //         re
        //     }
        // }
        return self.tokens.to_vec();
    }

    fn peek(&mut self) -> Option<char> {
        if self.cursor + 1 > self.source.len() {
            return None;
        } else {
            return self.source.chars().nth(self.cursor);
        }
    }

    fn consume(&mut self) -> char {
        self.cursor += 1;
        return self.source.chars().nth(self.cursor - 1).unwrap();
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self::new()
    }
}
