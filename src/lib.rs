mod evaluator;
mod parser;

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub from: usize,
    pub to: usize,
}

impl Span {
    pub fn merge(self, other: Span) -> Span {
        Span {
            from: self.from.min(other.from),
            to: self.to.max(other.to),
        }
    }
}

pub fn evaluate(statement: &str) -> Result<f32, MathError> {
    let expression = parser::Expression::parse(statement)?;
    evaluator::evaluate(expression).map_err(Into::into)
}

#[derive(Debug)]
pub enum MathError {
    ParseError(parser::ParseError),
    EvaluateError(evaluator::EvaluateError),
}

impl MathError {
    pub fn get_span(&self) -> Option<Span> {
        match *self {
            MathError::ParseError(ref e) => e.get_span(),
            MathError::EvaluateError(ref e) => e.get_span(),
        }
    }
}

impl From<parser::ParseError> for MathError {
    fn from(e: parser::ParseError) -> MathError {
        MathError::ParseError(e)
    }
}

impl From<evaluator::EvaluateError> for MathError {
    fn from(e: evaluator::EvaluateError) -> MathError {
        MathError::EvaluateError(e)
    }
}
