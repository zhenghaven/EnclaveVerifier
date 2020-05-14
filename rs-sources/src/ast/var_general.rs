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

impl super::Deserializible<VarDecl> for VarDecl
{
	fn from_bytes(bytes : &[u8]) -> Result<(&[u8], VarDecl), String>
	{
		let (bytes_left_1, parsed_var_type) = super::data_type::DataType::from_bytes(bytes)?;
		let (bytes_left_2, parsed_name) = super::primit_serialize::string_from_bytes(bytes_left_1)?;

		Result::Ok((bytes_left_2, VarDecl {var_type : parsed_var_type, name : parsed_name}))
	}
}

impl super::Serializible for VarDecl
{
	fn to_bytes(&self) -> Result<Vec<u8>, String>
	{
		let mut res = self.var_type.to_bytes()?;
		res.append(&mut super::primit_serialize::string_to_bytes(&self.name));

		Result::Ok(res)
	}
}

impl fmt::Display for VarDecl
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f, "{} : {}", self.name, self.var_type)
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

impl super::Deserializible<VarRef> for VarRef
{
	fn from_bytes(bytes : &[u8]) -> Result<(&[u8], VarRef), String>
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
