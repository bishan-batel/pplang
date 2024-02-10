#![allow(clippy::unwrap_used, clippy::needless_raw_string_hashes)]

use crate::parser;
use crate::parser::ast::variable::{Identifier, Type, Variable};
use crate::parser::lexer;
use crate::parser::pass::{category, expression, statement};
use test_case::{test_case, test_matrix};
use crate::parser::ast::{Expression, Statement, TopLevelStatement};
use crate::parser::ast::function::{Function, FunctionSignature};
use crate::parser::ast::operator::Binary;
use crate::parser::context::TokenStream;
use crate::parser::token::Literal;

#[test_case("unit", & Type::Unit)]
#[test_case("i64", & Type::I64)]
#[test_case("i32", & Type::I32)]
#[test_case("i8", & Type::I8)]
#[test_case("u64", & Type::U64)]
#[test_case("u32", & Type::U32)]
#[test_case("u8", & Type::U8)]
#[test_case("f32", & Type::F32)]
#[test_case("f64", & Type::F64)]
#[test_case("usize", & Type::USize)]
#[test_case("BruhMoment", & Type::custom("BruhMoment"))]
#[test_case("Vec<i32>", & Type::template("Vec", vec ! [Type::I32] ))]
#[test_case("Vec<i32, f32>", & Type::template("Vec", vec ! [Type::I32, Type::F32] ))]
#[test_case("*i32", & Type::I32.as_pointer())]
#[test_case("***i32", & Type::I32.as_pointer().as_pointer().as_pointer())]
#[test_case("const *i32", & Type::I32.as_pointer().as_const())]
#[test_case("* const i32", & Type::I32.as_const().as_pointer())]
#[test_case("const const const const i32", & Type::I32.as_const())]
fn consume_type(source: &'static str, ty: &Type) {
	let mut ctx = lexer::tokenize(source.into()).unwrap().into();
	assert_eq!(&category::consume_type(&mut ctx).unwrap(), ty);
}

#[test]
fn consume_expression() {
	let t = |s: &'static str|
		expression::consume(&mut TokenStream::from(lexer::tokenize(s.into()).unwrap())).unwrap();

	assert_eq!(t("wha "), "wha".into());
	assert_eq!(t("a*b"), Expression::Binary {
		lhs: Box::new("a".into()),
		operator: Binary::Multiply,
		rhs: Box::new("b".into()),
	});
	assert_eq!(t("a*b + c"), Expression::Binary {
		lhs: Expression::Binary {
			lhs: Box::new("a".into()),
			operator: Binary::Multiply,
			rhs: Box::new("b".into()),
		}.into(),
		operator: Binary::Add,
		rhs: Box::new("c".into()),
	});
	assert_eq!(t("4 + 2"), Expression::Binary {
		lhs: Box::new(Literal::Integer(4).into()),
		operator: Binary::Add,
		rhs: Box::new(Literal::Integer(2).into()),
	});
}

#[test]
fn consume_statement() {
	let t = |s: &'static str| statement::consume(&mut lexer::tokenize(s.into()).unwrap().into()).unwrap();

	assert_eq!(t("var a: i32"), Statement::Declaration {
		var: Variable::new("a", Type::I32),
		initialisation: None,
	});

	assert_eq!(t("var a: i32 = bruh"), Statement::Declaration {
		var: Variable::new("a", Type::I32),
		initialisation: Some(Expression::ObjectReference("bruh".into())),
	});

	assert_eq!(t("let a: i32 = bruh"), Statement::Declaration {
		var: Variable::new("a", Type::I32.as_const()),
		initialisation: Some(Expression::ObjectReference("bruh".into())),
	});

	//

	assert_eq!(t("let a: *const i32 = bruh"), Statement::Declaration {
		var: Variable::new("a", Type::I32.as_const().as_pointer().as_const()),
		initialisation: Some(Expression::ObjectReference("bruh".into())),
	});
}

#[test]
fn consume_top_level() {
	let t = |s: &'static str| TokenStream::parse(lexer::tokenize(s.into()).unwrap()).unwrap();

	{
		let what_unit = vec![TopLevelStatement::Function {
			ident: "what".into(),
			function: Function {
				signature: FunctionSignature::new_named(vec![], Type::Unit),
				body: Box::new(Expression::Scope(vec![])),
			},
		}];

		assert_eq!(t("function what() {}"), what_unit);
		assert_eq!(t("function what() -> unit {}"), what_unit);

		assert_eq!(t(r#"function huh(x: f32, y: f32) {}"#), vec![TopLevelStatement::Function {
			ident: "huh".into(),
			function: Function {
				signature: FunctionSignature::new_named(vec![
					Variable::new("x", Type::F32),
					Variable::new("y", Type::F32),
				], Type::Unit),
				body: Box::new(Expression::Scope(vec![])),
			},
		}]);

		assert_eq!(t(r#"function huh() -> Bruh {
			{}
		}"#), vec![TopLevelStatement::Function {
			ident: "huh".into(),
			function: Function {
				signature: FunctionSignature::new_named(vec![], Type::custom("Bruh")),
				body: Box::new(Expression::Scope(vec![Expression::Scope(vec![]).into()])),
			},
		}]);

		assert_eq!(t(r#"function man() -> Bruh {
			man
		}"#), vec![TopLevelStatement::Function {
			ident: "man".into(),
			function: Function {
				signature: FunctionSignature::new_named(vec![], Type::custom("Bruh")),
				body: Box::new(Expression::Scope(vec![Expression::ObjectReference("man".into()).into()])),
			},
		}]);

		assert_eq!(t(r#"function lambda() -> f32 => bruh"#), vec![TopLevelStatement::Function {
			ident: "lambda".into(),
			function: Function {
				signature: FunctionSignature::new_named(vec![], Type::F32),
				body: Box::new(Expression::ObjectReference("bruh".into())),
			},
		}]);
	}
}
