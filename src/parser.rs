use crate::{
    ast::*,
    errors::Errors,
    lexer::{Lexer, Reader},
    token::*,
};

struct Parser {
    input: Vec<Token>,
    pt: usize,
    reader: Reader,
}

impl Parser {
    fn new(input: Vec<Token>, reader: Reader) -> Self {
        Parser {
            input,
            pt: 0,
            reader,
        }
    }

    fn next(&mut self) -> Token {
        let token = self.input[self.pt];
        if self.pt < self.input.len() - 1 {
            self.pt += 1
        };
        token
    }

    fn peek(&self) -> Token {
        self.input[self.pt]
    }

    fn parse(&mut self) -> Result<Vec<Stmt>, Errors> {
        let prog = Vec::new();
        while TokenType::Eof == self.peek().token_type {
            prog.push(self.parse_stmt()?);
        }
        Ok(prog)
    }
    fn parse_stmt(&mut self) -> Result<Stmt, Errors> {
        Ok(Stmt::Expr(Box::from(self.parse_expr()?)))
    }

    fn parse_expr(&mut self) -> Result<Expr, Errors> {
        self.parse_oof()
    }

    fn parse_oof(&mut self) -> Result<Expr, Errors> {
        if TokenType::Oof == self.peek().token_type {
            Ok(Expr::Unary(Operator::Oof, Box::new(self.parse_oof()?)))
        } else {
            self.parse_asgn()
        }
    }

    fn parse_asgn(&mut self) -> Result<Expr, Errors> {
        let left = self.parse_not()?;
        if TokenType::Be == self.peek().token_type {
            if let Expr::Id(_) = left {
                self.next();
                Ok(Expr::Binary(
                    Operator::Be,
                    Box::from(left),
                    Box::from(self.parse_asgn()?),
                ))
            } else {
                Err(self.parse_error("EXPECTED IDENTIFIER".to_string()))
            }
        } else {
            Ok(left)
        }
    }

    fn parse_not(&mut self) -> Result<Expr, Errors> {
        if self.peek().token_type == TokenType::Not {
            self.next();
            Ok(Expr::Unary(Operator::Not, Box::new(self.parse_not()?)))
        } else {
            self.parse_eq()
        }
    }

    fn parse_eq(&mut self) -> Result<Expr, Errors> {}
    fn parse_cmp(&mut self) -> Result<Expr, Errors> {}
    fn parse_arith(&mut self) -> Result<Expr, Errors> {}
    fn parse_term(&mut self) -> Result<Expr, Errors> {}
    fn parse_exp(&mut self) -> Result<Expr, Errors> {}
    fn parse_conv(&mut self) -> Result<Expr, Errors> {}
    fn parse_unary(&mut self) -> Result<Expr, Errors> {}
    fn parse_dot(&mut self) -> Result<Expr, Errors> {}
    fn parse_atom(&mut self) -> Result<Expr, Errors> {}
    fn parse_noob(&mut self) -> Result<Expr, Errors> {}
    fn parse_literal(&mut self) -> Result<Expr, Errors> {}
    fn parse_id(&mut self) -> Result<Expr, Errors> {}
    fn parse_paren(&mut self) -> Result<Expr, Errors> {}

    fn parse_error(&self, msg: String) -> Errors {
        let (line, col) = self.peek().location;
        Errors::SyntaxError(
            msg,
            (line, col),
            Lexer::to_string_lossy(self.reader.get_line(line)),
        )
    }
}
