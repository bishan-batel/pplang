use std::fmt::{Debug};
use crate::parser::ast::Expression;
use crate::parser::ast::variable::Identifier;

pub type TracedTokenList = Vec<TracedToken>;

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
	Identifier(Identifier),
	Literal(Literal),
	EOF,
}

impl From<Identifier> for Token {
	fn from(value: Identifier) -> Self {
		Self::Identifier(value)
	}
}


#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
	String(String),
	Character(char),
	Integer(i64),
	Float(f64),
	Bool(bool),
	Unit,
}

impl From<Literal> for Token {
	fn from(value: Literal) -> Self {
		Self::Literal(value)
	}
}

impl From<Literal> for Expression {
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
	Star,
	Divide,
	Mod,
	Comma,
	Dot,
	Colon,
	SemiColon,

	Reference,

	Equals,
	NotEquals,
	Greater,
	GreaterOrEquals,
	Less,
	LessOrEquals,
	Assignment,
	ShiftLeft,
	ShiftRight,

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
	Const,
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
			"const" => Self::Const,
			"as" => Self::As,
			_ => return Err(())
		})
	}
}