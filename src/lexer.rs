use std::{
    cmp,
    fs::File,
    io::{BufReader, Read},
};

use crate::{
    errors::Errors,
    token::{Token, TokenType},
};

// Char-by-char reader
pub struct Reader {
    input: Vec<u8>,
    pt: usize,
    col: usize,
    line: usize,
}

impl Reader {
    pub fn new(mut reader: BufReader<File>) -> Result<Self, Errors> {
        let mut input = Vec::new();
        reader
            .read_to_end(&mut input)
            .map_err(|_| Errors::IOError)?;
        Ok(Self {
            input,
            pt: 0,
            col: 1,
            line: 1,
        })
    }

    pub fn pos(&self) -> (usize, usize) {
        (self.line, self.col)
    }

    pub fn read(&mut self, n: usize) -> &[u8] {
        let prev = self.pt;
        self.skip(n);
        self.col += n;
        &self.input[prev..self.pt]
    }

    pub fn readc(&mut self) -> u8 {
        let c = self.peek();
        self.pt += 1;
        self.col += 1;
        c
    }

    pub fn peek(&mut self) -> u8 {
        if self.pt >= self.input.len() {
            return 26;
        }
        self.input[self.pt]
    }

    pub fn skip(&mut self, n: usize) {
        let prev = self.pt;
        self.pt = cmp::min(self.input.len(), self.pt + n);
        self.col += self.pt - prev;
    }

    pub fn back(&mut self, n: usize) {
        let prev = self.pt;
        self.pt = cmp::max(0, self.pt - n);
        self.col -= prev - self.pt;
    }

    pub fn next_line(&mut self) {
        self.line += 1;
        self.col = 1;
    }
}

pub struct Lexer {
    reader: Reader,
}

impl Lexer {
    pub fn new(reader: Reader) -> Self {
        Self { reader }
    }
    pub fn lex(&mut self) -> Result<Vec<Token>, Errors> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.scan_token()?;
            if let TokenType::Eof = tok.token_type {
                // Freaking borrow checker
                tokens.push(tok);
                break;
            }
            tokens.push(tok);
        }
        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Token, Errors> {
        self.skip_spaces();
        let c = self.reader.peek();
        Ok(match c {
            b'"' => self.scan_string()?,
            b'\n' | b'\r' => self.scan_newline()?,
            26 => self.scan_eof(),
            c if c.is_ascii_digit() => self.scan_number()?,
            c if c.is_ascii_alphabetic() => self.scan_word(),
            c => {
                return Err(Errors::SyntaxError(
                    format!("WEIRD CHARACTER: {}", c as char),
                    self.reader.pos(),
                ))
            }
        })
    }

    fn scan_word(&mut self) -> Token {
        let mut s = Vec::new();
        let pos = self.reader.pos();
        while self.reader.peek().is_ascii_alphanumeric() {
            s.push(self.reader.readc());
        }

        let s = Self::to_string_lossy(&s);
        // Is keyword
        if let Some(token) = Token::from_str(&s, pos) {
            // Read until newline if comment
            if let TokenType::Com = token.token_type {
                while !Lexer::is_eol(self.reader.peek()) {
                    self.reader.readc();
                }
            }
            return token;
        }

        // Is identifier
        return Token::new(TokenType::Id(s), pos);
    }

    fn scan_string(&mut self) -> Result<Token, Errors> {
        let pos = self.reader.pos();
        self.reader.skip(1);
        let mut s = Vec::new();
        while self.reader.peek() != b'"' {
            if Lexer::is_eol(self.reader.peek()) {
                return Err(Errors::SyntaxError(
                    "Unterminated string".to_string(),
                    self.reader.pos(),
                ));
            }
            s.push(self.reader.readc());
        }

        let s = Self::to_string_lossy(&s);

        self.reader.skip(1);
        Ok(Token::new(TokenType::String(s), pos))
    }

    fn scan_number(&mut self) -> Result<Token, Errors> {
        let pos = self.reader.pos();
        let mut s = Vec::new();
        while self.reader.peek().is_ascii_digit() {
            s.push(self.reader.readc());
        }

        let s = Self::to_string_lossy(&s);
        let num: u128 = s
            .parse()
            .map_err(|_| Errors::SyntaxError("NUMBER TOO LARGE".to_string(), self.reader.pos()))?;

        Ok(Token::new(TokenType::Number(num), pos))
    }

    fn scan_newline(&mut self) -> Result<Token, Errors> {
        let pos = self.reader.pos();
        if self.reader.peek() == b'\r' {
            self.reader.readc();
        }

        if self.reader.readc() != b'\n' {
            return Err(Errors::SyntaxError(
                "Carriage without return? Goofy".to_string(),
                self.reader.pos(),
            ));
        }

        self.reader.next_line();
        Ok(Token::new(TokenType::Lf, pos))
    }

    fn scan_eof(&mut self) -> Token {
        let token = Token::new(TokenType::Eof, self.reader.pos());
        self.reader.skip(1);
        token
    }

    fn skip_spaces(&mut self) {
        while let b' ' | b'\t' = self.reader.peek() {
            self.reader.skip(1);
        }
    }

    fn is_eol(c: u8) -> bool {
        match c {
            b'\r' | b'\n' | 26 => true,
            _ => false,
        }
    }

    // &[u8] to String
    pub fn to_string_lossy(s: &[u8]) -> String {
        String::from_utf8_lossy(s).into_owned()
    }
}
