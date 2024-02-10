use std::fmt;
use std::fmt::Formatter;
use crate::parser::ast::Expression;
use crate::parser::ast::function::FunctionSignature;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Identifier(pub String);

impl From<String> for Identifier {
	fn from(value: String) -> Self {
		Self(value)
	}
}

impl From<&'static str> for Identifier {
	fn from(value: &'static str) -> Self {
		Self::from(value.to_string())
	}
}

impl From<Identifier> for String {
	fn from(value: Identifier) -> Self {
		value.0
	}
}

impl From<&'static str> for Expression {
	fn from(value: &'static str) -> Self {
		Identifier(value.into()).into()
	}
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Type {
	Unit,
	I64,
	I32,
	I8,
	U64,
	U32,
	U8,
	F32,
	F64,
	USize,
	Function(Box<FunctionSignature>),
	Pointer(Box<Type>),
	Const(Box<Type>),
	Array {
		ty: Box<Type>,
		length: usize,
	},
	Custom {
		name: Identifier,
		template_args: Vec<Type>,
	},
}

impl fmt::Display for Type {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		f.write_str(self.name().as_str())
	}
}

impl Type {
	pub fn custom(name: impl Into<Identifier>) -> Self {
		Self::template(name.into(), vec![])
	}
	pub fn template(name: impl Into<Identifier>, template_args: Vec<Self>) -> Self {
		Self::Custom { name: name.into(), template_args }
	}

	pub fn as_array(&self, length: usize) -> Self {
		Self::Array {
			ty: Box::new(self.clone()),
			length,
		}
	}

	pub fn as_pointer(&self) -> Self {
		Self::Pointer(Box::new(self.clone()))
	}

	pub fn as_const(&self) -> Self {
		match self {
			Self::Const(ty) => self.clone(),
			ty => Self::Const(Box::new(ty.clone()))
		}
	}

	pub fn size_of(&self) -> Option<usize> {
		match self {
			Self::Unit => Some(0),
			Self::USize => Some(std::mem::size_of::<usize>()),
			Self::I64 | Self::U64 | Self::F64 => Some(8),
			Self::I32 | Self::U32 | Self::F32 => Some(4),
			Self::U8 | Self::I8 => Some(1),
			Self::Const(ty) => ty.size_of(),
			Self::Array { ty, length } => ty.size_of().map(|x| x * length),
			Self::Function(_) | Self::Pointer(_) => Self::USize.size_of(),
			Self::Custom { .. } => None,
		}
	}

	pub fn name(&self) -> String {
		match self {
			Self::Unit => "unit".into(),
			Self::USize => "usize".into(),
			Self::I64 => "i64".into(),
			Self::I32 => "i32".into(),
			Self::I8 => "i8".into(),
			Self::U64 => "u64".into(),
			Self::U32 => "u32".into(),
			Self::U8 => "u8".into(),
			Self::F32 => "f32".into(),
			Self::F64 => "f64".into(),
			Self::Function(signature) => format!("{signature}"),
			Self::Const(underlying) => format!("const {underlying}"),
			Self::Pointer(underlying) => format!("*{underlying}"),
			Self::Array {
				ty, length
			} => format!("[{ty}; {length}]"),
			Self::Custom { name, template_args } => {
				let mut string = name.0.clone();

				if !template_args.is_empty() {
					string.push('<');

					string.push_str(format!("{}", template_args[0]).as_str());
					let template_args = &template_args[1..];

					for arg in template_args {
						string.push_str(format!(", {arg}").as_str());
					}

					string.push('>');
				}

				string
			}
		}
	}

	const fn is_pointer_type(&self) -> bool {
		matches!(self, &Self::Pointer(_))
	}

	fn value_at_pointer(&self) -> Option<Self> {
		match self {
			Self::Pointer(ty) => Some(ty.as_ref().clone()),
			_ => None
		}
	}

	fn value_under_cost(&self) -> Self {
		match self {
			Self::Const(ty) => ty.value_under_cost(),
			ty => ty.clone()
		}
	}
}

impl From<Identifier> for Type {
	fn from(value: Identifier) -> Self {
		match value.0.as_str() {
			"i64" => Self::I64,
			"i32" => Self::I32,
			"i8" => Self::I8,
			"u64" => Self::U64,
			"u32" => Self::U32,
			"u8" => Self::U8,
			"f32" => Self::F32,
			"f64" => Self::F64,
			"unit" => Self::Unit,
			"usize" => Self::USize,
			name => Self::Custom {
				name: name.to_string().into(),
				template_args: vec![],
			}
		}
	}
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Variable {
	ident: Identifier,
	type_id: Type,
}

impl Variable {
	pub fn new(ident: impl Into<Identifier>, type_id: Type) -> Self {
		Self { ident: ident.into(), type_id }
	}

	pub const fn get_name(&self) -> &Identifier {
		&self.ident
	}

	pub const fn get_type(&self) -> &Type {
		&self.type_id
	}
}
