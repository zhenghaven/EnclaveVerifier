use std::fmt;

pub enum Exp
{
	A {e : super::aexp::Aexp},
	B {e : super::bexp::Bexp},
}

enum ByteId
{
	A,
	B,
}

impl ByteId
{
	fn to_byte(&self) -> u8
	{
		match self
		{
			ByteId::A => 0u8,
			ByteId::B => 1u8,
		}
	}

	fn from_byte(b : &u8) -> Result<ByteId, String>
	{
		match b
		{
			0u8 => Result::Ok(ByteId::A),
			1u8 => Result::Ok(ByteId::B),
			_   => Result::Err(format!("Unrecognized type ID ({}) from byte for Exp.", b)),
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
			Exp::B {e:_} => ByteId::B,
		}
	}

	pub fn from_bytes(bytes : &[u8]) -> Result<(&[u8], Exp), String>
	{
		use constructor_helper::*;

		let byte_id = ByteId::from_byte(&bytes[0])?;

		if bytes.len() > 0
		{
			match byte_id
			{
				ByteId::A =>
				{
					let (left_bytes, aexp_res) = super::aexp::Aexp::from_bytes(&bytes[1..])?;
					Result::Ok((left_bytes, aexp_res.to_exp()))
				},
				ByteId::B =>
				{
					let (left_bytes, bexp_res) = super::bexp::Bexp::from_bytes(&bytes[1..])?;
					Result::Ok((left_bytes, bexp_res.to_exp()))
				},
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
			},
			Exp::B {e} =>
			{
				res.append(&mut e.to_bytes()?);

				Result::Ok(res)
			},
		}
	}
}

impl fmt::Display for Exp
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match self
		{
			Exp::A {e} => write!(f, "{}", e),
			Exp::B {e} => write!(f, "{}", e),
		}
	}
}

pub mod constructor_helper
{
	pub trait ToExp
	{
		fn to_exp(self) -> super::Exp;
	}

	impl ToExp for super::super::aexp::Aexp
	{
		fn to_exp(self) -> super::Exp
		{
			super::Exp::A {e : self}
		}
	}

	impl ToExp for super::super::bexp::Bexp
	{
		fn to_exp(self) -> super::Exp
		{
			super::Exp::B {e : self}
		}
	}
}