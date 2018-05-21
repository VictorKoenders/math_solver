use parser::Expression;

pub fn evaluate(expr: Expression) -> Result<f32, EvaluateError> {
    let mut result;
    match expr {
        Expression::Constant(val) => result = val,
        Expression::Block(statements) => {
            if statements.len() == 0 {
                return Err(EvaluateError::EmptyBlock);
            }
            let mut iter = statements.into_iter();
            result = evaluate(iter.next().unwrap())?;
            while let Some(expr) = iter.next() {
                let rhs = evaluate(match iter.next() {
                    Some(v) => v,
                    None => return Err(EvaluateError::MissingRightHandSide),
                })?;
                match expr {
                    Expression::Add => result += rhs,
                    Expression::Divide => result /= rhs,
                    Expression::Minus => result -= rhs,
                    Expression::Multiply => result *= rhs,
                    x => return Err(EvaluateError::ExpectedOperation { got: x })
                }
            }
        },
        x => return Err(EvaluateError::UnexpectedToken(x)),
    }
    Ok(result)
}

#[derive(Debug)]
pub enum EvaluateError {
    UnexpectedToken(Expression),
    ExpectedOperation { got: Expression },
    MissingRightHandSide,
    EmptyBlock,
}
