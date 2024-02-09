use crate::parser::token::{Token, TracedToken};

pub mod token;
pub mod ast;
mod lexer;

#[cfg(test)]
#[allow(clippy::needless_raw_strings, clippy::needless_raw_string_hashes)]
mod test;
mod parser;


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

pub fn parse(contents: String) -> Result<Vec<TracedToken>, Error> {
	Ok(lexer::tokenize(contents)?)
}