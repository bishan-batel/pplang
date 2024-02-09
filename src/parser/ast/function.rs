use std::fmt::{Display, Formatter, Write};
use crate::parser::ast::Expression;
use crate::parser::ast::variable::{Type, Variable};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct FunctionSignature {
	args: Vec<Variable>,
	returns: Type,
}

impl FunctionSignature {
	pub const fn new_named(args: Vec<Variable>, returns: Type) -> Self {
		Self { args, returns }
	}

	pub fn new(args: Vec<Type>, returns: Type) -> Self {
		Self::new_named(args.into_iter().map(|x| { Variable::new("".into(), x) }).collect(), returns)
	}

	pub fn as_type(&self) -> Type {
		self.clone().into()
	}
}

impl From<FunctionSignature> for Type {
	fn from(value: FunctionSignature) -> Self {
		Self::Function(Box::new(value))
	}
}

impl Display for FunctionSignature {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_str("function (")?;
		let args = &self.args;
		if !args.is_empty() {
			f.write_fmt(format_args!("{}", args[0].get_type()))?;
		}

		let args = &self.args[1..];
		for arg in args {
			f.write_fmt(format_args!(", {}", arg.get_type()))?;
		}

		f.write_fmt(format_args!(") => {}", self.returns))?;

		Ok(())
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
	signature: FunctionSignature,
	body: Box<Expression>,
}