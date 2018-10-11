use crate::analysis::*;

pub struct Lexer {
  text: Vec<char>,
  position: usize,
  diagonistics: Vec<String>,
}

impl<'a> Lexer {
  pub fn new(text: &'a mut str) -> Self {
    Lexer {
      text: text.chars().collect(),
      position: 0,
      diagonistics: Vec::new(),
    }
  }

  fn current(&'a self) -> Option<&'a char> {
    let i = self.position;
    self.text.get(i)
  }

  fn next(&mut self) {
    self.position += 1;
  }

  pub fn next_token(&mut self) -> SyntaxToken<'a> {
    let current = self.current();

    if current.is_none() {
      return SyntaxToken::new(
        SyntaxType::EndOfFileToken,
        None,
        "".to_string(),
        self.position,
      );
    }

    let current = current.unwrap();

    if current.is_digit(10) {
      let start = self.position;

      while self.current().is_some() && self.current().unwrap().is_digit(10) {
        self.next()
      }

      let name: String = self.text[start..self.position].into_iter().collect();

      let value = name.parse::<i32>();
      if let Err(_) = value {
        self
          .diagonistics
          .push(format!("The number {} isn't a valid Int32.", name).to_string());
      }

      return SyntaxToken::new(
        SyntaxType::NumberToken,
        Some(TokenValue::Integer(value.unwrap())),
        name,
        start,
      );
    }

    if current.is_whitespace() {
      let start = self.position;
      while self.current().is_some() && self.current().unwrap().is_whitespace() {
        self.next()
      }
      let name: String = self.text[start..self.position].into_iter().collect();

      return SyntaxToken::new(SyntaxType::WhitespaceToken, None, name, start);
    }

    if *current == '+' {
      self.position += 1;
      return SyntaxToken::new(SyntaxType::PlusToken, None, "+".to_string(), self.position);
    } else if *current == '-' {
      self.position += 1;
      return SyntaxToken::new(SyntaxType::PlusToken, None, "+".to_string(), self.position);
    } else if *current == '*' {
      self.position += 1;
      return SyntaxToken::new(SyntaxType::StarToken, None, "*".to_string(), self.position);
    } else if *current == '/' {
      self.position += 1;
      return SyntaxToken::new(SyntaxType::SlashToken, None, "/".to_string(), self.position);
    } else if *current == '(' {
      self.position += 1;
      return SyntaxToken::new(
        SyntaxType::OpenParenthesisToken,
        None,
        "(".to_string(),
        self.position,
      );
    } else if *current == ')' {
      self.position += 1;
      return SyntaxToken::new(
        SyntaxType::CloseParenthesisToken,
        None,
        ")".to_string(),
        self.position,
      );
    }

    self.diagonistics.push(format!(
      "ERROR: bad character input: '{}'",
      self.current().unwrap_or(&'\0')
    ));

    return SyntaxToken::new(
      SyntaxType::BadToken,
      None,
      "".to_string(),
      self.position - 1,
    );
  }
}
