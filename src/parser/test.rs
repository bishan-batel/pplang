use test_case::test_case;
use crate::parser;
use crate::parser::ast::variable::Type;
use crate::parser::context::TokenStream;
use crate::parser::lexer;
use crate::parser::pass::category;
use crate::parser::token::{Keyword, Literal, Operator, Parenthetical, Token};

#[test_case("function", Keyword::Function; "Function Keyword")]
#[test_case("return", Keyword::Return; "Return Keyword")]
#[test_case("break", Keyword::Break; "Break Keyword")]
#[test_case("while", Keyword::While; "While Keyword")]
#[test_case("for", Keyword::For; "For Keyword")]
#[test_case("let", Keyword::Let; "Let Keyword")]
#[test_case("var", Keyword::Var; "Var Keyword")]
#[test_case("in", Keyword::In; "In Keyword")]
fn keyword(contents: &str, keyword: Keyword) -> lexer::Result<()> {
	let contents = contents.to_string();
	let tokens = lexer::tokenize(contents)?;

	assert_eq!(tokens.len(), 1);
	assert_eq!(tokens[0].token, Token::Keyword(keyword));

	Ok(())
}


#[test_case("bruh moment", & ["bruh", "moment"]; "Multiple Identifiers")]
#[test_case("the2", & ["the2"]; "Identifier with numbers")]
#[test_case("wha_42_b muh", & ["wha_42_b", "muh"]; "Identifier with underscores")]
fn identifier(contents: &str, idents: &[&'static str]) -> lexer::Result<()> {
	let contents = contents.to_string();
	let tokens = lexer::tokenize(contents)?;

	assert_eq!(tokens.len(), idents.len());

	for (tt, ident) in tokens.into_iter().zip(idents.iter()) {
		assert_eq!(tt.token, Token::Identifier((*ident).into()));
	}

	Ok(())
}

#[test_case("42_2", & [422]; "Int Literal with Underscore")]
#[test_case("919", & [919]; "Int Literal")]
#[test_case("100_532_3", & [1_005_323]; "Int Literal with multiple underscores")]
#[test_case("48 597 816", & [48, 597, 816]; "Multiple Int Literals")]
#[test_case("000___420___69___", & [42069]; "Int Literal with shit ton of underscores")]
fn int_literals(contents: &str, idents: &[i64]) -> lexer::Result<()> {
	let contents = contents.to_string();
	let tokens = lexer::tokenize(contents)?;

	assert_eq!(tokens.len(), idents.len());
	for (tt, literal) in tokens.into_iter().zip(idents.iter()) {
		assert_eq!(tt.token, Token::Literal(Literal::Integer(*literal)));
	}

	Ok(())
}

#[test_case("584.38", & [584.38]; "Float Literal")]
#[test_case("42.", & [42.]; "Float Literal with dot suffix")]
#[test_case("100.__52", & [100.52]; "Float Literal with Underscores")]
#[test_case("72.78 735.53", & [72.78, 735.53]; "Multiple Float Literals")]
fn float_literals(contents: &str, idents: &[f64]) -> lexer::Result<()> {
	let contents = contents.to_string();
	let tokens = lexer::tokenize(contents)?;

	assert_eq!(tokens.len(), idents.len());
	for (tt, literal) in tokens.into_iter().zip(idents.iter()) {
		assert_eq!(tt.token, Token::Literal(Literal::Float(*literal)));
	}

	Ok(())
}

#[test_case(":", & [Operator::Colon]; "Colon Operator")]
#[test_case("+", & [Operator::Add]; "Add Operator")]
#[test_case("-", & [Operator::Minus]; "Minus Operator")]
#[test_case("*", & [Operator::Star]; "Multiply Operator")]
#[test_case("/", & [Operator::Divide]; "Divide Operator")]
#[test_case("%", & [Operator::Mod]; "Mod Operator")]
#[test_case(",", & [Operator::Comma]; "Comma Operator")]
#[test_case(".", & [Operator::Dot]; "Dot Operator")]
#[test_case("==", & [Operator::Equals]; "Equals Operator")]
#[test_case(">", & [Operator::Greater]; "Greater Operator")]
#[test_case(">=", & [Operator::GreaterOrEquals]; "GreaterOrEquals Operator")]
#[test_case("<", & [Operator::Less]; "Less Operator")]
#[test_case("<=", & [Operator::LessOrEquals]; "LessOrEquals Operator")]
#[test_case("=", & [Operator::Assignment]; "Assignment Operator")]
#[test_case("and", & [Operator::And]; "And Operator")]
#[test_case("or", & [Operator::Or]; "Or Operator")]
#[test_case("not", & [Operator::Not]; "Not Operator")]
#[test_case("xor", & [Operator::Xor]; "Xor Operator")]
#[test_case("->", & [Operator::ThinArrow]; "ThinArrow Operator")]
#[test_case("=>", & [Operator::Arrow]; "Arrow Operator")]
#[test_case("&", & [Operator::Reference]; "Reference Operator")]
#[test_case("+ - % > <= > and or -> => = > , . xor &", & [
Operator::Add,
Operator::Minus,
Operator::Mod,
Operator::Greater,
Operator::LessOrEquals,
Operator::Greater,
Operator::And,
Operator::Or,
Operator::ThinArrow,
Operator::Arrow,
Operator::Assignment,
Operator::Greater,
Operator::Comma,
Operator::Dot,
Operator::Xor,
Operator::Reference];
"All Operators")]
fn operators_simple(contents: &str, expected: &[Operator]) -> lexer::Result<()> {
	let contents = contents.to_string();
	let tokens = lexer::tokenize(contents)?;

	assert_eq!(tokens.len(), expected.len());
	for (tt, operator) in tokens.into_iter().zip(expected.iter()) {
		assert_eq!(tt.token, (*operator).into());
	}

	Ok(())
}

#[test_case(r#" "hello world" "#, & ["hello world"]; "String Literal")]
#[test_case(r#" "wha \\ moment ' \" " "\n""#, & [r#"wha \ moment ' " "#, "\n"]; "String Literal with Escape")]
#[test_case(r#" "\u0289 bruh" "#, & ["\u{289} bruh"]; "String Literal with unicode escape")]
fn string_literal(contents: &str, expected: &[&'static str]) -> lexer::Result<()> {
	let contents = contents.to_string();
	let tokens = lexer::tokenize(contents)?;

	assert_eq!(tokens.len(), expected.len());
	for (tt, operator) in tokens.into_iter().zip(expected.iter()) {
		assert_eq!(tt.token, Literal::String((*operator).into()).into());
	}

	Ok(())
}

#[test_case(r#" 'a' "#, & ['a']; "Char Literal")]
#[test_case(r#" '\u0050' '\n'"#, & ['\u{50}', '\n']; "Char literal with escape")]
fn char_literal(contents: &str, expected: &[char]) -> lexer::Result<()> {
	let contents = contents.to_string();
	let tokens = lexer::tokenize(contents)?;

	assert_eq!(tokens.len(), expected.len());
	for (tt, operator) in tokens.into_iter().zip(expected.iter()) {
		assert_eq!(tt.token, Literal::Character(*operator).into());
	}

	Ok(())
}

#[test]
fn program_is_even() -> lexer::Result<()> {
	const CONTENTS: &str = r#"
		function is_even(n: i32) {
			return n % 2 == 0
		}
 "#;

	let tokens = lexer::tokenize(CONTENTS.to_string())?;
	let expected = vec![
		Keyword::Function.into(),
		Token::Identifier("is_even".into()),
		Parenthetical::NormalOpen.into(),
		Token::Identifier("n".into()),
		Operator::Colon.into(),
		Token::Identifier("i32".into()),
		Parenthetical::NormalClose.into(),
		Parenthetical::CurlyOpen.into(),
		Keyword::Return.into(),
		Token::Identifier("n".into()),
		Operator::Mod.into(),
		Literal::Integer(2).into(),
		Operator::Equals.into(),
		Literal::Integer(0).into(),
		Parenthetical::CurlyClose.into(),
	];

	let tokens: Vec<_> = tokens.into_iter().map(|x| x.token).collect();

	assert_eq!(tokens, expected);

	Ok(())
}
