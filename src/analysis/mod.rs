pub mod lexer;

#[derive(Debug)]
pub enum SyntaxType {
  EndOfFileToken,
  BadToken,
  NumberToken,
  WhitespaceToken,
}

#[derive(Debug)]
enum TokenValue<'a> {
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
