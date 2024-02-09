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
		signature: Function,
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
	Literal(Literal),
	FunctionCall {
		ident: Identifier,
		arguments: Vec<Expression>,
	},
	VariableReference(Identifier),
	Lambda(Function),
	Scope {
		expressions: Vec<Expression>
	},
	ArrayAccess {
		expr: Box<Expression>,
		index: Box<Expression>,
	},
}


impl From<Literal> for Expression {
	fn from(value: Literal) -> Self {
		Self::Literal(value)
	}
}
