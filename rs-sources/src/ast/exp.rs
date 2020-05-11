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
