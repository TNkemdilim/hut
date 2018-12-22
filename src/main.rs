mod analysis;
mod parser;

use self::parser::Parser;
use std::io::{self, Write};
use term;

fn main() -> Result<(), io::Error> {
	loop {
        let mut t = term::stdout().unwrap();
		print!("> ");
        io::stdout().flush().unwrap();
		
        let mut line = String::new();
		io::stdin().read_line(&mut line)?;

		if line.is_empty() || line == " " {
			Err(io::Error::new(io::ErrorKind::UnexpectedEof, ""))?;
		};

		let mut parser = Parser::new(&mut line);
		println!("{:#?}", parser.parse());
		println!("\n");

        
		parser.diagonistics.iter().for_each(|s| {
            t.fg(term::color::RED).unwrap();
            write!(t, "{:?}\n", s).unwrap();
		});

        t.reset().unwrap();
	}
}
