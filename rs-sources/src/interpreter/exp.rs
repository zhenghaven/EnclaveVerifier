use std::fmt;
use std::rc::Rc;
use std::string::String;

use super::super::ast::exp;
use super::super::ast::data_type;
use super::super::ast::states::FuncStatesStack;
use super::super::ast::states::VarStatesStack;

use super::states::FuncState;
use super::states::VarState;

#[derive(Clone)]
pub enum ExpValue
{
	A(super::aexp::AexpValue),
	B(bool)
}

impl ExpValue
{
	pub fn from_aexp_val(val : super::aexp::AexpValue) -> ExpValue
	{
		ExpValue::A(val)
	}

	pub fn from_bexp_val(val : bool) -> ExpValue
	{
		ExpValue::B(val)
	}

	pub fn to_exp(self) -> exp::Exp
	{
		use super::super::ast::bexp;
		use exp::constructor_helper::ToExp;
		use bexp::constructor_helper::ToBexp;

		match self
		{
			ExpValue::A(val) => val.to_aexp().to_exp(),
			ExpValue::B(val) => val.to_bexp().to_exp(),
		}
	}

	pub fn get_type(&self) -> data_type::DataType
	{
		match self
		{
			ExpValue::A(val) => val.get_type(),
			ExpValue::B(_)   => data_type::DataType::Bool,
		}
	}

	pub fn to_bexp_val(self) -> Result<bool, String>
	{
		match self
		{
			ExpValue::A(_)   => Result::Err(format!("Expecting an BExp value, while a AExp value is given.")),
			ExpValue::B(val) => Result::Ok(val),
		}
	}
}

impl super::aexp::CanConvertToAexpVal for ExpValue
{
	fn to_aexp_val(self) -> Result<super::aexp::AexpValue, String>
	{
		match self
		{
			super::exp::ExpValue::A(a_val) => Result::Ok(a_val),
			super::exp::ExpValue::B(_)     => Result::Err(format!("Expecting an AExp value, while a BExp value is given.")),
		}
	}
}

impl fmt::Display for ExpValue
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match &self
		{
			ExpValue::A(val) => write!(f, "{}", val),
			ExpValue::B(val) => write!(f, "{}", val),
		}
	}
}

pub trait CanConvertToExpVal
{
	fn to_exp_val(self) -> ExpValue;
}

pub trait CanEvalToExpVal
{
	fn eval_to_exp_val(
		&self,
		func_states : & Rc<FuncStatesStack<FuncState> >,
		var_states  : & Rc<VarStatesStack<ExpValue, VarState> >)
		-> Result<ExpValue, String>;

	fn simp_eval_to_exp_val(&self) -> Result<ExpValue, String>;
}

impl CanEvalToExpVal for exp::Exp
{
	fn eval_to_exp_val(
		&self,
		func_states : & Rc<FuncStatesStack<FuncState> >,
		var_states  : & Rc<VarStatesStack<ExpValue, VarState> >)
		-> Result<ExpValue, String>
	{
		use super::aexp::CanEvalToAexpVal;
		use super::bexp::CanEvalToBexpVal;

		match self
		{
			exp::Exp::A { e } => Result::Ok((e.eval_to_aexp_val(func_states, var_states)?).to_exp_val()),
			exp::Exp::B { e } => Result::Ok((e.eval_to_bexp_val(func_states, var_states)?).to_exp_val()),
		}
	}

	fn simp_eval_to_exp_val(&self) -> Result<ExpValue, String>
	{
		use super::aexp::CanEvalToAexpVal;
		use super::bexp::CanEvalToBexpVal;

		match self
		{
			exp::Exp::A { e } => Result::Ok((e.simp_eval_to_aexp_val()?).to_exp_val()),
			exp::Exp::B { e } => Result::Ok((e.simp_eval_to_bexp_val()?).to_exp_val()),
		}
	}
}
