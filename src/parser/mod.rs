use std::num::ParseFloatError;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
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

impl Expression {
    pub fn parse(input: impl AsRef<str>) -> Result<Expression, ParseError> {
        let mut reader: Reader = input.as_ref().into();
        let mut result = Vec::new();
        while let Some(e) = Expression::parse_impl(&mut reader)? {
            result.push(e);
        }
        if result.len() == 0 {
            Err(ParseError::UnexpectedEnd)
        } else if !reader.is_at_eof() {
            Err(ParseError::TrailingTokens)
        } else if result.len() == 1 {
            let first = result.into_iter().next().unwrap();
            Ok(first)
        } else {
            Ok(Expression::Block(result))
        }
    }

    fn parse_impl(reader: &mut Reader) -> Result<Option<Expression>, ParseError> {
        loop {
            return match reader.peek() {
                Some(x) if x == '(' || x == '[' || x == '{' => {
                    let start = x;
                    let end_token = if start == '(' {
                            ')'
                        } else if start == '[' {
                            ']'
                        } else {
                            '}'
                        };
                        let start_index = reader.index;
                    let mut inner = Vec::new();
                    reader.take();
                    while let Some(child) = Expression::parse_impl(reader)? {
                        inner.push(child);
                    }
                    if reader.take() != Some(end_token) {
                        return Err(ParseError::MissingEndBracket {
                            start_bracket: start,
                            start_index,
                        });
                    }
                    Ok(Some(Expression::Block(inner)))
                }
                Some('+') => {
                    reader.take();
                    Ok(Some(Expression::Add))
                }
                Some('-') => {
                    reader.take();
                    Ok(Some(Expression::Minus))
                }
                Some('*') => {
                    reader.take();
                    Ok(Some(Expression::Multiply))
                }
                Some('/') => {
                    reader.take();
                    Ok(Some(Expression::Divide))
                }
                Some(x) if x.is_whitespace() => {
                    reader.take();
                    continue;
                }
                Some(x) if x.is_numeric() || x == '.' => {
                    let mut str = x.to_string();
                    let start_index = reader.index;
                    reader.take();
                    while let Some(x) = reader.peek() {
                        if x.is_numeric() || x == '.' {
                            str.push(x);
                            reader.take();
                        } else {
                            break;
                        }
                    }
                    match str.parse() {
                        Err(e) => Err(ParseError::ParseFloatError {
                            index: start_index,
                            error: e,
                        }),
                        Ok(v) => Ok(Some(Expression::Constant(v))),
                    }
                }
                Some(x) if x == ')' || x == '[' || x == '}' => Ok(None),
                Some(x) => Err(ParseError::InvalidToken {
                    token: x,
                    index: reader.index,
                }),
                None => Ok(None),
            };
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedEnd,
    TrailingTokens,
    InvalidToken {
        index: usize,
        token: char,
    },
    MissingEndBracket {
        start_bracket: char,
        start_index: usize,
    },
    ParseFloatError {
        index: usize,
        error: ParseFloatError,
    },
}

impl ParseError {
    pub fn get_position(&self) -> Option<usize> {
        match *self {
            ParseError::MissingEndBracket { start_index, .. } => Some(start_index),
            ParseError::InvalidToken { index, .. } | ParseError::ParseFloatError { index, .. } => Some(index),
            _ => None,
        }
    }
}
