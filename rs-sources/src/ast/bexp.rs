use std::fmt;

use std::vec::Vec;
use std::string::String;

use std::boxed::Box;

#[derive(Clone)]
pub enum Bexp
{
	/* T / F */  BoolConst {v :  bool},
	/// Equivalent for boolean expressions
	/* == */     Beq       {l : Box<Bexp>, r : Box<Bexp>},
	/// Not-equivalent for boolean expressions
	/* != */     Bneq      {l : Box<Bexp>, r : Box<Bexp>},
	/* && */     And       {l : Box<Bexp>, r : Box<Bexp>},
	/* || */     Or        {l : Box<Bexp>, r : Box<Bexp>},
	/* ! */      Not       {e : Box<Bexp>},
	/// Equivalent for arithmetic expressions
	/* == */     Aeq       {l : Box<super::aexp::Aexp>, r : Box<super::aexp::Aexp>},
	/// Not-equivalent for arithmetic expressions
	/* != */     Aneq      {l : Box<super::aexp::Aexp>, r : Box<super::aexp::Aexp>},
	/* <  */     Lt        {l : Box<super::aexp::Aexp>, r : Box<super::aexp::Aexp>},
	/* <= */     Lte       {l : Box<super::aexp::Aexp>, r : Box<super::aexp::Aexp>},
	/* >  */     Gt        {l : Box<super::aexp::Aexp>, r : Box<super::aexp::Aexp>},
	/* >= */     Gte       {l : Box<super::aexp::Aexp>, r : Box<super::aexp::Aexp>},
	/* x  */     Var       {v : super::var_general::VarRef},
	/* foo() */  FnCall    {fc : super::func_general::FnCall},
}

impl Bexp
{
	fn to_byte_id(&self) -> ByteId
	{
		match self
		{
			Bexp::BoolConst{v:_} => ByteId::BoolConst,
			Bexp::Beq {l:_, r:_} => ByteId::Beq,
			Bexp::Bneq{l:_, r:_} => ByteId::Bneq,
			Bexp::And {l:_, r:_} => ByteId::And,
			Bexp::Or  {l:_, r:_} => ByteId::Or,
			Bexp::Not {e:_}      => ByteId::Not,
			Bexp::Aeq {l:_, r:_} => ByteId::Aeq,
			Bexp::Aneq{l:_, r:_} => ByteId::Aneq,
			Bexp::Lt  {l:_, r:_} => ByteId::Lt,
			Bexp::Lte {l:_, r:_} => ByteId::Lte,
			Bexp::Gt  {l:_, r:_} => ByteId::Gt,
			Bexp::Gte {l:_, r:_} => ByteId::Gte,
			Bexp::Var {v:_}      => ByteId::Var,
			Bexp::FnCall{fc:_}   => ByteId::FnCall,
		}
	}
}

impl super::Serializible for Bexp
{
	/// Serialize the AST (of Bexp type) into serials of bytes, and return the vector of bytes.
	///
	/// Please refer to the documentation on the trait for detail.
	///
	/// # Bexp layout
	/// ```
	/// BoolConst:  | type=0 -  1 Byte  | bool - 2 bytes |
	/// Beq:        | type=1 -  1 Byte  | Bexp::bytes    | Bexp::bytes   |
	/// Bneq:       | type=2 -  1 Byte  | Bexp::bytes    | Bexp::bytes   |
	/// And:        | type=3 -  1 Byte  | Bexp::bytes    | Bexp::bytes   |
	/// Or:         | type=4 -  1 Byte  | Bexp::bytes    | Bexp::bytes   |
	/// Not:        | type=5 -  1 Byte  | Bexp::bytes
	/// Aeq:        | type=6 -  1 Byte  | Aexp::bytes    | Aexp::bytes   |
	/// Aneq:       | type=7 -  1 Byte  | Aexp::bytes    | Aexp::bytes   |
	/// Lt:         | type=8 -  1 Byte  | Aexp::bytes    | Aexp::bytes   |
	/// Lte:        | type=9 -  1 Byte  | Aexp::bytes    | Aexp::bytes   |
	/// Gt:         | type=10 - 1 Byte  | Aexp::bytes    | Aexp::bytes   |
	/// Gte:        | type=11 - 1 Byte  | Aexp::bytes    | Aexp::bytes   |
	/// Var:        | type=12 - 1 Byte  | VarRef::bytes  |
	/// FnCall:     | type=13 - 1 Byte  | FnCall::bytes  |
	/// ```
	///
	fn to_bytes(&self) -> Result<Vec<u8>, String>
	{
		let mut res : Vec<u8> = vec![self.to_byte_id().to_byte()];

		match self
		{
			Bexp::BoolConst{v} =>
			{
				res.append(&mut super::primit_serialize::bool_to_bytes(v));
				Result::Ok(res)
			},
			Bexp::Beq {l, r} =>
			{
				res.append(&mut (l.to_bytes()?));
				res.append(&mut (r.to_bytes()?));
				Result::Ok(res)
			},
			Bexp::Bneq{l, r} =>
			{
				res.append(&mut (l.to_bytes()?));
				res.append(&mut (r.to_bytes()?));
				Result::Ok(res)
			},
			Bexp::And {l, r} =>
			{
				res.append(&mut (l.to_bytes()?));
				res.append(&mut (r.to_bytes()?));
				Result::Ok(res)
			},
			Bexp::Or  {l, r} =>
			{
				res.append(&mut (l.to_bytes()?));
				res.append(&mut (r.to_bytes()?));
				Result::Ok(res)
			},
			Bexp::Not {e} =>
			{
				res.append(&mut (e.to_bytes()?));
				Result::Ok(res)
			},
			Bexp::Aeq {l, r} =>
			{
				res.append(&mut (l.to_bytes()?));
				res.append(&mut (r.to_bytes()?));
				Result::Ok(res)
			},
			Bexp::Aneq{l, r} =>
			{
				res.append(&mut (l.to_bytes()?));
				res.append(&mut (r.to_bytes()?));
				Result::Ok(res)
			},
			Bexp::Lt  {l, r} =>
			{
				res.append(&mut (l.to_bytes()?));
				res.append(&mut (r.to_bytes()?));
				Result::Ok(res)
			},
			Bexp::Lte {l, r} =>
			{
				res.append(&mut (l.to_bytes()?));
				res.append(&mut (r.to_bytes()?));
				Result::Ok(res)
			},
			Bexp::Gt  {l, r} =>
			{
				res.append(&mut (l.to_bytes()?));
				res.append(&mut (r.to_bytes()?));
				Result::Ok(res)
			},
			Bexp::Gte {l, r} =>
			{
				res.append(&mut (l.to_bytes()?));
				res.append(&mut (r.to_bytes()?));
				Result::Ok(res)
			},
			Bexp::Var {v}    =>
			{
				res.append(&mut (v.to_bytes()?));
				Result::Ok(res)
			},
			Bexp::FnCall{fc} =>
			{
				res.append(&mut (fc.to_bytes()?));
				Result::Ok(res)
			},
		}
	}
}

impl super::Deserializible<Bexp> for Bexp
{
	fn from_bytes(bytes : &[u8]) -> Result<(&[u8], Bexp), String>
	{
		use constructor_helper::*;
		if bytes.len() > 0
		{
			let type_id = ByteId::from_byte(&bytes[0])?;
			match type_id
			{
				ByteId::BoolConst =>
				{
					let (bytes_left, parsed_val) = super::primit_serialize::bool_from_bytes(&bytes[1..])?;

					Result::Ok((bytes_left, parsed_val.to_bexp()))
				},
				ByteId::Beq =>
				{
					let (bytes_left_l, parsed_val_l) = Bexp::from_bytes(&bytes[1..])?;
					let (bytes_left_r, parsed_val_r) = Bexp::from_bytes(bytes_left_l)?;

					Result::Ok((bytes_left_r, parsed_val_l.beq(parsed_val_r)))
				},
				ByteId::Bneq =>
				{
					let (bytes_left_l, parsed_val_l) = Bexp::from_bytes(&bytes[1..])?;
					let (bytes_left_r, parsed_val_r) = Bexp::from_bytes(bytes_left_l)?;

					Result::Ok((bytes_left_r, parsed_val_l.bneq(parsed_val_r)))
				},
				ByteId::And =>
				{
					let (bytes_left_l, parsed_val_l) = Bexp::from_bytes(&bytes[1..])?;
					let (bytes_left_r, parsed_val_r) = Bexp::from_bytes(bytes_left_l)?;

					Result::Ok((bytes_left_r, parsed_val_l.and(parsed_val_r)))
				},
				ByteId::Or =>
				{
					let (bytes_left_l, parsed_val_l) = Bexp::from_bytes(&bytes[1..])?;
					let (bytes_left_r, parsed_val_r) = Bexp::from_bytes(bytes_left_l)?;

					Result::Ok((bytes_left_r, parsed_val_l.or(parsed_val_r)))
				},
				ByteId::Not =>
				{
					let (bytes_left, parsed_val) = Bexp::from_bytes(&bytes[1..])?;

					Result::Ok((bytes_left, parsed_val.not()))
				},
				ByteId::Aeq =>
				{
					let (bytes_left_l, parsed_val_l) = super::aexp::Aexp::from_bytes(&bytes[1..])?;
					let (bytes_left_r, parsed_val_r) = super::aexp::Aexp::from_bytes(bytes_left_l)?;

					Result::Ok((bytes_left_r, parsed_val_l.aeq(parsed_val_r)))
				},
				ByteId::Aneq =>
				{
					let (bytes_left_l, parsed_val_l) = super::aexp::Aexp::from_bytes(&bytes[1..])?;
					let (bytes_left_r, parsed_val_r) = super::aexp::Aexp::from_bytes(bytes_left_l)?;

					Result::Ok((bytes_left_r, parsed_val_l.aneq(parsed_val_r)))
				},
				ByteId::Lt =>
				{
					let (bytes_left_l, parsed_val_l) = super::aexp::Aexp::from_bytes(&bytes[1..])?;
					let (bytes_left_r, parsed_val_r) = super::aexp::Aexp::from_bytes(bytes_left_l)?;

					Result::Ok((bytes_left_r, parsed_val_l.lt(parsed_val_r)))
				},
				ByteId::Lte =>
				{
					let (bytes_left_l, parsed_val_l) = super::aexp::Aexp::from_bytes(&bytes[1..])?;
					let (bytes_left_r, parsed_val_r) = super::aexp::Aexp::from_bytes(bytes_left_l)?;

					Result::Ok((bytes_left_r, parsed_val_l.lte(parsed_val_r)))
				},
				ByteId::Gt =>
				{
					let (bytes_left_l, parsed_val_l) = super::aexp::Aexp::from_bytes(&bytes[1..])?;
					let (bytes_left_r, parsed_val_r) = super::aexp::Aexp::from_bytes(bytes_left_l)?;

					Result::Ok((bytes_left_r, parsed_val_l.gt(parsed_val_r)))
				},
				ByteId::Gte =>
				{
					let (bytes_left_l, parsed_val_l) = super::aexp::Aexp::from_bytes(&bytes[1..])?;
					let (bytes_left_r, parsed_val_r) = super::aexp::Aexp::from_bytes(bytes_left_l)?;

					Result::Ok((bytes_left_r, parsed_val_l.gte(parsed_val_r)))
				},
				ByteId::Var =>
				{
					let (bytes_left, parsed_val) = super::var_general::VarRef::from_bytes(&bytes[1..])?;

					Result::Ok((bytes_left, Bexp::Var {v : parsed_val}))
				},
				ByteId::FnCall =>
				{
					let (bytes_left, parsed_val) = super::func_general::FnCall::from_bytes(&bytes[1..])?;

					Result::Ok((bytes_left, Bexp::FnCall {fc : parsed_val}))
				},
			}
		}
		else
		{
			Result::Err(format!("{}", "Failed to parse Bexp. Bytes are shorter than expected."))
		}
	}
}

impl fmt::Display for Bexp
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match self
		{
			Bexp::BoolConst{v} => write!(f, "{}", v),
			Bexp::Beq  {l, r}  => write!(f, "({} == {})", l, r),
			Bexp::Bneq {l, r}  => write!(f, "({} != {})", l, r),
			Bexp::And  {l, r}  => write!(f, "({} && {})", l, r),
			Bexp::Or   {l, r}  => write!(f, "({} || {})", l, r),
			Bexp::Not  {e}     => write!(f, "(!{})", e),
			Bexp::Aeq  {l, r}  => write!(f, "({} == {})", l, r),
			Bexp::Aneq {l, r}  => write!(f, "({} != {})", l, r),
			Bexp::Lt   {l, r}  => write!(f, "({} < {})",  l, r),
			Bexp::Lte  {l, r}  => write!(f, "({} <= {})", l, r),
			Bexp::Gt   {l, r}  => write!(f, "({} > {})",  l, r),
			Bexp::Gte  {l, r}  => write!(f, "({} >= {})", l, r),
			Bexp::Var  {v}     => write!(f, "{}", v),
			Bexp::FnCall{fc}   => write!(f, "{}", fc),
		}
	}
}

pub mod constructor_helper
{
	use std::boxed::Box;

	//Helper for constant types:

	pub trait ToBexp
	{
		fn to_bexp(self) -> super::Bexp;
	}

	impl ToBexp for bool
	{
		fn to_bexp(self) -> super::Bexp
		{
			super::Bexp::BoolConst{v : self}
		}
	}

	//Helper for Variables:

	pub trait RefToBexp
	{
		fn to_bexp(&self) -> super::Bexp;
	}

	impl RefToBexp for str
	{
		fn to_bexp(&self) -> super::Bexp
		{
			super::Bexp::Var
			{
				v : super::super::var_general::VarRef::from_str(self)
			}
		}
	}

	impl super::Bexp
	{
		pub fn beq(self, r_in : super::Bexp) -> super::Bexp
		{
			super::Bexp::Beq{l : Box::new(self), r : Box::new(r_in)}
		}

		pub fn bneq(self, r_in : super::Bexp) -> super::Bexp
		{
			super::Bexp::Bneq{l : Box::new(self), r : Box::new(r_in)}
		}

		pub fn and(self, r_in : super::Bexp) -> super::Bexp
		{
			super::Bexp::And{l : Box::new(self), r : Box::new(r_in)}
		}

		pub fn or(self, r_in : super::Bexp) -> super::Bexp
		{
			super::Bexp::Or{l : Box::new(self), r : Box::new(r_in)}
		}

		pub fn not(self) -> super::Bexp
		{
			super::Bexp::Not{e : Box::new(self)}
		}
	}

	impl super::super::aexp::Aexp
	{
		pub fn aeq(self, r_in : super::super::aexp::Aexp) -> super::Bexp
		{
			super::Bexp::Aeq{l : Box::new(self), r : Box::new(r_in)}
		}

		pub fn aneq(self, r_in : super::super::aexp::Aexp) -> super::Bexp
		{
			super::Bexp::Aneq{l : Box::new(self), r : Box::new(r_in)}
		}

		pub fn lt(self, r_in : super::super::aexp::Aexp) -> super::Bexp
		{
			super::Bexp::Lt{l : Box::new(self), r : Box::new(r_in)}
		}

		pub fn lte(self, r_in : super::super::aexp::Aexp) -> super::Bexp
		{
			super::Bexp::Lte{l : Box::new(self), r : Box::new(r_in)}
		}

		pub fn gt(self, r_in : super::super::aexp::Aexp) -> super::Bexp
		{
			super::Bexp::Gt{l : Box::new(self), r : Box::new(r_in)}
		}

		pub fn gte(self, r_in : super::super::aexp::Aexp) -> super::Bexp
		{
			super::Bexp::Gte{l : Box::new(self), r : Box::new(r_in)}
		}
	}
}

enum ByteId
{
	BoolConst,
	Beq,
	Bneq,
	And,
	Or,
    Not,
	Aeq,
	Aneq,
	Lt,
	Lte,
	Gt,
	Gte,
	Var,
	FnCall,
}

impl ByteId
{
	fn to_byte(&self) -> u8
	{
		match self
		{
			ByteId::BoolConst => 0u8,
			ByteId::Beq       => 1u8,
			ByteId::Bneq      => 2u8,
			ByteId::And       => 3u8,
			ByteId::Or        => 4u8,
			ByteId::Not       => 5u8,
			ByteId::Aeq       => 6u8,
			ByteId::Aneq      => 7u8,
			ByteId::Lt        => 8u8,
			ByteId::Lte       => 9u8,
			ByteId::Gt        => 10u8,
			ByteId::Gte       => 11u8,
			ByteId::Var       => 12u8,
			ByteId::FnCall    => 13u8,
		}
	}

	fn from_byte(b : &u8) -> Result<ByteId, String>
	{
		match b
		{
			0u8 => Result::Ok(ByteId::BoolConst),
			1u8 => Result::Ok(ByteId::Beq),
			2u8 => Result::Ok(ByteId::Bneq),
			3u8 => Result::Ok(ByteId::And),
			4u8 => Result::Ok(ByteId::Or),
			5u8 => Result::Ok(ByteId::Not),
			6u8 => Result::Ok(ByteId::Aeq),
			7u8 => Result::Ok(ByteId::Aneq),
			8u8 => Result::Ok(ByteId::Lt),
			9u8 => Result::Ok(ByteId::Lte),
			10u8 => Result::Ok(ByteId::Gt),
			11u8 => Result::Ok(ByteId::Gte),
			12u8 => Result::Ok(ByteId::Var),
			13u8 => Result::Ok(ByteId::FnCall),
			_   => Result::Err(format!("{}", "Unrecognized type ID from byte for Bexp."))
		}
	}
}
