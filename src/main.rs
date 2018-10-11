mod analysis;

use self::analysis::{lexer::Lexer, SyntaxType};
use std::io::{self, Write};

fn main() -> Result<(), io::Error> {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;

        if line.is_empty() || line == " " {
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, ""))?;
        }

        let mut lexer = Lexer::new(&mut line);
        loop {
            let token = lexer.next_token();
            println!("{:#?}", token);

            match token.syntax_type {
                SyntaxType::EndOfFileToken => break,
                _ => {}
            }
        }
    }
}
