use std::fmt;

pub enum Exp
{
	A {e : super::aexp::Aexp},
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