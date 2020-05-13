use std::fmt;

pub enum Exp
{
	A {e : super::aexp::Aexp},
}

enum ByteId
{
	A,
}

impl ByteId
{
	fn to_byte(&self) -> u8
	{
		match self
		{
			ByteId::A => 0u8,
		}
	}

	fn from_byte(b : &u8) -> Result<ByteId, String>
	{
		match b
		{
			0u8 => Result::Ok(ByteId::A),
			_   => Result::Err("Unrecognized type ID from byte for Exp.".to_string()),
		}
	}
}

impl Exp
{
	fn get_byte_id(&self) -> ByteId
	{
		match self
		{
			Exp::A {e:_} => ByteId::A,
		}
	}

	pub fn from_bytes(bytes : &[u8]) -> Result<(&[u8], Exp), String>
	{
		let byte_id = ByteId::from_byte(&bytes[0])?;

		if bytes.len() > 0
		{
			match byte_id
			{
				ByteId::A =>
				{
					use constructor_helper::ExpType;
					let (left_bytes, aexp_res) = super::aexp::Aexp::from_bytes(&bytes[1..])?;
					Result::Ok((left_bytes, aexp_res.to_exp()))
				}
			}
		}
		else
		{
			Result::Err("Failed to parse Exp. Bytes are shorter than expected.". to_string())
		}
	}
}

impl super::Serializible for Exp
{
	fn to_bytes(&self) -> Result<Vec<u8>, String>
	{
		let mut res :Vec<u8> = vec![self.get_byte_id().to_byte()];

		match self
		{
			Exp::A {e} =>
			{
				res.append(&mut e.to_bytes()?);

				Result::Ok(res)
			}
		}
	}
}

impl fmt::Display for Exp
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match self
		{
			Exp::A {e} => write!(f, "{}", e)
		}
	}
}

pub mod constructor_helper
{
	pub trait ExpType
	{
		fn to_exp(self) -> super::Exp;
	}

	impl ExpType for super::super::aexp::Aexp
	{
		fn to_exp(self) -> super::Exp
		{
			super::Exp::A {e : self}
		}
	}
}