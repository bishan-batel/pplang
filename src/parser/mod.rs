use crate::parser::token::Token;

pub mod token;
mod lexer;

#[cfg(test)]
#[allow(clippy::needless_raw_strings, clippy::needless_raw_string_hashes)]
mod tests;


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
}

pub fn parse(contents: String) -> Result<Vec<Token>, Error> {
	Ok(lexer::tokenize(contents)?)
}