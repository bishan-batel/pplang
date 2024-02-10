use crate::parser::ast::function::{Function, FunctionSignature};
use crate::parser::ast::variable::{Identifier, Type, Variable};
use crate::parser::token::Literal;

pub mod operator;
pub mod variable;
pub mod function;
#[cfg(test)]
mod test;

#[derive(Debug, Clone, PartialEq)]
pub enum TopLevelStatement {
	Function {
		ident: Identifier,
		function: Function,
	},
	GlobalVariable {
		global: Variable
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
	Expression(Expression),
	Declaration {
		var: Variable,
		initialisation: Option<Expression>,
	},
	Return(Expression),
}

impl From<Expression> for Statement {
	fn from(value: Expression) -> Self {
		Self::Expression(value)
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
	Binary {
		lhs: Box<Expression>,
		operator: operator::Binary,
		rhs: Box<Expression>,
	},
	Unary {
		operator: operator::Unary,
		expr: Box<Expression>,
	},
	Cast {
		from: Box<Expression>,
		to: Type,
	},
	FunctionCall {
		function: Box<Expression>,
		arguments: Vec<Expression>,
	},
	ObjectReference(Identifier),
	ArrayAccess {
		expr: Box<Expression>,
		index: Box<Expression>,
	},
	Literal(Literal),
	Lambda(Function),
	Scope(Vec<Statement>),
}

impl From<Identifier> for Expression {
	fn from(value: Identifier) -> Self {
		Self::ObjectReference(value)
	}
}

impl From<Function> for Expression {
	fn from(value: Function) -> Self {
		Self::Lambda(value)
	}
}


