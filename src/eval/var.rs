use super::*;
impl<'a> Eval<'a> {
    pub fn asgn(&mut self, left: &Expr, right: &Value) -> Value {
        let Expr::Id(id, location) = left else {
            panic!("LEFT SIDE NOT NAME");
        };
        let name = Name { id: id.clone() };
        self.context.insert(name, right.clone());
        right.clone()
    }

    pub fn id(&mut self, name: &Name, location: Pos) -> Result<Value, Errors> {
        match self.context.get(name) {
            Some(value) => Ok(value.clone()),
            None => Err(self.runtime_error(format!("NAME {} NOT FOUND", name.id), location)),
        }
    }
}
