mod parser;
mod evaluator;


pub fn evaluate(statement: String) -> Result<f32, MathError> {
    let expression = parser::Expression::parse(&statement)?;
    evaluator::evaluate(expression).map_err(Into::into)
}

#[derive(Debug)]
pub enum MathError {
    ParseError(parser::ParseError),
    EvaluateError(evaluator::EvaluateError)
}

impl MathError {
    pub fn get_position(&self) -> Option<usize> {
        match *self {
            MathError::ParseError(ref e) => e.get_position(),
            _ => None,
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