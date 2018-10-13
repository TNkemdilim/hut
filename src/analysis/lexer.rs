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
    self.text.get(self.position)
  }

  fn next(&mut self) {
    self.position += 1;
  }

  pub fn next_token(&mut self) -> SyntaxToken<'a> {
    let token = match self.current() {
      // Lex end of file
      None => SyntaxToken::new(
        SyntaxType::EndOfFileToken,
        None,
        "".to_string(),
        self.position,
      ),

      // Lex digits
      Some(ref current) if current.is_digit(10) => {
        let start = self.position;
        while self.current().is_some() && self.current().unwrap().is_digit(10) {
          self.next()
        }
        let name: String = self.text[start..self.position].into_iter().collect();
        let value = name.parse::<i32>();

        if value.is_err() {
          self
            .diagonistics
            .push(format!("The number {} isn't a valid Int32.", name).to_string());

          return SyntaxToken::new_without_value(SyntaxType::BadToken, name, start);
        }

        self.position -= 1;
        SyntaxToken::new(
          SyntaxType::NumberToken,
          Some(TokenValue::Integer(value.unwrap())),
          name,
          start,
        )
      }

      // Lex whitespaces
      Some(ref current) if current.is_whitespace() => {
        let start = self.position;
        while self.current().is_some() && self.current().unwrap().is_whitespace() {
          self.next()
        }

        let name: String = self.text[start..self.position].into_iter().collect();
        SyntaxToken::new_without_value(SyntaxType::WhitespaceToken, name, start)
      }

      // Lex Operators
      Some(ref operator) => match operator {
        '+' => {
          SyntaxToken::new_without_value(SyntaxType::PlusToken, operator.to_string(), self.position)
        }
        '-' => {
          SyntaxToken::new_without_value(SyntaxType::PlusToken, operator.to_string(), self.position)
        }
        '*' => {
          SyntaxToken::new_without_value(SyntaxType::StarToken, operator.to_string(), self.position)
        }
        '/' => SyntaxToken::new_without_value(
          SyntaxType::SlashToken,
          operator.to_string(),
          self.position,
        ),
        '(' => SyntaxToken::new_without_value(
          SyntaxType::OpenParenthesisToken,
          operator.to_string(),
          self.position,
        ),
        ')' => SyntaxToken::new_without_value(
          SyntaxType::CloseParenthesisToken,
          operator.to_string(),
          self.position,
        ),
        _ => {
          let opt = operator.to_string();
          self
            .diagonistics
            .push(format!("ERROR: bad character input: '{}'", opt));

          SyntaxToken::new_without_value(SyntaxType::BadToken, opt, self.position - 1)
        }
      },
    };

    self.next();
    token
  }
}
