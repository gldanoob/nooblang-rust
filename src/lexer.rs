use std::{
    cmp,
    fs::File,
    io::{BufReader, Read},
};

use crate::{
    errors::Errors,
    token::{Token, TokenType},
};

// location in code
#[derive(Debug, Clone, Copy)]
pub struct Pos(pub usize, pub usize);

// bytes reader
pub struct Reader {
    input: Vec<u8>,
    // I dumb
    newlines: Vec<usize>,
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
            newlines: Vec::new(),
            pt: 0,
            col: 1,
            line: 1,
        })
    }

    pub fn pos(&self) -> Pos {
        Pos(self.line, self.col)
    }

    pub fn _read(&mut self, n: usize) -> &[u8] {
        let prev = self.pt;
        self.skip(n);
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

    pub fn _back(&mut self, n: usize) {
        let prev = self.pt;
        self.pt = cmp::max(0, self.pt - n);
        self.col -= prev - self.pt;
    }

    // Consume newline character
    pub fn newline(&mut self) {
        self.newlines.push(self.pt);
        self.line += 1;
        self.col = 1;
        self.pt += 1;
    }

    // Used in lexer
    pub fn get_this_line(&mut self) -> &[u8] {
        let start = if self.line > 1 {
            self.newlines[self.line - 2] + 1
        } else {
            0
        };

        let end = if self.line <= self.newlines.len() {
            self.newlines[self.line - 1]
        } else {
            // Read to end of newline
            while !Lexer::is_eol(self.peek()) {
                self.readc();
            }
            self.pt
        };

        &self.input[start..end]
    }

    // Used in parser, eval etc
    pub fn get_line(&self, line: usize) -> &[u8] {
        println!("{:?} {:?}", self.input, self.newlines);
        let start = if line > 1 {
            self.newlines[line - 2] + 1
        } else {
            0
        };

        let end = if line <= self.newlines.len() {
            self.newlines[line - 1]
        } else {
            // Must be the last line
            self.input.len()
        };

        &self.input[start..end]
    }
}

pub struct Lexer<'a> {
    reader: &'a mut Reader,
}

impl<'a> Lexer<'a> {
    pub fn new(reader: &'a mut Reader) -> Self {
        Self { reader }
    }
    pub fn lex(&mut self) -> Result<Vec<Token>, Errors> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.scan_token()?;
            if TokenType::Eof == tok.token_type {
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
                return Err(
                    self.syntax_error(format!("WEIRD SYMBOL: {}", c as char), self.reader.pos())
                );
            }
        })
    }

    fn scan_word(&mut self) -> Token {
        let mut s = Vec::new();
        let pos = self.reader.pos();
        while self.reader.peek().is_ascii_alphabetic() {
            s.push(self.reader.readc());
        }

        let s = Self::to_string_lossy(&s);
        // Is keyword
        if let Some(token) = Token::from_str(&s, pos) {
            // Read until newline if comment
            if TokenType::Note == token.token_type {
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
            if Self::is_eol(self.reader.peek()) {
                return Err(self.syntax_error("MISSING QUOTE".to_string(), self.reader.pos()));
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
        let num = s.parse();

        // range of i128 is [-2^127, 2^127-1]
        if let Ok(num) = num {
            if num < 1 << 127 {
                return Ok(Token::new(TokenType::Number(num), pos));
            }
        }

        Err(self.syntax_error("NUMBER TOO LARGE".to_string(), pos))?
    }

    fn scan_newline(&mut self) -> Result<Token, Errors> {
        let pos = self.reader.pos();
        if self.reader.peek() == b'\r' {
            self.reader.readc();
        }

        if self.reader.peek() != b'\n' {
            return Err(self.syntax_error("Carriage without return? Goofy".to_string(), pos));
        }

        // Skip \n
        self.reader.newline();

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

    fn syntax_error(&mut self, msg: String, pos: Pos) -> Errors {
        Errors::SyntaxError(msg, pos, Self::to_string_lossy(self.reader.get_this_line()))
    }

    // &[u8] to String
    pub fn to_string_lossy(s: &[u8]) -> String {
        String::from_utf8_lossy(s).into_owned()
    }
}
