use crate::parser::LexerError;
use crate::parser::token::{Keyword, Literal, Operator, Parenthetical, Token};

// const fn is kinda the same thing as a 'constexpr function'
// where its code that , if possible, will run at compile time
// not like a macro but

pub type Result<T> = std::result::Result<T, LexerError>;

struct Lexer {
	pos: usize,
	contents: String,
	tokens: Vec<Token>,
}

impl Lexer {
	fn new(contents: String) -> Self {
		Self {
			pos: 0,
			tokens: vec![],
			contents,
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

	fn is_eof(&self) -> bool {
		self.curr().is_none()
	}

	fn tokenize(mut self) -> Result<Vec<Token>> {
		while !self.is_eof() {
			self.read_token()?;
		}

		Ok(self.tokens)
	}

	fn read_token(&mut self) -> Result<()> {
		const PASSES: &[fn(&mut Lexer) -> bool] = &[
			Lexer::whitespace,
			Lexer::operator_double,
			Lexer::parenthetical,
			Lexer::operator_simple,
			Lexer::number_literal,
			Lexer::string_literal,
			Lexer::identifier
		];

		for pass in PASSES {
			if pass(self) {
				return Ok(());
			}
		}

		Err(LexerError::UnexpectedChar(self.curr_or_whitespace()))
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

		self.tokens.push(if has_decimal {
			let num: f64 = num.parse().expect("Failed to build float string properly");
			Literal::Float(num)
		} else {
			let num: i64 = num.parse().expect("Failed to build int string properly");
			Literal::Integer(num)
		}.into());

		true
	}

	fn string_literal(&mut self) -> bool {
		if self.curr_or_whitespace() != '"' { return false; }

		let mut string = String::new();

		while !self.is_eof() && self.advance_or_whitespace() != '"' {
			string.push(self.curr_or_whitespace());
		}
		self.advance();

		self.tokens.push(Literal::String(string).into());

		true
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
		self.tokens.push(match identifier.as_str() {
			"and" => Operator::And.into(),
			"or" => Operator::Or.into(),
			"not" => Operator::Not.into(),
			"xor" => Operator::Xor.into(),

			// if not a special character, try to find a keyword, if all else fails
			// add a new identifier token
			_ => Keyword::try_from(identifier.as_str())
				.map_or(Token::Identifier(identifier), Token::Keyword)
		});

		true
	}

	fn parenthetical(&mut self) -> bool {
		self.tokens.push(match self.curr_or_whitespace() {
			'(' => Parenthetical::NormalOpen.into(),
			')' => Parenthetical::NormalClose.into(),
			'[' => Parenthetical::BracketOpen.into(),
			']' => Parenthetical::BracketClose.into(),
			'{' => Parenthetical::CurlyOpen.into(),
			'}' => Parenthetical::CurlyClose.into(),
			_ => return false
		});
		self.advance();
		true
	}
	fn operator_simple(&mut self) -> bool {
		self.tokens.push(Token::Operator(match self.curr_or_whitespace() {
			'+' => Operator::Add,
			'-' => Operator::Minus,
			'*' => Operator::Multiply,
			'/' => Operator::Divide,
			'%' => Operator::Mod,
			',' => Operator::Comma,
			'.' => Operator::Dot,
			'>' => Operator::Greater,
			'<' => Operator::Less,
			'=' => Operator::Assignment,
			_ => return false
		}));
		self.advance();
		true
	}

	fn operator_double(&mut self) -> bool {
		let slice = self.slice(2);

		if slice.len() != 2 { return false; }

		self.tokens.push(Token::Operator(match slice {
			"==" => Operator::Equals,
			">=" => Operator::GreaterOrEquals,
			"<=" => Operator::LessOrEquals,
			"->" => Operator::ThinArrow,
			"=>" => Operator::Arrow,
			_ => return false
		}));

		self.advance();
		self.advance();

		true
	}
}

pub fn tokenize(contents: String) -> Result<Vec<Token>> {
	Lexer::new(contents).tokenize()
}