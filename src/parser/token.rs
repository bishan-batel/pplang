use std::fmt::{Debug};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
	Keyword(Keyword),
	Operator(Operator),
	Parenthetical(Parenthetical),
	Identifier(String),
	Literal(Literal),
}


#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
	String(String),
	Character(char),
	Integer(i64),
	Float(f64),
	Bool(bool),
}

impl From<Literal> for Token {
	fn from(value: Literal) -> Self {
		Self::Literal(value)
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Parenthetical {
	NormalOpen,
	NormalClose,
	BracketOpen,
	BracketClose,
	CurlyOpen,
	CurlyClose,
}

impl Parenthetical {
	const fn is_closer_for(self, opener: Self) -> bool {
		matches!((opener, self),
			(Self::NormalOpen, Self::NormalClose) |
			(Self::BracketOpen, Self::BracketClose) |
			(Self::CurlyOpen, Self::CurlyClose)
		)
	}

	const fn is_opener_for(self, closing: Self) -> bool {
		closing.is_closer_for(self)
	}
}

impl From<Parenthetical> for Token {
	fn from(value: Parenthetical) -> Self {
		Self::Parenthetical(value)
	}
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
	Add,
	Minus,
	Multiply,
	Divide,
	Mod,
	Comma,
	Dot,

	Equals,
	Greater,
	GreaterOrEquals,
	Less,
	LessOrEquals,
	Assignment,

	And,
	Or,
	Not,
	Xor,

	ThinArrow,
	Arrow,
}

impl From<Operator> for Token {
	fn from(value: Operator) -> Self {
		Self::Operator(value)
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Keyword {
	Function,
	Return,
	Break,
	While,
	For,
	Let,
	Var,
	In,
}


impl From<Keyword> for Token {
	fn from(value: Keyword) -> Self {
		Self::Keyword(value)
	}
}

impl TryFrom<&str> for Keyword {
	type Error = ();

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Ok(match value {
			"function" => Self::Function,
			"return" => Self::Return,
			"break" => Self::Break,
			"while" => Self::While,
			"for" => Self::For,
			"let" => Self::Let,
			"var" => Self::Var,
			"in" => Self::In,
			_ => return Err(())
		})
	}
}