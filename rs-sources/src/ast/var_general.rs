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

	pub fn from_bytes(bytes : &[u8]) -> Result<(&[u8], VarRef), String>
	{
		let (bytes_left, parsed_val) = super::primit_serialize::string_from_bytes(bytes)?;

		Result::Ok((bytes_left, VarRef::from_str(&parsed_val[..])))
	}
}

impl super::Serializible for VarRef
{
	fn to_bytes(&self) -> Result<Vec<u8>, String>
	{
		Result::Ok(super::primit_serialize::string_to_bytes(&self.name))
	}
}

impl fmt::Display for VarRef
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f, "{}", self.name)
	}
}
