use super::*;
impl<'a> Eval<'a> {
    pub fn write(&self, operand: &Value, location: Pos) -> Result<Value, Errors> {
        let mut formatted = if let Value::Text(s) = self.text(operand) {
            s
        } else {
            // Shouldn't execute
            String::new()
        };
        formatted.push('\n');
        stdout()
            .write(formatted.as_bytes())
            .map_err(|_| Errors::IOError)?;
        Ok(Value::Nothing)
    }

    pub fn read(&self, location: Pos) -> Result<Value, Errors> {
        let mut buf = String::new();
        std::io::stdin()
            .read_line(&mut buf)
            .map_err(|_| Errors::IOError)?;
        Ok(Value::Text(buf.trim_end().to_owned()))
    }
}
