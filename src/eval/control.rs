use super::*;
impl<'a> Eval<'a> {
    pub fn switch(&mut self, stmt: &Stmt, cond: &Expr, _line: usize) -> Result<(), Errors> {
        let cond = self.eval_expr(cond)?;
        if let Value::Choice(true) = self.choice(&cond) {
            self.run_stmt(&stmt)?;
        }
        Ok(())
    }

    pub fn run_from(&mut self, from: &Expr, to: &Expr, line: usize) -> Result<(), Errors> {
        let from = self.eval_expr(from)?;
        let to = self.eval_expr(to)?;
        if let (Value::Int(from), Value::Int(to)) = (from, to) {
            if from <= to && from >= 1 && to <= self.input.len() as i128 {
                let from = from as usize;
                let to = to as usize;

                for stmt in &self.input[from - 1..=to - 1] {
                    self.run_stmt(stmt)?;
                }
            }
        }
        return Err(self.runtime_error("INVALID LINE RANGE".to_string(), Pos(line, 1)));
    }

    pub fn run_at(&mut self, at: &Expr, line: usize) -> Result<(), Errors> {
        let at = self.eval_expr(at)?;
        if let Value::Int(at) = at {
            if at >= 1 && at <= self.input.len() as i128 {
                let at = at as usize;
                return self.run_stmt(&self.input[at - 1]).and(Ok(()));
            }
        }
        return Err(self.runtime_error("INVALID LINE NUMBER".to_string(), Pos(line, 1)));
    }
}
