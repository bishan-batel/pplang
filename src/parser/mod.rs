use crate::parser::ast::TopLevelStatement;
use crate::parser::token::{Token, TracedToken, TracedTokenList};

pub mod token;
pub mod ast;
mod lexer;

#[cfg(test)]
#[allow(clippy::needless_raw_strings, clippy::needless_raw_string_hashes)]
mod test;
mod context;
mod pass;


pub type Result<T = ()> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error(transparent)]
	Lexer(#[from] LexerError),

	#[error("Expected token {expected:?}, given {given:?}")]
	ExpectedToken {
		expected: Token,
		given: Token,
	},

	#[error("Unexpected token {0:#?}")]
	UnexpectedToken(Token),

	#[error("Cannot have an array of size {0}")]
	InvalidArraySize(i64),
}

impl Error {
	pub fn unexpected_token(given: impl Into<Token>) -> Error {
		Self::UnexpectedToken(given.into())
	}

	pub fn expected_token(given: impl Into<Token>, expected: impl Into<Token>) -> Error {
		Self::ExpectedToken {
			expected: expected.into(),
			given: given.into(),
		}
	}
}

impl<T> From<Error> for Result<T> {
	fn from(value: Error) -> Self {
		Err(value)
	}
}

#[derive(Debug, thiserror::Error)]
pub enum LexerError {
	#[error("Unexpected character {0:#?}")]
	UnexpectedChar(char),

	#[error("Unterminated string literal")]
	UnterminatedLiteral(String),

	#[error("Unable to parse string literal as json")]
	#[from(serde_json::Error)]
	InvalidLiteral(String, serde_json::Error),

	#[error("Empty character literal")]
	EmptyLiteral,

	#[error("Character literals cannot have more than one character")]
	LongCharacterLiteral(String),
}

pub fn parse(contents: String) -> Result<Vec<TopLevelStatement>> {
	context::TokenStream::parse(lexer::tokenize(contents)?)
}