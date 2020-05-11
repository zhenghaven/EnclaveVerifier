use std::fmt;

pub struct VarDecl
{
	var_type : super::data_type::DataType,
	name : String,
}

impl VarDecl
{
	pub fn new(var_type : super::data_type::DataType, name : String) -> VarDecl
	{
		VarDecl{var_type : var_type, name : name}
	}
}

impl fmt::Display for VarDecl
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f, "var {} : {}", self.name, self.var_type)
	}
}

pub struct VarRef
{
	name : String,
}

impl VarRef
{
	pub fn from_str(name : &str) -> VarRef
	{
		VarRef{name : name.to_string()}
	}
}

impl fmt::Display for VarRef
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f, "{}", self.name)
	}
}
