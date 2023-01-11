use super::*;

impl<'a> Eval<'a> {
    pub fn is(&self, a: &Value, b: &Value) -> Value {
        Value::Choice(match (a, b) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Int(a), Value::Float(b)) => *a as f64 == *b,
            (Value::Float(a), Value::Int(b)) => *a == *b as f64,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Text(a), Value::Text(b)) => a == b,
            (Value::Choice(a), Value::Choice(b)) => a == b,
            _ => false,
        })
    }

    pub fn not(&self, a: &Value) -> Value {
        Value::Choice(match self.choice(a) {
            Value::Choice(a) => !a,
            _ => panic!("NOT BOOLEAN AFTER CONVERTING WITH BOO"),
        })
    }

    pub fn and(&self, a: &Value, b: &Value) -> Value {
        Value::Choice(match (self.choice(a), self.choice(b)) {
            (Value::Choice(a), Value::Choice(b)) => a && b,
            _ => panic!("NOT BOOLEAN AFTER CONVERTING WITH BOO"),
        })
    }

    pub fn or(&self, a: &Value, b: &Value) -> Value {
        Value::Choice(match (self.choice(a), self.choice(b)) {
            (Value::Choice(a), Value::Choice(b)) => a || b,
            _ => panic!("NOT BOOLEAN AFTER CONVERTING WITH BOO"),
        })
    }

    pub fn isnt(&self, a: &Value, b: &Value) -> Value {
        self.not(&self.is(a, b))
    }

    pub fn below(&self, a: &Value, b: &Value, location: Pos) -> Result<Value, Errors> {
        Ok(Value::Choice(match (a, b) {
            (Value::Int(a), Value::Int(b)) => a < b,
            (Value::Int(a), Value::Float(b)) => (*a as f64) < *b,
            (Value::Float(a), Value::Int(b)) => *a < (*b as f64),
            (Value::Float(a), Value::Float(b)) => a < b,
            (Value::Text(a), Value::Text(b)) => a < b,
            _ => return Err(self.runtime_error("CANNOT COMPARE VALUES".to_string(), location)),
        }))
    }

    pub fn above(&self, a: &Value, b: &Value, location: Pos) -> Result<Value, Errors> {
        Ok(Value::Choice(match (a, b) {
            (Value::Int(a), Value::Int(b)) => a > b,
            (Value::Int(a), Value::Float(b)) => (*a as f64) > *b,
            (Value::Float(a), Value::Int(b)) => *a > (*b as f64),
            (Value::Float(a), Value::Float(b)) => a > b,
            (Value::Text(a), Value::Text(b)) => a > b,
            _ => return Err(self.runtime_error("CAN'T COMPARE VALUES".to_string(), location)),
        }))
    }

    pub fn atmost(&self, a: &Value, b: &Value, location: Pos) -> Result<Value, Errors> {
        Ok(Value::Choice(match (a, b) {
            (Value::Int(a), Value::Int(b)) => a <= b,
            (Value::Int(a), Value::Float(b)) => (*a as f64) <= *b,
            (Value::Float(a), Value::Int(b)) => *a <= (*b as f64),
            (Value::Float(a), Value::Float(b)) => a <= b,
            (Value::Text(a), Value::Text(b)) => a <= b,
            _ => return Err(self.runtime_error("CAN'T COMPARE VALUES".to_string(), location)),
        }))
    }

    pub fn atleast(&self, a: &Value, b: &Value, location: Pos) -> Result<Value, Errors> {
        Ok(Value::Choice(match (a, b) {
            (Value::Int(a), Value::Int(b)) => a >= b,
            (Value::Int(a), Value::Float(b)) => (*a as f64) >= *b,
            (Value::Float(a), Value::Int(b)) => *a >= (*b as f64),
            (Value::Float(a), Value::Float(b)) => a >= b,
            (Value::Text(a), Value::Text(b)) => a >= b,
            _ => return Err(self.runtime_error("CAN'T COMPARE VALUES".to_string(), location)),
        }))
    }
}
