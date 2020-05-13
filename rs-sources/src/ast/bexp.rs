use std::fmt;

pub enum Bexp
{
	/* T / F */  BoolConst {v :  bool},
	/* == */     Beq       {l : Box<Bexp>, r : Box<Bexp>},
	/* != */     Bneq      {l : Box<Bexp>, r : Box<Bexp>},
	/* && */     And       {l : Box<Bexp>, r : Box<Bexp>},
	/* || */     Or        {l : Box<Bexp>, r : Box<Bexp>},
	/* == */     Aeq       {l : Box<super::aexp::Aexp>, r : Box<super::aexp::Aexp>},
	/* != */     Aneq      {l : Box<super::aexp::Aexp>, r : Box<super::aexp::Aexp>},
	/* <  */     Lt        {l : Box<super::aexp::Aexp>, r : Box<super::aexp::Aexp>},
	/* <= */     Lte       {l : Box<super::aexp::Aexp>, r : Box<super::aexp::Aexp>},
	/* >  */     Gt        {l : Box<super::aexp::Aexp>, r : Box<super::aexp::Aexp>},
	/* >= */     Gte       {l : Box<super::aexp::Aexp>, r : Box<super::aexp::Aexp>},
	/* x  */     Var       {v : super::var_general::VarRef},
	/* foo() */  FnCall    {fc : super::func_general::FnCall},
}

pub mod constructor_helper
{
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
