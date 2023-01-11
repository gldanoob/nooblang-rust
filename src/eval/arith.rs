use super::*;
use Value::*;

impl<'a> Eval<'a> {
    pub fn plus(&self, a: &Value, b: &Value, location: Pos) -> Result<Value, Errors> {
        Ok(match (a, b) {
            (Int(a), Int(b)) => match a.checked_add(*b) {
                Some(v) => Int(v),
                None => Float(*a as f64 + *b as f64),
            },
            (Int(a), Float(b)) => Float(*a as f64 + b),
            (Float(a), Int(b)) => Float(*a + *b as f64),
            (Float(a), Float(b)) => Float(a + b),
            (Text(a), Text(b)) => Text(a.to_owned() + &b),
            _ => return Err(self.runtime_error("CAN'T ADD DIS".to_string(), location)),
        })
    }

    pub fn minus(&self, a: &Value, b: &Value, location: Pos) -> Result<Value, Errors> {
        Ok(match (a, b) {
            (Int(a), Int(b)) => match a.checked_sub(*b) {
                Some(v) => Int(v),
                None => Float(*a as f64 - *b as f64),
            },
            (Int(a), Float(b)) => Float(*a as f64 - b),
            (Float(a), Int(b)) => Float(a - *b as f64),
            (Float(a), Float(b)) => Float(a - b),
            _ => return Err(self.runtime_error("CAN'T SUBTRACT DIS".to_string(), location)),
        })
    }

    pub fn times(&self, a: &Value, b: &Value, location: Pos) -> Result<Value, Errors> {
        Ok(match (a, b) {
            (Int(a), Int(b)) => match a.checked_mul(*b) {
                Some(v) => Int(v),
                None => Float(*a as f64 * *b as f64),
            },
            (Int(a), Float(b)) => Float(*a as f64 * b),
            (Float(a), Int(b)) => Float(a * *b as f64),
            (Float(a), Float(b)) => Float(a * b),
            _ => return Err(self.runtime_error("CAN'T MULTIPLY DIS".to_string(), location)),
        })
    }

    pub fn over(&self, a: &Value, b: &Value, location: Pos) -> Result<Value, Errors> {
        Ok(match (a, b) {
            (Int(a), Int(b)) => match a.checked_mul(*b) {
                Some(v) => Int(v),
                None => Float(*a as f64 / *b as f64),
            },
            (Int(a), Float(b)) => Float(*a as f64 / b),
            (Float(a), Int(b)) => Float(a / *b as f64),
            (Float(a), Float(b)) => Float(a / b),
            _ => return Err(self.runtime_error("CAN'T DIVIDE DIS".to_string(), location)),
        })
    }

    pub fn pow(&self, a: &Value, b: &Value, location: Pos) -> Result<Value, Errors> {
        Ok(match (a, b) {
            (Int(a), Int(b)) => {
                if let Ok(b) = u32::try_from(*b) {
                    match a.checked_pow(b) {
                        Some(v) => return Ok(Int(v)),
                        _ => (),
                    }
                }
                Float((*a as f64).powf(*b as f64))
            }
            (Int(a), Float(b)) => Float((*a as f64).powf(*b)),
            (Float(a), Int(b)) => Float(a.powf(*b as f64)),
            (Float(a), Float(b)) => Float(a.powf(*b)),
            _ => {
                return Err(self.runtime_error("CAN'T RAISE TO POWER OF DIS".to_string(), location))
            }
        })
    }

    pub fn modolo(&self, a: &Value, b: &Value, location: Pos) -> Result<Value, Errors> {
        Ok(match (a, b) {
            (Int(a), Int(b)) => Int(a % b),
            (Int(a), Float(b)) => Float(*a as f64 % b),
            (Float(a), Int(b)) => Float(a % *b as f64),
            (Float(a), Float(b)) => Float(a % b),
            _ => return Err(self.runtime_error("CAN'T MODOLO DIS".to_string(), location)),
        })
    }

    pub fn neg(&self, a: &Value, location: Pos) -> Result<Value, Errors> {
        Ok(match a {
            Int(v) => Int(-v),
            Float(v) => Float(-v),
            _ => return Err(self.runtime_error("CAN'T NEGATE DIS".to_string(), location)),
        })
    }
}
