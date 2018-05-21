use super::Span;
use std::num::ParseFloatError;

#[derive(Debug, Clone)]
pub struct Expression {
    pub span: Span,
    pub expression: ExpressionEnum,
}

impl PartialEq for Expression {
    fn eq(&self, other: &Expression) -> bool {
        self.expression == other.expression
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionEnum {
    Constant(f32),
    Add,
    Minus,
    Multiply,
    Divide,
    Block(Vec<Expression>),
}

struct Reader {
    str: Vec<char>,
    index: usize,
}

impl<'a> From<&'a str> for Reader {
    fn from(s: &str) -> Reader {
        Reader {
            str: s.chars().collect(),
            index: 0,
        }
    }
}

impl Reader {
    pub fn take(&mut self) -> Option<char> {
        let result = self.peek();
        self.index += 1;
        result
    }
    pub fn peek(&mut self) -> Option<char> {
        self.str.get(self.index).cloned()
    }

    pub fn is_at_eof(&self) -> bool {
        self.index >= self.str.len()
    }
}

const OPERATORS: &[(char, ExpressionEnum)] = &[
    ('*', ExpressionEnum::Multiply),
    ('/', ExpressionEnum::Divide),
    ('-', ExpressionEnum::Minus),
    ('+', ExpressionEnum::Add),
];

impl Expression {
    pub fn parse(input: impl AsRef<str>) -> Result<Expression, ParseError> {
        let mut reader: Reader = input.as_ref().into();
        let mut result = Vec::new();
        while let Some(e) = Expression::parse_impl(&mut reader)? {
            result.push(e);
        }
        if result.is_empty() {
            Err(ParseError::UnexpectedEnd)
        } else if !reader.is_at_eof() {
            Err(ParseError::TrailingTokens {
                span: Span {
                    from: reader.index,
                    to: reader.str.len(),
                },
            })
        } else if result.len() == 1 {
            let first = result.into_iter().next().unwrap();
            Ok(first)
        } else {
            Ok(Expression {
                span: Span {
                    from: 0,
                    to: reader.index,
                },
                expression: ExpressionEnum::Block(result),
            })
        }
    }

    fn parse_block(reader: &mut Reader) -> Result<Option<Expression>, ParseError> {
        let start_index = reader.index;
        let start = reader.take().unwrap();
        let end_token = if start == '(' {
            ')'
        } else if start == '[' {
            ']'
        } else {
            '}'
        };
        let mut inner = Vec::new();
        while let Some(child) = Expression::parse_impl(reader)? {
            inner.push(child);
        }
        if reader.take() != Some(end_token) {
            return Err(ParseError::MissingEndBracket {
                start_bracket: start,
                span: Span {
                    from: start_index,
                    to: reader.index,
                },
            });
        }
        Ok(Some(Expression {
            span: Span {
                from: start_index,
                to: reader.index,
            },
            expression: ExpressionEnum::Block(inner),
        }))
    }

    fn parse_impl(reader: &mut Reader) -> Result<Option<Expression>, ParseError> {
        let mut c = match reader.peek() {
            Some(c) => c,
            None => return Ok(None),
        };
        while c.is_whitespace() {
            reader.take();
            c = match reader.peek() {
                Some(c) => c,
                None => return Ok(None),
            };
        }
        let operator = OPERATORS.iter().find(|o| o.0 == c);
        if let Some(operator) = operator {
            reader.take();
            return Ok(Some(Expression {
                span: Span {
                    from: reader.index - 1,
                    to: reader.index,
                },
                expression: operator.1.clone(),
            }));
        } else if c == '(' || c == '[' || c == '{' {
            Expression::parse_block(reader)
        } else if c == ')' || c == ']' || c == '}' {
            Ok(None)
        } else if c.is_numeric() || c == '-' {
            let mut str = c.to_string();
            let start_index = reader.index;
            reader.take();
            while let Some(c) = reader.peek() {
                if c.is_numeric() || c == '.' {
                    str.push(c);
                    reader.take();
                } else if c == '_' {
                    continue;
                } else {
                    break;
                }
            }
            match str.parse() {
                Err(e) => Err(ParseError::ParseFloatError {
                    span: Span {
                        from: start_index,
                        to: reader.index,
                    },
                    error: e,
                }),
                Ok(v) => Ok(Some(Expression {
                    span: Span {
                        from: start_index,
                        to: reader.index,
                    },
                    expression: ExpressionEnum::Constant(v),
                })),
            }
        } else {
            Err(ParseError::InvalidToken {
                token: c,
                span: Span {
                    from: reader.index,
                    to: reader.index + 1,
                },
            })
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEnd,
    TrailingTokens { span: Span },
    InvalidToken { span: Span, token: char },
    MissingEndBracket { start_bracket: char, span: Span },
    ParseFloatError { span: Span, error: ParseFloatError },
}

impl ParseError {
    pub fn get_span(&self) -> Option<Span> {
        match *self {
            ParseError::TrailingTokens { span, .. }
            | ParseError::InvalidToken { span, .. }
            | ParseError::MissingEndBracket { span, .. }
            | ParseError::ParseFloatError { span, .. } => Some(span),
            ParseError::UnexpectedEnd => None,
        }
    }
}
