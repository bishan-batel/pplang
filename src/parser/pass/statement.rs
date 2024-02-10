use crate::parser;
use crate::parser::ast::{Statement, TopLevelStatement};
use crate::parser::ast::variable::Variable;
use crate::parser::context::{PassAttempt, TokenStream};
use crate::parser::Error;
use crate::parser::pass::{category, expression};
use crate::parser::token::{Keyword, Operator};

type PassResult = parser::Result<Option<Statement>>;

const STATEMENT_PASSES: &[PassAttempt<Statement>] = &[
	let_var_pass,
	var_pass,
	return_pass,
	expression_pass
];

fn let_var_pass(stream: &mut TokenStream) -> PassResult {
	if stream.try_consume(Keyword::Let).is_none() {
		return Ok(None);
	}

	let var = category::consume_variable(stream)?;
	let var = Variable::new(var.get_name().clone(), var.get_type().as_const());

	stream.consume(Operator::Assignment);

	let initialisation = Some(expression::consume(stream)?);

	Ok(Statement::Declaration {
		var,
		initialisation,
	}.into())
}

fn var_pass(stream: &mut TokenStream) -> PassResult {
	if stream.try_consume(Keyword::Var).is_none() { return Ok(None); }

	let var = category::consume_variable(stream)?;

	let initialisation = if stream.try_consume(Operator::Assignment).is_some() {
		Some(expression::consume(stream)?)
	} else {
		None
	};


	Ok(Statement::Declaration {
		var,
		initialisation,
	}.into())
}

fn return_pass(stream: &mut TokenStream) -> PassResult {
	if stream.try_consume(Keyword::Return).is_none() {
		return Ok(None);
	}
	Ok(Some(Statement::Return(expression::consume(stream)?)))
}

fn expression_pass(stream: &mut TokenStream) -> PassResult {
	Ok(Some(expression::consume(stream)?.into()))
}

pub fn consume(stream: &mut TokenStream) -> parser::Result<Statement> {
	stream.use_passes(STATEMENT_PASSES)?.map_or_else(
		|| Error::unexpected_token(stream.curr_token().clone()).into(),
		Ok,
	)
}