use std::fmt;

pub enum DataType
{
	Void,
	Int32,
	Float32,
	Bool,
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
