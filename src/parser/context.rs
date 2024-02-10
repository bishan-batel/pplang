use crate::parser::ast::TopLevelStatement;
use crate::parser::ast::variable::Identifier;
use crate::parser::{Error};
use crate::parser::Error::UnexpectedToken;
use crate::parser::pass::top_level;
use crate::parser::token::{Token, TracedToken, TracedTokenList};
use crate::parser::Result;

#[derive(Debug)]
pub struct TokenStream {
	tokens: TracedTokenList,
	pos: usize,
}

pub const TOP_LEVEL_PASSES: &[PassAttempt<TopLevelStatement>] = &[
	top_level::function_pass,
];

pub type Pass<T = ()> = fn(ctx: &mut TokenStream) -> Result<T>;
pub type PassAttempt<T> = Pass<Option<T>>;

impl From<TracedTokenList> for TokenStream {
	fn from(tokens: TracedTokenList) -> Self {
		Self {
			tokens,
			pos: 0,
		}
	}
}

impl TokenStream {
	pub fn use_passes<T>(&mut self, passes: &[PassAttempt<T>]) -> Result<Option<T>> {
		for pass in passes {
			if let Some(res) = pass(self)? {
				return Ok(res.into());
			}
		}
		Ok(None)
	}

	pub fn parse(tokens: TracedTokenList) -> Result<Vec<TopLevelStatement>> {
		let mut parser = Self {
			tokens,
			pos: 0,
		};

		let statements = {
			let mut statements = vec![];
			while let Some(statement) = parser.use_passes(TOP_LEVEL_PASSES)? {
				statements.push(statement);
			};
			statements
		};

		for pass in TOP_LEVEL_PASSES {
			pass(&mut parser)?;
		}

		Ok(statements)
	}

	pub fn is_eof(&self) -> bool {
		self.is_curr(Token::EOF)
	}

	pub fn curr_token(&self) -> &Token {
		&self.tokens[self.pos].token
	}

	pub fn next(&mut self) -> &Token {
		self.pos = (self.pos + 1).min(self.tokens.len() - 1);
		self.curr_token()
	}

	pub fn is_curr(&self, token: impl Into<Token>) -> bool {
		self.curr_token() == &token.into()
	}
	pub fn not_curr(&self, token: impl Into<Token>) -> bool { !self.is_curr(token) }

	pub fn try_consume(&mut self, token: impl Into<Token>) -> Option<Token> {
		if self.is_curr(token) {
			let tok = Some(self.curr_token().clone());
			self.next();
			tok
		} else {
			None
		}
	}

	pub fn take_curr(&mut self) -> Token {
		let curr = self.curr_token().clone();
		self.next();
		curr
	}

	pub fn consume(&mut self, token: impl Into<Token>) -> Result<Token> {
		let expected = token.into();
		let given = self.take_curr();
		if given == expected { Ok(given) } else { Err(Error::ExpectedToken { expected, given }) }
	}

	pub fn consume_identifier(&mut self) -> Result<Identifier> {
		match self.take_curr() {
			Token::Identifier(ident) => Ok(ident),
			token => Err(Error::ExpectedToken {
				expected: Token::Identifier("".into()),
				given: token,
			})
		}
	}
}
