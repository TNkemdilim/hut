mod lexer;
pub(crate) mod symbols;
mod tokens;

pub(crate) use self::{lexer::Lexer, tokens::*};
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub(crate) enum LexerError {
	ParseIntegerError { position: usize },
	InvalidToken { position: usize },
}

type LexerTokenStream = (Vec<SyntaxToken>, Vec<String>);

impl fmt::Display for LexerError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let messsage = match self {
			LexerError::ParseIntegerError { position } => {
				format!("Invalid integer at position: {}", position)
			}
			LexerError::InvalidToken { position } => {
				format!("Invalid token at position: {}", position)
			}
		};

		write!(f, "{}", messsage)
	}
}
