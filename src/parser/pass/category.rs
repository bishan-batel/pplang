use crate::parser::ast::variable::{Type, Variable};
use crate::parser::context::TokenStream;
use crate::parser::Error::UnexpectedToken;
use crate::parser::{Error, Result};
use crate::parser::token::{Keyword, Literal, Operator, Parenthetical, Token};

pub fn consume_variable(ctx: &mut TokenStream) -> Result<Variable> {
	let ident = ctx.consume_identifier()?;
	ctx.consume(Operator::Colon)?;
	Ok(Variable::new(ident, consume_type(ctx)?))
}

pub fn consume_type(ctx: &mut TokenStream) -> Result<Type> {
	match ctx.take_curr() {
		Token::Keyword(Keyword::Const) => Ok(consume_type(ctx)?.as_const()),
		Token::Operator(Operator::Star) => Ok(consume_type(ctx)?.as_pointer()),

		Token::Parenthetical(Parenthetical::BracketOpen) => {
			let ty = consume_type(ctx)?;
			ctx.consume(Operator::SemiColon);

			let length = match ctx.take_curr() {
				Token::Literal(Literal::Integer(size)) => size,
				given => return Error::expected_token(given, Literal::Integer(0)).into()
			};

			let length = usize::try_from(length).map_err(|_| Error::InvalidArraySize(length))?;

			ctx.consume(Parenthetical::BracketClose)?;
			Ok(Type::Array {
				ty: Box::new(ty),
				length,
			})
		}

		Token::Identifier(ident) => {
			let mut ty = Type::from(ident);

			match &mut ty {
				Type::Custom { name, template_args } => {
					if ctx.is_curr(Operator::Less) {
						ctx.next();

						while !ctx.is_curr(Operator::Greater) {
							template_args.push(consume_type(ctx)?);
							if ctx.consume(Operator::Comma).is_err() { break; }
						}

						ctx.consume(Operator::Greater)?;
					}
					Ok(ty)
				}
				_ => Ok(ty)
			}
		}
		tok => Error::unexpected_token(tok).into()
	}
}