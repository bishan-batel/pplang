use const_panic::fmt::IsLast::No;
use crate::parser;
use crate::parser::ast::{Expression, operator};
use crate::parser::{context, Error};
use crate::parser::context::{TokenStream, PassAttempt};
use crate::parser::Error::UnexpectedToken;
use crate::parser::pass::{expression, statement};
use crate::parser::token::{Literal, Operator, Parenthetical, Token};


const BINARY_ORDER_OF_OPERATIONS: &[&[operator::Binary]] = {
	use operator::Binary as B;
	&[
		&[B::Dot],
		&[B::Multiply, B::Divide, B::Mod],
		&[B::Add, B::Minus],
		&[B::ShiftLeft, B::ShiftRight],
		&[B::Less, B::LessOrEquals, B::Greater, B::GreaterOrEquals],
		&[B::Equals, B::NotEquals],
		&[B::BitAnd],
		&[B::BitXor],
		&[B::BitOr],
		&[B::And],
		&[B::Or],
		&[B::Assignment],
	]
};

const UNARY_PASSES: &[PassAttempt<Expression>] = &[
	identifier_reference_pass,
	scope_pass,
	literal_pass,
	unary_pass,
	parenthesis_pass,
];

const POSTFIX_PASS: &[PassAttempt<Expression>] = &[];

type ExpressionResult = parser::Result<Expression>;
type ExpressionPassResult = parser::Result<Option<Expression>>;


/// Consumes a unary 'atom', eg either an expression that cannot be broken down further -
/// or a composite such as a function call, scope, or control flow block
pub fn consume_atom(stream: &mut TokenStream) -> ExpressionResult {
	stream.use_passes(UNARY_PASSES)?.map_or_else(
		|| Error::unexpected_token(stream.curr_token().clone()).into(),
		Ok,
	)
}

pub fn unary_pass(stream: &mut TokenStream) -> ExpressionPassResult {
	Ok(if let Ok(operator) = stream.curr_token().try_into() {
		Some(Expression::Unary {
			operator,
			expr: Box::new(consume(stream)?),
		})
	} else {
		None
	})
}

#[allow(clippy::unnecessary_wraps)]
pub fn literal_pass(stream: &mut TokenStream) -> ExpressionPassResult {
	Ok(if let Token::Literal(literal) = stream.curr_token() {
		let literal = literal.clone();
		stream.next();
		Some(literal.into())
	} else {
		None
	})
}

pub fn parenthesis_pass(stream: &mut TokenStream) -> ExpressionPassResult {
	if stream.try_consume(Parenthetical::NormalOpen).is_none() { return Ok(None); }

	if stream.try_consume(Parenthetical::NormalClose).is_some() {
		return Ok(Some(Literal::Unit.into()));
	}

	let expr = consume(stream)?;
	stream.consume(Parenthetical::NormalClose)?;

	Ok(Some(expr))
}

pub fn scope_pass(stream: &mut TokenStream) -> ExpressionPassResult {
	if stream.try_consume(Parenthetical::CurlyOpen).is_none() { return Ok(None); }

	let mut body = vec![];
	while stream.try_consume(Parenthetical::CurlyClose).is_none() {
		body.push(statement::consume(stream)?);
	}

	Ok(Expression::Scope(body).into())
}

pub fn identifier_reference_pass(stream: &mut TokenStream) -> ExpressionPassResult {
	match stream.curr_token() {
		Token::Identifier(_) => stream.consume_identifier().map(|x| Some(x.into())),
		_ => Ok(None)
	}
}

/// Consumes a binary expression from a token screen
/// (along with a number indicating the order of operation level)
pub fn consume_binary(stream: &mut TokenStream, operation_level: usize) -> ExpressionResult {
	if operation_level == 0 {
		return consume_atom(stream);
	}

	let operators = BINARY_ORDER_OF_OPERATIONS[operation_level - 1];

	let mut lhs = consume_binary(stream, operation_level - 1)?;

	while let Token::Operator(op) = stream.curr_token() {
		if let Ok(op) = (*op).try_into() {
			if operators.contains(&op) {
				stream.next();
				lhs = Expression::Binary {
					lhs: lhs.into(),
					operator: op,
					rhs: consume_binary(stream, operation_level - 1)?.into(),
				};
				continue;
			}
		}
		break;
	}

	Ok(lhs)
}


/// Consumes an expression from a token stream
pub fn consume(stream: &mut TokenStream) -> ExpressionResult {
	consume_binary(stream, BINARY_ORDER_OF_OPERATIONS.len())
}