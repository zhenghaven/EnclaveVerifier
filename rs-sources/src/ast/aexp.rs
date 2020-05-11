pub enum Aexp
{
	IntConst {v :  i32},
	FloConst {v :  f32},
	Add {l : Box<Aexp>, r : Box<Aexp>},
	Sub {l : Box<Aexp>, r : Box<Aexp>},
	Mul {l : Box<Aexp>, r : Box<Aexp>},
	Div {l : Box<Aexp>, r : Box<Aexp>},
	Mod {l : Box<Aexp>, r : Box<Aexp>},
}

pub mod constructor_helper
{
	use std::ops;

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

fn int_const_to_string(v : &i32) -> String
{
	format!("{}", v).to_string()
}

fn float_const_to_string(v : &f32) -> String
{
	format!("{}", v).to_string()
}

fn add_exp_to_string(l : &Aexp, r : &Aexp) -> String
{
	let mut left_string = to_string(l);
	left_string.push_str(" + ");
	left_string.push_str(&to_string(r));

	left_string
}

fn sub_exp_to_string(l : &Aexp, r : &Aexp) -> String
{
	let mut left_string = to_string(l);
	left_string.push_str(" - ");
	left_string.push_str(&to_string(r));

	left_string
}

fn mul_exp_to_string(l : &Aexp, r : &Aexp) -> String
{
	let mut left_string = to_string(l);
	left_string.push_str(" * ");
	left_string.push_str(&to_string(r));

	left_string
}

fn div_exp_to_string(l : &Aexp, r : &Aexp) -> String
{
	let mut left_string = to_string(l);
	left_string.push_str(" / ");
	left_string.push_str(&to_string(r));

	left_string
}

fn mod_exp_to_string(l : &Aexp, r : &Aexp) -> String
{
	let mut left_string = to_string(l);
	left_string.push_str(" % ");
	left_string.push_str(&to_string(r));

	left_string
}

pub fn to_string(aexp : &Aexp) -> String
{
	match aexp
	{
		Aexp::IntConst{v} => int_const_to_string(v),
		Aexp::FloConst{v} => float_const_to_string(v),
		Aexp::Add{l, r} => add_exp_to_string(l, r),
		Aexp::Sub{l, r} => sub_exp_to_string(l, r),
		Aexp::Mul{l, r} => mul_exp_to_string(l, r),
		Aexp::Div{l, r} => div_exp_to_string(l, r),
		Aexp::Mod{l, r} => mod_exp_to_string(l, r),
	}
}
