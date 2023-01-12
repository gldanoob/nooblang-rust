use std::{
    collections::HashMap,
    io::{stdout, Write},
};

use crate::{
    ast::*,
    errors::Errors,
    lexer::{Lexer, Pos, Reader},
};

mod arith;
mod cmp;
mod control;
mod conv;
mod io;
mod var;

#[derive(Debug, PartialEq, Eq, Hash)]

pub struct Name {
    id: String,
    // scope
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i128),
    Float(f64),
    Text(String),
    Choice(bool),
    Nothing,
}

pub struct Eval<'a> {
    input: &'a Vec<Stmt>,
    reader: &'a Reader,
    context: HashMap<Name, Value>,
}

impl<'a> Eval<'a> {
    pub fn new(reader: &'a Reader, input: &'a Vec<Stmt>) -> Self {
        Self {
            input,
            reader,
            context: HashMap::new(),
        }
    }

    pub fn run_prog(&mut self) -> Result<Value, Errors> {
        let mut ret = Value::Nothing;
        for stmt in self.input {
            let v = self.run_stmt(stmt);
            // End statement
            if let Err(Errors::EndProg) = v {
                break;
            }
            ret = v?;
        }
        Ok(ret)
    }

    pub fn run_stmt(&mut self, stmt: &Stmt) -> Result<Value, Errors> {
        match stmt {
            Stmt::Expr(expr) => return self.eval_expr(expr),
            Stmt::Write(expr, line) => self.write(expr.as_ref(), *line)?,
            Stmt::RunFrom(from, to, line) => self.run_from(from.as_ref(), to.as_ref(), *line)?,
            Stmt::RunAt(expr, line) => self.run_at(expr.as_ref(), *line)?,
            Stmt::Switch(stmt, cond, line) => self.switch(stmt.as_ref(), cond.as_ref(), *line)?,
            Stmt::Asgn(id, expr, line) => self.asgn(id.as_ref(), expr.as_ref(), *line)?,
            Stmt::Blank => (),
            Stmt::End => return Err(Errors::EndProg),
        }
        Ok(Value::Nothing)
    }

    pub fn eval_expr(&mut self, expr: &Expr) -> Result<Value, Errors> {
        match expr {
            Expr::Binary(op, left, right, location) => {
                let left = self.eval_expr(left.as_ref())?;
                let right = self.eval_expr(right.as_ref())?;
                match op {
                    // arith.rs
                    Operator::Plus => self.plus(&left, &right, *location),
                    Operator::Minus => self.minus(&left, &right, *location),
                    Operator::Times => self.times(&left, &right, *location),
                    Operator::Over => self.over(&left, &right, *location),
                    Operator::Mod => self.modolo(&left, &right, *location),
                    Operator::ToThe => self.pow(&left, &right, *location),

                    // cmp.rs
                    Operator::Or => Ok(self.or(&left, &right)),
                    Operator::And => Ok(self.and(&left, &right)),
                    Operator::Is => Ok(self.is(&left, &right)),
                    Operator::Isnt => Ok(self.isnt(&left, &right)),
                    Operator::Below => self.below(&left, &right, *location),
                    Operator::Above => self.above(&left, &right, *location),
                    Operator::AtMost => self.atmost(&left, &right, *location),
                    Operator::AtLeast => self.atleast(&left, &right, *location),

                    _ => Ok(Value::Nothing),
                }
            }
            Expr::Unary(op, operand, location) => {
                let operand = self.eval_expr(operand.as_ref())?;
                match op {
                    // arith.rs
                    Operator::Neg => self.neg(&operand, *location),

                    // cmp.rs
                    Operator::Not => Ok(self.not(&operand)),

                    // conv.rs
                    Operator::Num => Ok(self.num(&operand, *location)),
                    Operator::Text => Ok(self.text(&operand)),
                    Operator::Choice => Ok(self.choice(&operand)),

                    _ => Ok(Value::Nothing),
                }
            }
            Expr::Nullary(op, location) => match op {
                Operator::Read => self.read(*location),
                _ => Ok(Value::Nothing),
            },
            Expr::Literal(literal, _) => Ok(match literal {
                Literal::Integer(n) => Value::Int(*n as i128),
                Literal::Decimal(n) => Value::Float(*n),
                Literal::Text(s) => Value::Text(s.to_owned()),
                Literal::Choice(b) => Value::Choice(*b),
            }),
            Expr::Id(id, location) => self.id(&Name { id: id.to_owned() }, *location),
        }
    }

    pub fn display(value: &Value) -> String {
        match value {
            Value::Int(n) => n.to_string(),
            Value::Float(n) => n.to_string(),
            Value::Text(s) => format!("\"{}\"", s),
            Value::Choice(b) => (if *b { "yes" } else { "no" }).to_string(),
            Value::Nothing => "nothing".to_string(),
        }
    }

    fn runtime_error(&self, msg: String, location: Pos) -> Errors {
        let Pos(line, col) = location;
        Errors::RuntimeError(
            msg,
            Pos(line, col),
            Lexer::to_string_lossy(self.reader.get_line(line)),
        )
    }
}
