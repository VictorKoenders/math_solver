use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, PartialEq)]
pub enum Unit {
    Value(f32),
    /// A complex number, in the form (a + b ^ i)
    ComplexNumber {
        a: Box<Unit>,
        b: Box<Unit>,
        i: Box<Unit>,
    },
}

impl Add for Unit {
    type Output = Unit;

    fn add(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Unit::Value(v1), Unit::Value(v2)) => Unit::Value(v1 + v2),
            (_, _) => unimplemented!(),
        }
    }
}

impl Sub for Unit {
    type Output = Unit;

    fn sub(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Unit::Value(v1), Unit::Value(v2)) => Unit::Value(v1 - v2),
            (_, _) => unimplemented!(),
        }
    }
}
impl Div for Unit {
    type Output = Unit;

    fn div(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Unit::Value(v1), Unit::Value(v2)) => Unit::Value(v1 / v2),
            (_, _) => unimplemented!(),
        }
    }
}

impl Mul for Unit {
    type Output = Unit;

    fn mul(self, rhs: Self) -> Self {
        match (self, rhs) {
            (Unit::Value(v1), Unit::Value(v2)) => Unit::Value(v1 * v2),
            (_, _) => unimplemented!(),
        }
    }
}
