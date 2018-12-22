use crate::analysis::{symbols::*, LexerError, LexerTokenStream, SyntaxToken, Token};
use std::{iter::Peekable, str::Chars};

pub(crate) struct Lexer<'a> {
	text: Peekable<Chars<'a>>,
	position: usize,
}

impl<'a> Lexer<'a> {
	pub fn new(text: &'a str) -> Self {
		let chrs = text.chars();

		Lexer {
			text: chrs.into_iter().peekable(),
			position: 0,
		}
	}

	fn peek(&mut self) -> Option<&char> {
		self.text.peek()
	}

	fn next(&mut self) -> Option<char> {
		self.position += 1;
		self.text.next()
	}

	fn lex_digit(&mut self) -> Result<SyntaxToken, LexerError> {
		let mut digits = vec![self.next().unwrap()];
		let start = self.position;

		while self.peek().is_some() && self.peek().unwrap().is_digit(10) {
			digits.push(self.next().ok_or(LexerError::ParseIntegerError {
				position: self.position,
			})?);
		}

		Ok(SyntaxToken::digit(
			digits
				.into_iter()
				.collect::<String>()
				.parse::<i32>()
				.map_err(|_| {
					LexerError::ParseIntegerError {
						position: self.position,
					}
				})?,
			start,
		))
	}

	fn lex_whitespace(&mut self) -> SyntaxToken {
		self.next().unwrap();

		while self.peek().is_some() && self.peek().unwrap().is_whitespace() {
			self.next();
		}

		SyntaxToken::whitespace(self.position)
	}

	fn lex_operators(&mut self) -> Result<SyntaxToken, LexerError> {
		let operator = self.next().unwrap();

		match operator {
			PLUS => Ok(SyntaxToken::operator(Token::Plus, self.position)),

			MINUS => Ok(SyntaxToken::operator(Token::Minus, self.position)),

			STAR => Ok(SyntaxToken::operator(Token::Star, self.position)),

			FORWARD_SLASH => Ok(SyntaxToken::operator(Token::ForwardSlash, self.position)),

			OPEN_PARANTHESIS => Ok(SyntaxToken::operator(Token::OpenParenthesis, self.position)),

			CLOSE_PARANTHESIS => {
				Ok(SyntaxToken::operator(
					Token::CloseParenthesis,
					self.position,
				))
			}
			_ => {
				Err(LexerError::InvalidToken {
					position: self.position,
				})
			}
		}
	}

	fn lex_token(&mut self) -> Result<SyntaxToken, LexerError> {
		let token = self.peek();

		match token {
			None => Ok(SyntaxToken::end_of_file(self.position)),
			Some(ref next_value) if next_value.is_digit(10) => self.lex_digit(),
			Some(ref next_value) if next_value.is_whitespace() => Ok(self.lex_whitespace()),
			Some(_) => self.lex_operators(),
		}
	}

	pub(crate) fn lex(&mut self) -> LexerTokenStream {
		let mut v = vec![];
		let mut diagnostics = vec![];

		loop {
			match self.lex_token() {
				Ok(ref a) if a.token_type == Token::EndOfFileToken => break,
				Ok(a) => {
					// skips all whitespace tokens as they aren't needed
					if a.token_type != Token::WhitespaceToken {
						v.push(a);
					}
				}
				Err(err) => diagnostics.push(format!("{}", err)),
			}
		}

		(v, diagnostics)
	}
}
