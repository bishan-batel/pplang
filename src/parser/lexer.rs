use serde_json::Error;
use crate::parser::LexerError;
use crate::parser::token::{FilePos, Keyword, Literal, Operator, Parenthetical, Token, TracedToken, Trace, TracedTokenList};

// const fn is kinda the same thing as a 'constexpr function'
// where its code that , if possible, will run at compile time
// not like a macro but

pub type Result<T> = std::result::Result<T, LexerError>;
type LexerPass = fn(&mut Lexer) -> bool;

struct Lexer {
	pos: usize,
	contents: String,
	tokens: TracedTokenList,
	last_token_position: FilePos,
}

impl Lexer {
	fn new(contents: String) -> Self {
		Self {
			pos: 0,
			tokens: vec![],
			contents,
			last_token_position: FilePos::beginning(),
		}
	}

	fn slice(&self, len: usize) -> &str {
		&self.contents[self.pos..(self.pos + len).min(self.contents.len())]
	}

	fn advance(&mut self) -> Option<char> {
		self.pos += 1;
		self.curr()
	}

	fn curr(&self) -> Option<char> {
		self.contents.chars().nth(self.pos)
	}

	fn advance_or_whitespace(&mut self) -> char {
		self.advance().unwrap_or(' ')
	}

	fn curr_or_whitespace(&self) -> char {
		self.curr().unwrap_or(' ')
	}

	fn push_token(&mut self, token: impl Into<Token>) {
		let contents_to_now = &self.contents[0..self.pos.min(self.contents.len())];

		let end_pos = FilePos {
			row: contents_to_now.chars().filter(|x| *x == '\n').count(),
			column: contents_to_now.chars().fold(1, |x, c| {
				match c {
					'\n' => 1,
					_ => x + 1
				}
			}),
		};

		self.tokens.push(TracedToken {
			token: token.into(),
			trace: Trace {
				source_file: "".to_string(),
				begin_pos: self.last_token_position,
				end_pos,
			},
		});
		self.last_token_position = end_pos;
	}

	fn is_eof(&self) -> bool {
		self.curr().is_none()
	}

	fn tokenize(mut self) -> Result<TracedTokenList> {
		while !self.is_eof() {
			self.read_token()?;
		}

		Ok(self.tokens)
	}

	fn read_token(&mut self) -> Result<()> {
		const PASSES: &[LexerPass] = &[
			Lexer::whitespace,
			Lexer::operator_double,
			Lexer::parenthetical,
			Lexer::operator_simple,
			Lexer::number_literal,
			Lexer::identifier
		];

		// manual string pass as it is the only one that can fail
		if self.string_literal()? { return Ok(()); }
		if self.char_literal()? { return Ok(()); }

		if PASSES.iter().any(|x| x(self)) {
			Ok(())
		} else {
			Err(LexerError::UnexpectedChar(self.curr_or_whitespace()))
		}
		// for pass in PASSES {
		// 	if pass(self) {
		// 		return Ok(());
		// 	}
		// }
		// Err(LexerError::UnexpectedChar(self.curr_or_whitespace()))
	}

	fn whitespace(&mut self) -> bool {
		self.curr().map_or(false, |curr| if curr.is_whitespace() {
			self.advance();
			true
		} else {
			false
		},
		)
	}

	fn number_literal(&mut self) -> bool {
		if !self.curr_or_whitespace().is_ascii_digit() {
			return false;
		}

		let mut num: String = self.curr_or_whitespace().to_string();
		let mut has_decimal = false;

		loop {
			let curr = self.advance_or_whitespace();

			if !(curr == '_' || (!has_decimal && curr == '.') || curr.is_ascii_digit()) {
				break;
			}

			num.push(curr);
			has_decimal |= curr == '.';
		}

		// Sanitize _ bc rust parse shits itself
		let num = num.replace('_', "");

		self.push_token(if has_decimal {
			let num: f64 = num.parse().expect("Failed to build float string properly");
			Literal::Float(num)
		} else {
			let num: i64 = num.parse().expect("Failed to build int string properly");
			Literal::Integer(num)
		});

		true
	}

	fn char_literal(&mut self) -> Result<bool> {
		if self.curr_or_whitespace() != '\'' { return Ok(false); }

		let mut literal = String::new();

		while self.advance_or_whitespace() != '\'' {
			if self.is_eof() { return Err(LexerError::UnterminatedLiteral(literal)); }

			let curr = self.curr_or_whitespace();
			literal.push(curr);
			if curr == '\\' {
				if let Some(curr) = self.advance() {
					literal.push(curr);
				}
			}
		}
		self.advance();

		let char_literal: String = serde_json::from_str(&format!(r#""{literal}""#))
			.map_err(|x| LexerError::InvalidLiteral(literal, x))?;

		if char_literal.len() > 1 {
			return Err(LexerError::LongCharacterLiteral(char_literal));
		}

		let char_literal = char_literal.chars().nth(0).ok_or(LexerError::EmptyLiteral)?;

		self.push_token(Literal::Character(char_literal));
		Ok(true)
	}

	fn string_literal(&mut self) -> Result<bool> {
		if self.curr_or_whitespace() != '"' { return Ok(false); }

		let mut string = String::new();

		while self.advance_or_whitespace() != '"' {
			if self.is_eof() { return Err(LexerError::UnterminatedLiteral(string)); }

			let curr = self.curr_or_whitespace();
			string.push(curr);
			if curr == '\\' {
				if let Some(curr) = self.advance() {
					string.push(curr);
				}
			}
		}
		self.advance();


		let string = serde_json::from_str(&format!(r#""{string}""#)).map_err(|x| LexerError::InvalidLiteral(string, x))?;

		self.push_token(Literal::String(string));
		Ok(true)
	}

	fn identifier(&mut self) -> bool {
		let is_valid_char = |x: char|
			x.is_ascii_alphanumeric() || x.is_ascii_digit() || x == '_';

		if !is_valid_char(self.curr_or_whitespace()) {
			return false;
		}

		let mut identifier = self.curr_or_whitespace().to_string();

		while is_valid_char(self.advance_or_whitespace()) {
			identifier.push(self.curr_or_whitespace());
		}

		// Operator check, for operator keywords like 'and'
		self.push_token(match identifier.as_str() {
			"and" => Operator::And.into(),
			"or" => Operator::Or.into(),
			"not" => Operator::Not.into(),
			"xor" => Operator::Xor.into(),
			"mod" => Operator::Mod.into(),

			"no_cap" | "true" => Literal::Bool(true).into(),
			"cap" | "false" => Literal::Bool(false).into(),

			// if not a special character, try to find a keyword, if all else fails
			// add a new identifier token
			_ => Keyword::try_from(identifier.as_str())
				.map_or(Token::Identifier(identifier.into()), Token::Keyword)
		});

		true
	}

	fn parenthetical(&mut self) -> bool {
		self.push_token(match self.curr_or_whitespace() {
			'(' => Parenthetical::NormalOpen,
			')' => Parenthetical::NormalClose,
			'[' => Parenthetical::BracketOpen,
			']' => Parenthetical::BracketClose,
			'{' => Parenthetical::CurlyOpen,
			'}' => Parenthetical::CurlyClose,
			_ => return false
		});
		self.advance();
		true
	}

	fn operator_simple(&mut self) -> bool {
		self.push_token(match self.curr_or_whitespace() {
			'+' => Operator::Add,
			'-' => Operator::Minus,
			'*' => Operator::Star,
			'/' => Operator::Divide,
			'%' => Operator::Mod,
			',' => Operator::Comma,
			'.' => Operator::Dot,
			':' => Operator::Colon,
			';' => Operator::SemiColon,
			'>' => Operator::Greater,
			'<' => Operator::Less,
			'=' => Operator::Assignment,
			'&' => Operator::Reference,
			_ => return false
		});
		self.advance();
		true
	}

	fn operator_double(&mut self) -> bool {
		let slice = self.slice(2);

		if slice.len() != 2 { return false; }

		self.push_token(match slice {
			"==" => Operator::Equals,
			"!=" => Operator::NotEquals,
			">=" => Operator::GreaterOrEquals,
			"<=" => Operator::LessOrEquals,
			"->" => Operator::ThinArrow,
			"=>" => Operator::Arrow,
			"<<" => Operator::ShiftLeft,
			">>" => Operator::ShiftRight,
			_ => return false
		});

		self.advance();
		self.advance();

		true
	}
}

pub fn tokenize(contents: String) -> Result<TracedTokenList> {
	Lexer::new(contents).tokenize()
}