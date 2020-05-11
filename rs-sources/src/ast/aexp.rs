use std::fmt;

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

pub mod constructor_helper
{
	use std::ops;

	//Helper for constant types:

	pub trait ConstType
	{
		fn to_aexp(self) -> super::Aexp;
	}

	impl ConstType for i32
	{
		fn to_aexp(self) -> super::Aexp
		{
			super::Aexp::IntConst{v : self}
		}
	}

	impl ConstType for f32
	{
		fn to_aexp(self) -> super::Aexp
		{
			super::Aexp::FloConst{v : self}
		}
	}

	//Helper for Variables:

	pub trait Var
	{
		fn to_aexp(&self) -> super::Aexp;
	}

	impl Var for str
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
