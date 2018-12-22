use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Values
    Number(Option<i32>), // improve by intriducing float, int32, bool etc.

    // Misc
    EndOfFileToken,
    WhitespaceToken,

    // Operators
    Plus,
    Minus,
    Star,
    ForwardSlash,
    OpenParenthesis,
    CloseParenthesis,
}

#[derive(Debug)]
pub struct SyntaxToken {
    pub token_type: Token,
    pub position: usize,
}

impl SyntaxToken {
    pub fn new(token_type: Token, position: usize) -> Self {
        SyntaxToken {
            token_type,
            position,
        }
    }

    pub fn end_of_file(position: usize) -> Self {
        Self::new(Token::EndOfFileToken, position)
    }

    pub fn whitespace(position: usize) -> Self {
        Self::new(Token::WhitespaceToken, position)
    }

    pub fn digit(value: i32, position: usize) -> Self {
        Self::new(Token::Number(Some(value)), position)
    }

    pub fn operator(token_type: Token, position: usize) -> Self {
        SyntaxToken {
            token_type,
            position,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let m = match self {
            Token::Number(_) => format!("integer"),
            Token::EndOfFileToken => format!("EOF"),

            // Todo: Implement the remaining display for remaining error types.
            _ => format!("{:#?}", self),
        };

        write!(f, "{}", m)
    }
}
