use crate::parser::ast::function::{Function, FunctionSignature};
use crate::parser::ast::{Expression, Statement, TopLevelStatement};
use crate::parser::ast::variable::Type;
use crate::parser::context::TokenStream;
use crate::parser::pass::{category, expression};
use crate::parser::Result;
use crate::parser::token::{Keyword, Literal, Operator, Parenthetical, Token};


pub fn function_pass(stream: &mut TokenStream) -> Result<Option<TopLevelStatement>> {
	if !stream.is_curr(Keyword::Function) {
		return Ok(None);
	}

	stream.consume(Keyword::Function)?;

	let ident = stream.consume_identifier()?;

	stream.consume(Parenthetical::NormalOpen);


	let mut args = vec![];

	while !stream.is_curr(Parenthetical::NormalClose) {
		args.push(category::consume_variable(stream)?);

		if stream.is_curr(Operator::Comma) {
			stream.next();
			continue;
		}
		break;
	}
	stream.consume(Parenthetical::NormalClose);

	let returns = if stream.is_curr(Operator::ThinArrow) {
		stream.next();
		category::consume_type(stream)?
	} else {
		Type::Unit
	};

	let signature = FunctionSignature::new_named(args, returns);

	let body = Box::new(if let Some(scope) = expression::scope_pass(stream)? {
		scope
	} else {
		stream.consume(Operator::Arrow)?;
		expression::consume(stream)?
	});


	Ok(Some(TopLevelStatement::Function {
		ident,
		function: Function { signature, body },
	}))
}