use crate::parser::token::{Operator, Token};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Binary {
	Add,
	Minus,
	Multiply,
	Divide,
	Mod,

	Equals,
	NotEquals,
	Greater,
	GreaterOrEquals,
	Less,
	LessOrEquals,
	Assignment,
	ShiftLeft,
	ShiftRight,

	BitAnd,
	BitOr,
	BitXor,

	And,
	Or,

	Dot,
}

impl TryFrom<Operator> for Binary {
	type Error = ();

	fn try_from(value: Operator) -> Result<Self, Self::Error> {
		Ok(match value {
			Operator::Add => Self::Add,
			Operator::Minus => Self::Minus,
			Operator::Star => Self::Multiply,
			Operator::Divide => Self::Divide,
			Operator::Mod => Self::Mod,
			Operator::Dot => Self::Dot,
			Operator::Equals => Self::Equals,
			Operator::Greater => Self::Greater,
			Operator::GreaterOrEquals => Self::GreaterOrEquals,
			Operator::Less => Self::Less,
			Operator::LessOrEquals => Self::LessOrEquals,
			Operator::Assignment => Self::Assignment,
			Operator::ShiftLeft => Self::ShiftLeft,
			Operator::ShiftRight => Self::ShiftRight,
			Operator::And => Self::And,
			Operator::Or => Self::Or,
			Operator::Xor => Self::BitXor,
			Operator::NotEquals => Self::NotEquals,
			_ => return Err(())
		})
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Unary {
	Negate,
	Not,
	Reference,
	Dereference,
}

impl TryFrom<Operator> for Unary {
	type Error = ();

	fn try_from(value: Operator) -> Result<Self, Self::Error> {
		Ok(match value {
			Operator::Minus => Self::Negate,
			Operator::Not => Self::Not,
			Operator::Reference => Self::Reference,
			Operator::Star => Self::Dereference,
			_ => return Err(())
		})
	}
}

impl TryFrom<&Token> for Unary {
	type Error = ();

	fn try_from(value: &Token) -> Result<Self, Self::Error> {
		if let Token::Operator(op) = value { (*op).try_into() } else { Err(()) }
	}
}