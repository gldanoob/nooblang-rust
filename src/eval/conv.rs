use super::*;
impl<'a> Eval<'a> {
    pub fn choice(&self, a: &Value) -> Value {
        Value::Choice(match a {
            Value::Choice(a) => *a,
            Value::Int(a) => *a != 0,
            Value::Float(a) => *a != 0.0,
            Value::Text(a) => a.len() != 0,
            Value::Nothing => false,
        })
    }

    pub fn num(&self, a: &Value, _location: Pos) -> Value {
        match a {
            Value::Choice(a) => Value::Int(if *a { 0 } else { 1 }),
            Value::Int(a) => Value::Int(*a),
            Value::Float(a) => Value::Float(*a),
            Value::Text(a) => {
                if let Ok(v) = a.parse::<i128>() {
                    Value::Int(v)
                } else if let Ok(v) = a.parse::<f64>() {
                    Value::Float(v)
                } else {
                    // Can't convert, silently return nil :)
                    Value::Nothing
                }
            }
            Value::Nothing => Value::Int(0),
        }
    }

    pub fn text(&self, a: &Value) -> Value {
        Value::Text(match a {
            Value::Int(n) => n.to_string(),
            Value::Float(n) => n.to_string(),
            Value::Text(s) => s.to_owned(),
            Value::Choice(b) => (if *b { "yes" } else { "no" }).to_string(),
            Value::Nothing => "nothing".to_string(),
        })
    }
}
