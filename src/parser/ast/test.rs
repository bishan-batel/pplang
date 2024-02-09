use crate::parser::ast::variable::Type;
use test_case::test_case;
use crate::parser::ast::function::FunctionSignature;

#[test_case(Type::Unit, "unit")]
#[test_case(Type::I64, "i64")]
#[test_case(Type::I32, "i32")]
#[test_case(Type::I8, "i8")]
#[test_case(Type::U64, "u64")]
#[test_case(Type::U32, "u32")]
#[test_case(Type::U8, "u8")]
#[test_case(Type::F32, "f32")]
#[test_case(Type::F64, "f64")]
#[test_case(Type::Unit.as_pointer(), "*unit")]
#[test_case(Type::U32.as_pointer().as_pointer(), "**u32")]
#[test_case(Type::F32.as_array(10), "[f32; 10]")]
#[test_case(Type::custom("BruhMoment".into()), "BruhMoment")]
#[test_case(Type::template("BruhMoment".into(), vec ! [Type::I64]), "BruhMoment<i64>")]
#[test_case(Type::template("BruhMoment".into(), vec ! [Type::I64, Type::F32.as_pointer()]), "BruhMoment<i64, *f32>")]
#[test_case(Type::U8.as_const(), "const u8")]
#[test_case(Type::U8.as_const().as_pointer(), "*const u8")]
#[test_case(FunctionSignature::new(vec ! [Type::F32], Type::Unit).into(), "function (f32) => unit")]
#[test_case(FunctionSignature::new(vec ! [Type::F32, Type::U8], Type::Unit).into(), "function (f32, u8) => unit")]
#[test_case(FunctionSignature::new(vec ! [Type::F32], Type::Unit).as_type().as_pointer().as_const(), "const *function (f32) => unit")]
fn ty_names(ty: Type, expected: &'static str) {
	assert_eq!(ty.name(), expected.to_string());
	drop(ty);
}