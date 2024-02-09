use std::fmt::{Debug};

#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub struct TracedToken {
	pub(crate) token: Token,
	pub(crate) trace: Trace,
}

impl From<TracedToken> for Token {
	fn from(value: TracedToken) -> Self {
		value.token
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FilePos {
	pub row: usize,
	pub column: usize,
}

impl FilePos {
	pub const fn beginning() -> Self {
		Self {
			row: 0,
			column: 0,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Trace {
	pub source_file: String,
	pub begin_pos: FilePos,
	pub end_pos: FilePos,
}

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
	Colon,

	Reference,

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
	As,
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
			"as" => Self::As,
			_ => return Err(())
		})
	}
}