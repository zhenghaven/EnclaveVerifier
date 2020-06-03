use std::fmt;

use std::vec::Vec;
use std::string::String;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DataType
{
	Void,
	Int32,
	Float32,
	Bool,
}

impl DataType
{
	fn to_byte(&self) -> u8
	{
		match self
		{
			DataType::Void    => 0u8,
			DataType::Int32   => 1u8,
			DataType::Float32 => 2u8,
			DataType::Bool    => 3u8,
		}
	}

	fn from_byte(b : &u8) -> Result<DataType, String>
	{
		match b
		{
			0u8 => Result::Ok(DataType::Void),
			1u8 => Result::Ok(DataType::Int32),
			2u8 => Result::Ok(DataType::Float32),
			3u8 => Result::Ok(DataType::Bool),
			_   => Result::Err(format!("{}", "Unrecognized byte ID for data type."))
		}
	}
}

impl super::Serializible for DataType
{
	/// Serialize the AST (of VarDecl type) into serials of bytes, and return the vector of bytes.
	///
	/// Please refer to the documentation on the trait for detail.
	///
	/// # VarDecl layout
	/// ```
	///            | Datatype - 1 Byte |
	/// ```
	///
	fn to_bytes(&self) -> Result<Vec<u8>, String>
	{
		Ok(vec![self.to_byte()])
	}
}

impl super::Deserializible for DataType
{
	fn from_bytes(bytes : &[u8]) -> Result<(&[u8], DataType), String>
	{
		let data_type = DataType::from_byte(&bytes[0])?;

		Result::Ok((&bytes[1..], data_type))
	}
}

impl fmt::Display for DataType
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match self
		{
			DataType::Void => write!(f, "{}", "void"),
			DataType::Int32 => write!(f, "{}", "i32"),
			DataType::Float32 => write!(f, "{}", "f32"),
			DataType::Bool => write!(f, "{}", "bool"),
		}
	}
}
