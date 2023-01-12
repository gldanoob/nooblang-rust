use super::*;
impl<'a> Eval<'a> {
    pub fn asgn(&mut self, left: &Expr, right: &Expr, _line: usize) -> Result<(), Errors>  {
        let right = self.eval_expr(right)?;
        let Expr::Id(id, _location) = left else {
            panic!("LEFT SIDE NOT ID");
        };
        let name = Name { id: id.clone() };
        self.context.insert(name, right.clone());
        Ok(())
    }

    pub fn id(&mut self, name: &Name, location: Pos) -> Result<Value, Errors> {
        match self.context.get(name) {
            Some(value) => Ok(value.clone()),
            None => Err(self.runtime_error(format!("VARIABLE {} NOT FOUND", name.id), location)),
        }
    }
}
