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

#[derive(Debug)]
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
      return SyntaxToken {
        syntax_type: SyntaxType::EndOfFileToken,
        value: None,
        name: "".to_string(),
        position: self.position,
      };
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

      return SyntaxToken {
        syntax_type: SyntaxType::NumberToken,
        value: Some(TokenValue::Integer(value.unwrap())),
        position: start,
        name,
      };
    }

    if current.is_whitespace() {
      let start = self.position;

      while self.current().is_some() && self.current().unwrap().is_whitespace() {
        self.next()
      }

      let name: String = self.text[start..self.position].into_iter().collect();
      return SyntaxToken {
        syntax_type: SyntaxType::WhitespaceToken,
        position: start,
        value: None,
        name,
      };
    }

    self.position += 1;
    self.diagonistics.push(format!(
      "ERROR: bad character input: '{}'",
      self.current().unwrap_or(&'\0')
    ));

    return SyntaxToken {
      syntax_type: SyntaxType::BadToken,
      value: None,
      name: "".to_string(),
      position: self.position - 1,
    };
  }
}
