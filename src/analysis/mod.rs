pub mod lexer;

#[derive(Debug)]
pub enum SyntaxType {
  EndOfFileToken,
  BadToken,
  NumberToken,
  WhitespaceToken,

  // Operators
  PlusToken,
  StarToken,
  SlashToken,
  OpenParenthesisToken,
  CloseParenthesisToken,
}

#[derive(Debug)]
pub enum TokenValue<'a> {
  Integer(i32),
  String(&'a str),
}

#[derive(Debug)]
pub struct SyntaxToken<'a> {
  pub syntax_type: SyntaxType,
  value: Option<TokenValue<'a>>,
  name: String,
  position: usize,
}

impl<'a> SyntaxToken<'a> {
  pub fn new(
    syntax_type: SyntaxType,
    value: Option<TokenValue<'a>>,
    name: String,
    position: usize,
  ) -> Self {
    SyntaxToken {
      syntax_type,
      value,
      name,
      position,
    }
  }
}
