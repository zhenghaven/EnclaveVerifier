use std::fmt;

use std::vec::Vec;
use std::string::String;

use std::boxed::Box;

#[derive(Clone)]
pub enum Aexp
{
	IntConst {v :  i32},
	FloConst {v :  f32},
	Add {l : Box<Aexp>, r : Box<Aexp>},
	Sub {l : Box<Aexp>, r : Box<Aexp>},
	Mul {l : Box<Aexp>, r : Box<Aexp>},
	Div {l : Box<Aexp>, r : Box<Aexp>},
	Mod {l : Box<Aexp>, r : Box<Aexp>},
	Var {v : super::var_general::VarRef},
	FnCall {fc : super::func_general::FnCall},
}

impl Aexp
{
	fn to_byte_id(&self) -> ByteId
	{
		match self
		{
			Aexp::IntConst{v:_} => ByteId::IntConst,
			Aexp::FloConst{v:_} => ByteId::FloConst,
			Aexp::Add{l:_, r:_} => ByteId::Add,
			Aexp::Sub{l:_, r:_} => ByteId::Sub,
			Aexp::Mul{l:_, r:_} => ByteId::Mul,
			Aexp::Div{l:_, r:_} => ByteId::Div,
			Aexp::Mod{l:_, r:_} => ByteId::Mod,
			Aexp::Var{v:_}      => ByteId::Var,
			Aexp::FnCall{fc:_}  => ByteId::FnCall,
		}
	}
}

impl super::Serializible for Aexp
{
	/// Serialize the AST (of Aexp type) into serials of bytes, and return the vector of bytes.
	///
	/// Please refer to the documentation on the trait for detail.
	///
	/// # Aexp layout
	/// ```
	/// IntConst:  | type=0 - 1 Byte | i32 - 5 bytes |
	/// FloConst:  | type=1 - 1 Byte | f32 - 5 bytes |
	/// Add:       | type=2 - 1 Byte | Aexp::bytes   | Aexp::bytes   |
	/// Sub:       | type=3 - 1 Byte | Aexp::bytes   | Aexp::bytes   |
	/// Mul:       | type=4 - 1 Byte | Aexp::bytes   | Aexp::bytes   |
	/// Div:       | type=5 - 1 Byte | Aexp::bytes   | Aexp::bytes   |
	/// Mod:       | type=6 - 1 Byte | Aexp::bytes   | Aexp::bytes   |
	/// Var:       | type=7 - 1 Byte | VarRef::bytes |
	/// FnCall:    | type=8 - 1 Byte | FnCall::bytes |
	/// ```
	///
	fn to_bytes(&self) -> Result<Vec<u8>, String>
	{
		let mut res : Vec<u8> = vec![self.to_byte_id().to_byte()];

		match self
		{
			Aexp::IntConst{v} =>
			{
				res.append(&mut super::primit_serialize::int32_to_bytes(v));
				Result::Ok(res)
			},
			Aexp::FloConst{v} =>
			{
				res.append(&mut super::primit_serialize::flo32_to_bytes(v));
				Result::Ok(res)
			},
			Aexp::Add{l, r} =>
			{
				let mut lf_res = l.to_bytes()?;
				let mut rt_res = r.to_bytes()?;
				res.append(&mut lf_res);
				res.append(&mut rt_res);
				Result::Ok(res)
			},
			Aexp::Sub{l, r} =>
			{
				let mut lf_res = l.to_bytes()?;
				let mut rt_res = r.to_bytes()?;
				res.append(&mut lf_res);
				res.append(&mut rt_res);
				Result::Ok(res)
			},
			Aexp::Mul{l, r} =>
			{
				let mut lf_res = l.to_bytes()?;
				let mut rt_res = r.to_bytes()?;
				res.append(&mut lf_res);
				res.append(&mut rt_res);
				Result::Ok(res)
			},
			Aexp::Div{l, r} =>
			{
				let mut lf_res = l.to_bytes()?;
				let mut rt_res = r.to_bytes()?;
				res.append(&mut lf_res);
				res.append(&mut rt_res);
				Result::Ok(res)
			},
			Aexp::Mod{l, r} =>
			{
				let mut lf_res = l.to_bytes()?;
				let mut rt_res = r.to_bytes()?;
				res.append(&mut lf_res);
				res.append(&mut rt_res);
				Result::Ok(res)
			},
			Aexp::Var{v} =>
			{
				let mut var_bytes = v.to_bytes()?;
				res.append(&mut var_bytes);
				Result::Ok(res)
			},
			Aexp::FnCall{fc} =>
			{
				let mut fc_bytes = fc.to_bytes()?;
				res.append(&mut fc_bytes);
				Result::Ok(res)
			},
		}
	}
}

impl super::Deserializible<Aexp> for Aexp
{
	fn from_bytes(bytes : &[u8]) -> Result<(&[u8], Aexp), String>
	{
		use constructor_helper::*;
		if bytes.len() > 0
		{
			let type_id = ByteId::from_byte(&bytes[0])?;
			match type_id
			{
				ByteId::IntConst => //Aexp::IntConst
				{
					let (bytes_left, parsed_val) = super::primit_serialize::int32_from_bytes(&bytes[1..])?;

					Result::Ok((bytes_left, parsed_val.to_aexp()))
				},
				ByteId::FloConst => //Aexp::FloConst
				{
					let (bytes_left, parsed_val) = super::primit_serialize::flo32_from_bytes(&bytes[1..])?;

					Result::Ok((bytes_left, parsed_val.to_aexp()))
				},
				ByteId::Add => //Aexp::Add
				{
					let (bytes_left_l, parsed_val_l) = Aexp::from_bytes(&bytes[1..])?;
					let (bytes_left_r, parsed_val_r) = Aexp::from_bytes(bytes_left_l)?;

					Result::Ok((bytes_left_r, parsed_val_l + parsed_val_r))
				},
				ByteId::Sub => //Aexp::Sub
				{
					let (bytes_left_l, parsed_val_l) = Aexp::from_bytes(&bytes[1..])?;
					let (bytes_left_r, parsed_val_r) = Aexp::from_bytes(bytes_left_l)?;

					Result::Ok((bytes_left_r, parsed_val_l - parsed_val_r))
				},
				ByteId::Mul => //Aexp::Mul
				{
					let (bytes_left_l, parsed_val_l) = Aexp::from_bytes(&bytes[1..])?;
					let (bytes_left_r, parsed_val_r) = Aexp::from_bytes(bytes_left_l)?;

					Result::Ok((bytes_left_r, parsed_val_l * parsed_val_r))
				},
				ByteId::Div => //Aexp::Div
				{
					let (bytes_left_l, parsed_val_l) = Aexp::from_bytes(&bytes[1..])?;
					let (bytes_left_r, parsed_val_r) = Aexp::from_bytes(bytes_left_l)?;

					Result::Ok((bytes_left_r, parsed_val_l / parsed_val_r))
				},
				ByteId::Mod => //Aexp::Mod
				{
					let (bytes_left_l, parsed_val_l) = Aexp::from_bytes(&bytes[1..])?;
					let (bytes_left_r, parsed_val_r) = Aexp::from_bytes(bytes_left_l)?;

					Result::Ok((bytes_left_r, parsed_val_l % parsed_val_r))
				},
				ByteId::Var => //Aexp::Var
				{
					let (bytes_left, parsed_val) = super::var_general::VarRef::from_bytes(&bytes[1..])?;

					Result::Ok((bytes_left, Aexp::Var {v : parsed_val}))
				},
				ByteId::FnCall => //Aexp::Var
				{
					let (bytes_left, parsed_val) = super::func_general::FnCall::from_bytes(&bytes[1..])?;

					Result::Ok((bytes_left, Aexp::FnCall {fc : parsed_val}))
				}
			}
		}
		else
		{
			Result::Err(format!("{}", "Failed to parse Aexp. Bytes are shorter than expected."))
		}
	}
}

impl fmt::Display for Aexp
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match self
		{
			Aexp::IntConst{v} => write!(f, "{}", v),
			Aexp::FloConst{v} => write!(f, "{}", v),
			Aexp::Add{l, r} => write!(f, "({} + {})", l, r),
			Aexp::Sub{l, r} => write!(f, "({} - {})", l, r),
			Aexp::Mul{l, r} => write!(f, "({} * {})", l, r),
			Aexp::Div{l, r} => write!(f, "({} / {})", l, r),
			Aexp::Mod{l, r} => write!(f, "({} % {})", l, r),
			Aexp::Var{v} => write!(f, "{}", v),
			Aexp::FnCall{fc} => write!(f, "{}", fc),
		}
	}
}

pub mod constructor_helper
{
	use std::ops;
	use std::boxed::Box;

	//Helper for constant types:

	pub trait ToAexp
	{
		fn to_aexp(self) -> super::Aexp;
	}

	impl ToAexp for i32
	{
		fn to_aexp(self) -> super::Aexp
		{
			super::Aexp::IntConst{v : self}
		}
	}

	impl ToAexp for f32
	{
		fn to_aexp(self) -> super::Aexp
		{
			super::Aexp::FloConst{v : self}
		}
	}

	//Helper for Variables:

	pub trait RefToAexp
	{
		fn to_aexp(&self) -> super::Aexp;
	}

	impl RefToAexp for str
	{
		fn to_aexp(&self) -> super::Aexp
		{
			super::Aexp::Var
			{
				v : super::super::var_general::VarRef::from_str(self)
			}
		}
	}

	//Helper for operations:

	impl ops::Add<super::Aexp> for super::Aexp
	{
		type Output = super::Aexp;

		fn add(self, _rhs : super::Aexp) -> super::Aexp
		{
			super::Aexp::Add {l : Box::new(self), r : Box::new(_rhs)}
		}
	}

	impl ops::Sub<super::Aexp> for super::Aexp
	{
		type Output = super::Aexp;

		fn sub(self, _rhs : super::Aexp) -> super::Aexp
		{
			super::Aexp::Sub {l : Box::new(self), r : Box::new(_rhs)}
		}
	}

	impl ops::Mul<super::Aexp> for super::Aexp
	{
		type Output = super::Aexp;

		fn mul(self, _rhs : super::Aexp) -> super::Aexp
		{
			super::Aexp::Mul {l : Box::new(self), r : Box::new(_rhs)}
		}
	}

	impl ops::Div<super::Aexp> for super::Aexp
	{
		type Output = super::Aexp;

		fn div(self, _rhs : super::Aexp) -> super::Aexp
		{
			super::Aexp::Div {l : Box::new(self), r : Box::new(_rhs)}
		}
	}

	impl ops::Rem<super::Aexp> for super::Aexp
	{
		type Output = super::Aexp;

		fn rem(self, _rhs : super::Aexp) -> super::Aexp
		{
			super::Aexp::Mod {l : Box::new(self), r : Box::new(_rhs)}
		}
	}
}

enum ByteId
{
	IntConst,
	FloConst,
	Add,
	Sub,
	Mul,
	Div,
	Mod,
	Var,
	FnCall,
}

impl ByteId
{
	fn to_byte(&self) -> u8
	{
		match self
		{
			ByteId::IntConst => 0u8,
			ByteId::FloConst => 1u8,
			ByteId::Add    => 2u8,
			ByteId::Sub    => 3u8,
			ByteId::Mul    => 4u8,
			ByteId::Div    => 5u8,
			ByteId::Mod    => 6u8,
			ByteId::Var    => 7u8,
			ByteId::FnCall => 8u8,
		}
	}

	fn from_byte(b : &u8) -> Result<ByteId, String>
	{
		match b
		{
			0u8 => Result::Ok(ByteId::IntConst),
			1u8 => Result::Ok(ByteId::FloConst),
			2u8 => Result::Ok(ByteId::Add),
			3u8 => Result::Ok(ByteId::Sub),
			4u8 => Result::Ok(ByteId::Mul),
			5u8 => Result::Ok(ByteId::Div),
			6u8 => Result::Ok(ByteId::Mod),
			7u8 => Result::Ok(ByteId::Var),
			8u8 => Result::Ok(ByteId::FnCall),
			_   => Result::Err(format!("{}", "Unrecognized type ID from byte for Aexp."))
		}
	}
}

