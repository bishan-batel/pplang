#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Binary {
	Add,
	Minus,
	Multiply,
	Divide,
	Mod,

	Equals,
	Greater,
	GreaterOrEquals,
	Less,
	LessOrEquals,
	Assignment,

	And,
	Or,
	Xor,

	Dot,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Unary {
	Negate,
	Not,
	Reference,
	Dereference,
}