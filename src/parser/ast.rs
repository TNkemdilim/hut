use crate::analysis::Token;

#[derive(Debug)]
pub enum Syntax {
    NumberExpression {
        value: Token,
    },

    BinaryExpression {
        left: Box<Syntax>,
        op: Token,
        right: Box<Syntax>,
    },
}

// impl Syntax {
// 	fn get_children(&mut self) -> &Syntax {
// 		match self {
// 			Syntax::BinaryExpression { left, op, right } => self,
// 			Syntax::NumberExpression { value } => self,
// 		}
// 	}
// }
