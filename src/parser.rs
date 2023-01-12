use crate::{
    ast::*,
    errors::Errors,
    lexer::{Lexer, Pos, Reader},
    token::*,
};

pub struct Parser<'a> {
    input: Vec<Token>,
    pt: usize,
    reader: &'a Reader,
}

impl<'a> Parser<'a> {
    pub fn new(input: Vec<Token>, reader: &'a Reader) -> Self {
        Parser {
            input,
            pt: 0,
            reader,
        }
    }

    fn read(&mut self) -> &Token {
        let tok = &self.input[self.pt];
        self.pt += 1;
        return tok;
    }

    fn peek(&self) -> &Token {
        &self.input[self.pt]
    }

    fn back(&mut self) {
        self.pt -= 1;
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Errors> {
        let mut prog = Vec::new();
        loop {
            // Each iteration is a line
            match self.peek().token_type {
                TokenType::Eof => break,
                TokenType::Lf => {
                    self.read();
                    prog.push(Stmt::Blank);
                }
                _ => {
                    prog.push(self.parse_switch()?);
                    let eol = self.peek();
                    match eol.token_type {
                        TokenType::Eof => break,
                        TokenType::Lf => {
                            self.read();
                        }
                        _ => {
                            return Err(self.parse_error("WHY THIS HERE".to_string(), eol.location))
                        }
                    }
                }
            };
        }
        Ok(prog)
    }

    fn parse_switch(&mut self) -> Result<Stmt, Errors> {
        let stmt = self.parse_stmt()?;
        let TokenType::If = self.peek().token_type else {
            return Ok(stmt);
        };

        // Found single if
        let line = self.read().location.0;
        Ok(Stmt::Switch(
            Box::new(stmt),
            Box::new(self.parse_expr()?),
            line,
        ))
    }

    fn parse_stmt(&mut self) -> Result<Stmt, Errors> {
        let tok = self.read();
        let Pos(line, _) = tok.location;
        Ok(match tok.token_type {
            TokenType::Run => self.parse_run()?,
            TokenType::Write => Stmt::Write(Box::from(self.parse_expr()?), line),
            TokenType::End => Stmt::End,
            _ => {
                self.back();
                self.parse_asgn()?
            }
        })
    }

    fn parse_run(&mut self) -> Result<Stmt, Errors> {
        let from = self.parse_expr()?;
        let tok = self.read();
        let location = tok.location;
        if let TokenType::To = tok.token_type {
            let to = self.parse_expr()?;
            Ok(Stmt::RunFrom(Box::from(from), Box::from(to), location.0))
        } else {
            let at = self.parse_expr()?;
            Ok(Stmt::RunAt(Box::from(at), location.0))
        }
    }

    fn parse_asgn(&mut self) -> Result<Stmt, Errors> {
        let location = self.peek().location;
        let left = self.parse_expr()?;
        if TokenType::Be == self.peek().token_type {
            self.read();
            match left {
                Expr::Id(..) => Ok(Stmt::Asgn(
                    Box::from(left),
                    Box::from(self.parse_expr()?),
                    location.0,
                )),
                _ => Err(self.parse_error("WHERE IDENTIFIER".to_string(), location)),
            }
        } else {
            Ok(Stmt::Expr(Box::from(left)))
        }
    }

    // fn parse_write(&mut self) -> Result<Expr, Errors> {
    //     if TokenType::Write == self.peek().token_type {
    //         let location = self.read().location;
    //         Ok(Expr::Unary(
    //             Operator::Write,
    //             Box::new(self.parse_write()?),
    //             location,
    //         ))
    //     } else {
    //         self.parse_or()
    //     }
    // }

    fn parse_expr(&mut self) -> Result<Expr, Errors> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, Errors> {
        let mut ast = self.parse_and()?;
        loop {
            let tok = self.read();
            let location = tok.location;
            ast = Expr::Binary(
                match tok.token_type {
                    TokenType::Or => Operator::Or,
                    _ => break,
                },
                Box::new(ast),
                Box::from(self.parse_and()?),
                location,
            );
        }
        self.back();
        Ok(ast)
    }

    fn parse_and(&mut self) -> Result<Expr, Errors> {
        let mut ast = self.parse_not()?;
        loop {
            let tok = self.read();
            let location = tok.location;
            ast = Expr::Binary(
                match tok.token_type {
                    TokenType::And => Operator::And,
                    _ => break,
                },
                Box::new(ast),
                Box::from(self.parse_not()?),
                location,
            );
        }
        self.back();
        Ok(ast)
    }

    fn parse_not(&mut self) -> Result<Expr, Errors> {
        if self.peek().token_type == TokenType::Not {
            let location = self.read().location;
            Ok(Expr::Unary(
                Operator::Not,
                Box::new(self.parse_is()?),
                location,
            ))
        } else {
            self.parse_is()
        }
    }

    fn parse_is(&mut self) -> Result<Expr, Errors> {
        let mut ast = self.parse_cmp()?;
        loop {
            let tok = self.read();
            let location = tok.location;
            ast = Expr::Binary(
                match tok.token_type {
                    TokenType::Is => Operator::Is,
                    TokenType::Isnt => Operator::Isnt,
                    _ => break,
                },
                Box::new(ast),
                Box::from(self.parse_cmp()?),
                location,
            );
        }
        self.back();
        Ok(ast)
    }

    fn parse_cmp(&mut self) -> Result<Expr, Errors> {
        let mut ast = self.parse_arith()?;
        loop {
            let tok = self.read();
            let location = tok.location;
            ast = Expr::Binary(
                match tok.token_type {
                    TokenType::Below => Operator::Below,
                    TokenType::Above => Operator::Above,
                    TokenType::AtMost => Operator::AtMost,
                    TokenType::AtLeast => Operator::AtLeast,
                    _ => break,
                },
                Box::new(ast),
                Box::from(self.parse_arith()?),
                location,
            );
        }
        self.back();
        Ok(ast)
    }

    fn parse_arith(&mut self) -> Result<Expr, Errors> {
        let mut ast = self.parse_term()?;
        loop {
            let tok = self.read();
            let location = tok.location;
            ast = Expr::Binary(
                match tok.token_type {
                    TokenType::Plus => Operator::Plus,
                    TokenType::Minus => Operator::Minus,
                    _ => break,
                },
                Box::new(ast),
                Box::from(self.parse_term()?),
                location,
            );
        }
        self.back();
        Ok(ast)
    }

    fn parse_term(&mut self) -> Result<Expr, Errors> {
        let mut ast = self.parse_exp()?;
        loop {
            let tok = self.read();
            let location = tok.location;
            ast = Expr::Binary(
                match tok.token_type {
                    TokenType::Times => Operator::Times,
                    TokenType::Over => Operator::Over,
                    TokenType::Mod => Operator::Mod,
                    _ => break,
                },
                Box::new(ast),
                Box::from(self.parse_exp()?),
                location,
            );
        }
        self.back();
        Ok(ast)
    }

    fn parse_exp(&mut self) -> Result<Expr, Errors> {
        let left = self.parse_conv()?;
        if TokenType::ToThe == self.peek().token_type {
            let location = self.read().location;
            Ok(Expr::Binary(
                Operator::ToThe,
                Box::from(left),
                Box::from(self.parse_conv()?),
                location,
            ))
        } else {
            Ok(left)
        }
    }

    fn parse_conv(&mut self) -> Result<Expr, Errors> {
        let tok = self.read();
        let location = tok.location;
        Ok(Expr::Unary(
            match tok.token_type {
                TokenType::Num => Operator::Num,
                TokenType::Text => Operator::Text,
                TokenType::Choice => Operator::Choice,
                _ => {
                    self.back();
                    return self.parse_neg();
                }
            },
            Box::from(self.parse_neg()?),
            location,
        ))
    }

    fn parse_neg(&mut self) -> Result<Expr, Errors> {
        if self.peek().token_type == TokenType::Neg {
            let location = self.read().location;
            Ok(Expr::Unary(
                Operator::Neg,
                Box::from(self.parse_decimal()?),
                location,
            ))
        } else {
            self.parse_decimal()
        }
    }

    fn parse_decimal(&mut self) -> Result<Expr, Errors> {
        let left = self.parse_atom()?;
        if self.peek().token_type == TokenType::Dot {
            let location = self.read().location;
            let right = self.parse_atom()?;
            match (left, right) {
                (Expr::Literal(Literal::Integer(a), _), Expr::Literal(Literal::Integer(b), _)) => {
                    let pow = if b == 0 {
                        1.0
                    } else {
                        (b as f64).log10().ceil()
                    };
                    Ok(Expr::Literal(
                        Literal::Decimal(a as f64 + b as f64 / 10_f64.powf(pow)),
                        location,
                    ))
                }
                _ => Err(self.parse_error("INVALID DECIMAL".to_string(), location)),
            }
        } else {
            Ok(left)
        }
    }

    fn parse_atom(&mut self) -> Result<Expr, Errors> {
        match self.peek().token_type {
            TokenType::Read => Ok(self.parse_read()),
            TokenType::Id(_) => self.parse_id(),
            TokenType::Number(_) | TokenType::String(_) | TokenType::Yes | TokenType::No => {
                self.parse_literal()
            }
            TokenType::Open => self.parse_paren(),
            _ => return Err(self.parse_error("NEED VALUE".to_string(), self.peek().location)),
        }
    }

    fn parse_read(&mut self) -> Expr {
        let location = self.read().location;
        return Expr::Nullary(Operator::Read, location);
    }

    fn parse_literal(&mut self) -> Result<Expr, Errors> {
        let tok = self.read();
        let location = tok.location;
        Ok(Expr::Literal(
            match tok.token_type {
                TokenType::Number(v) => Literal::Integer(v),
                TokenType::String(ref v) => Literal::Text(v.to_owned()),
                TokenType::Yes => Literal::Choice(true),
                TokenType::No => Literal::Choice(false),
                _ => return Err(self.parse_error("EXPECTED LITERAL".to_string(), location)),
            },
            tok.location,
        ))
    }

    fn parse_id(&mut self) -> Result<Expr, Errors> {
        let tok = self.read();
        let location = tok.location;
        if let TokenType::Id(ref name) = tok.token_type {
            Ok(Expr::Id(name.to_owned(), tok.location))
        } else {
            Err(self.parse_error("EXPECTED NAME".to_string(), location))
        }
    }

    fn parse_paren(&mut self) -> Result<Expr, Errors> {
        self.read();
        let ast = self.parse_expr()?;
        if self.peek().token_type != TokenType::Close {
            return Err(self.parse_error("NEED CLOSE".to_string(), self.peek().location));
        }
        self.read();
        return Ok(ast);
    }

    fn parse_error(&self, msg: String, location: Pos) -> Errors {
        let Pos(line, _) = location;
        Errors::SyntaxError(
            msg,
            location,
            Lexer::to_string_lossy(self.reader.get_line(line)),
        )
    }
}
