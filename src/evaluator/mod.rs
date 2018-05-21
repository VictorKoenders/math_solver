use super::Span;
use parser::{Expression, ExpressionEnum};

type EvaluatorFn = fn(Expression, Expression) -> Result<Expression, EvaluateError>;
pub fn evaluate(expr: Expression) -> Result<f32, EvaluateError> {
    match expr.expression {
        ExpressionEnum::Constant(val) => Ok(val),
        ExpressionEnum::Block(mut statements) => {
            if statements.is_empty() {
                return Err(EvaluateError::EmptyBlock { span: expr.span });
            }
            let operations: &[(_, EvaluatorFn)] = &[
                (ExpressionEnum::Multiply, multiply),
                (ExpressionEnum::Divide, divide),
                (ExpressionEnum::Add, add),
                (ExpressionEnum::Minus, minus),
            ];
            for operation in operations {
                let mut i = 0;
                while statements.len() > 2 && i < statements.len() - 2 {
                    if statements[i + 1].expression == operation.0 {
                        let left = statements.remove(i);
                        let right = statements.remove(i + 1);
                        statements[i] = operation.1(left, right)?;
                    } else {
                        i += 1;
                    }
                }
            }
            if statements.len() != 1 {
                Err(EvaluateError::InvalidExpression {
                    span: statements
                        .iter()
                        .skip(1)
                        .fold(Span { from: 0, to: 0 }, |acc, stm| {
                            if acc.from == 0 && acc.to == 0 {
                                stm.span
                            } else {
                                acc.merge(stm.span)
                            }
                        }),
                })
            } else if let ExpressionEnum::Constant(v) = statements[0].expression {
                Ok(v)
            } else {
                Err(EvaluateError::InvalidExpression {
                    span: statements[0].span,
                })
            }
        }
        _ => Err(EvaluateError::UnexpectedToken(expr)),
    }
}

fn multiply(left: Expression, right: Expression) -> Result<Expression, EvaluateError> {
    let left_span = left.span;
    let right_span = right.span;
    let left = evaluate(left)?;
    let right = evaluate(right)?;

    Ok(Expression {
        span: left_span.merge(right_span),
        expression: ExpressionEnum::Constant(left * right),
    })
}
fn divide(left: Expression, right: Expression) -> Result<Expression, EvaluateError> {
    let left_span = left.span;
    let right_span = right.span;
    let left = evaluate(left)?;
    let right = evaluate(right)?;

    Ok(Expression {
        span: left_span.merge(right_span),
        expression: ExpressionEnum::Constant(left / right),
    })
}
fn add(left: Expression, right: Expression) -> Result<Expression, EvaluateError> {
    let left_span = left.span;
    let right_span = right.span;
    let left = evaluate(left)?;
    let right = evaluate(right)?;

    Ok(Expression {
        span: left_span.merge(right_span),
        expression: ExpressionEnum::Constant(left + right),
    })
}
fn minus(left: Expression, right: Expression) -> Result<Expression, EvaluateError> {
    let left_span = left.span;
    let right_span = right.span;
    let left = evaluate(left)?;
    let right = evaluate(right)?;

    Ok(Expression {
        span: left_span.merge(right_span),
        expression: ExpressionEnum::Constant(left - right),
    })
}

#[derive(Debug)]
pub enum EvaluateError {
    UnexpectedToken(Expression),
    ExpectedConstant { got: Expression },
    ExpectedOperation { got: Expression },
    MissingRightHandSide { span: Span },
    EmptyBlock { span: Span },
    InvalidExpression { span: Span },
}

impl EvaluateError {
    pub fn get_span(&self) -> Option<Span> {
        match *self {
            EvaluateError::ExpectedConstant { ref got }
            | EvaluateError::UnexpectedToken(ref got)
            | EvaluateError::ExpectedOperation { ref got } => Some(got.span),
            EvaluateError::MissingRightHandSide { span }
            | EvaluateError::EmptyBlock { span }
            | EvaluateError::InvalidExpression { span } => Some(span),
        }
    }
}
