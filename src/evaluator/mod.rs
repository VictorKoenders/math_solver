use parser::Expression;

pub fn evaluate(expr: Expression) -> Result<f32, EvaluateError> {
    match expr {
        Expression::Constant(val) => Ok(val),
        Expression::Block(mut statements) => {
            if statements.len() == 0 {
                return Err(EvaluateError::EmptyBlock);
            }
            let operations: &[(_, fn(Expression, Expression) -> Result<Expression, EvaluateError>)] = &[
                (Expression::Multiply, multiply),
                (Expression::Divide, divide),
                (Expression::Add, add),
                (Expression::Minus, minus),
            ];
            for operation in operations {
                for i in (0..statements.len() - 2).rev() {
                    if statements[i + 1] == operation.0 {
                        let left = statements.remove(i);
                        let right = statements.remove(i + 1);
                        statements[i] = operation.1(left, right)?;
                    }
                }
                if statements.len() < 3 { break; }
            }
            if statements.len() != 1 {
                Err(EvaluateError::InvalidExpression)
            } else if let Expression::Constant(v) = statements[0] {
                Ok(v)
            } else {
                Err(EvaluateError::InvalidExpression)
            }
            /*
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
            */
        }
        x => Err(EvaluateError::UnexpectedToken(x)),
    }
}

fn multiply(left: Expression, right: Expression) -> Result<Expression, EvaluateError> {
    let left = if let Expression::Constant(v) = left {
        v
    } else {
        return Err(EvaluateError::ExpectedConstant { got: left });
    };
    let right = if let Expression::Constant(v) = right {
        v
    } else {
        return Err(EvaluateError::ExpectedConstant { got: right });
    };

    Ok(Expression::Constant(left * right))
}
fn divide(left: Expression, right: Expression) -> Result<Expression, EvaluateError> {
    let left = if let Expression::Constant(v) = left {
        v
    } else {
        return Err(EvaluateError::ExpectedConstant { got: left });
    };
    let right = if let Expression::Constant(v) = right {
        v
    } else {
        return Err(EvaluateError::ExpectedConstant { got: right });
    };

    Ok(Expression::Constant(left / right))
}
fn add(left: Expression, right: Expression) -> Result<Expression, EvaluateError> {
    let left = if let Expression::Constant(v) = left {
        v
    } else {
        return Err(EvaluateError::ExpectedConstant { got: left });
    };
    let right = if let Expression::Constant(v) = right {
        v
    } else {
        return Err(EvaluateError::ExpectedConstant { got: right });
    };

    Ok(Expression::Constant(left + right))
}
fn minus(left: Expression, right: Expression) -> Result<Expression, EvaluateError> {
    let left = if let Expression::Constant(v) = left {
        v
    } else {
        return Err(EvaluateError::ExpectedConstant { got: left });
    };
    let right = if let Expression::Constant(v) = right {
        v
    } else {
        return Err(EvaluateError::ExpectedConstant { got: right });
    };

    Ok(Expression::Constant(left - right))
}

#[derive(Debug)]
pub enum EvaluateError {
    UnexpectedToken(Expression),
    ExpectedConstant { got: Expression },
    ExpectedOperation { got: Expression },
    MissingRightHandSide,
    EmptyBlock,
    InvalidExpression,
}
