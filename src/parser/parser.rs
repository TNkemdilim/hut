use crate::{
    analysis::{Lexer, SyntaxToken, Token},
    parser::Syntax,
};
use std::{fmt, iter::Peekable};

#[derive(Debug)]
enum ParserError {
    UnexpectedToken {
        position: usize,
        found: Token,
        expected: Token,
    },
}

#[derive(Debug)]
pub struct Parser {
    pub(crate) tokens: Peekable<std::vec::IntoIter<SyntaxToken>>,
    pub(crate) diagonistics: Vec<String>,
    position: usize,
    text: String,
}

impl<'a> Parser {
    pub fn new(text: &'a str) -> Self {
        let token_stream = Lexer::new(&text).lex();

        Parser {
            tokens: token_stream.0.into_iter().peekable(),
            diagonistics: token_stream.1,
            position: 0,
            text: text.to_string(),
        }
    }

    // fn peek(&mut self) -> Option<&SyntaxToken> {
    // 	self.tokens.peek()
    // }

    fn next(&mut self) -> Option<SyntaxToken> {
        self.position += 1;
        self.tokens.next()
    }

    fn parse_primary_expression(&mut self) -> Syntax {
        let token = self.next();

        if let Some(current) = token {
            if let Token::Number(_) = current.token_type {
                return Syntax::NumberExpression {
                    value: current.token_type,
                };
            }
        }

        // End of file reached, as token returned a None
        self.diagonistics.push(format!(
            "{}",
            ParserError::UnexpectedToken {
                position: self.text.len() - 1,
                found: Token::EndOfFileToken,
                expected: Token::Number(None)
            }
        ));

        Syntax::NumberExpression {
            value: Token::Number(None),
        }
    }

    pub fn parse(&mut self) -> Syntax {
        let mut left = self.parse_primary_expression();

        while let Some(token) = self.next() {
            match token.token_type {
                Token::Plus | Token::Minus => {
                    left = Syntax::BinaryExpression {
                        left: Box::new(left),
                        op: token.token_type,
                        right: Box::new(self.parse_primary_expression()),
                    };
                }

                _ => {}
            }
        }

        left
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = match self {
            ParserError::UnexpectedToken {
                position,

                
                found,
                expected,
            } => format!(
                "Unexpected {} found at position: {}. Expected {}",
                found, position, expected
            ),
        };

        write!(f, "{}", a)
    }
}

mod test {
    #[test]
    fn no_withespace_token() {}
}
