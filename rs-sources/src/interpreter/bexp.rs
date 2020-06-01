use std::rc::Rc;
use std::string::String;

use super::super::ast::bexp;
use super::super::ast::states::FuncStatesStack;
use super::super::ast::states::VarStatesStack;

use super::exp::ExpValue;
use super::states;
use super::states::FuncState;
use super::states::VarState;

impl super::exp::CanConvertToExpVal for bool
{
	fn to_exp_val(self) -> super::exp::ExpValue
	{
		super::exp::ExpValue::B(self)
	}
}

pub trait CanEvalToBexpVal
{
	fn eval_to_bexp_val(
		&self,
		func_states : & Rc<FuncStatesStack<FuncState> >,
		var_states  : & Rc<VarStatesStack<ExpValue, VarState> >)
		-> Result<bool, String>;

	fn simp_eval_to_bexp_val(&self) -> Result<bool, String>;
}

impl CanEvalToBexpVal for bexp::Bexp
{
	fn eval_to_bexp_val(
		&self,
		func_states : & Rc<FuncStatesStack<FuncState> >,
		var_states  : & Rc<VarStatesStack<ExpValue, VarState> >)
		-> Result<bool, String>
	{
		use bexp::Bexp;
		use super::aexp::CanEvalToAexpVal;

		match self
		{
			Bexp::BoolConst{ v } => Result::Ok(v.clone()),
			Bexp::Beq { l, r }   =>
			{
				let l_val = l.eval_to_bexp_val(func_states, var_states)?;
				let r_val = r.eval_to_bexp_val(func_states, var_states)?;

				Result::Ok(l_val == r_val)
			},
			Bexp::Bneq{ l, r }   =>
			{
				let l_val = l.eval_to_bexp_val(func_states, var_states)?;
				let r_val = r.eval_to_bexp_val(func_states, var_states)?;

				Result::Ok(l_val != r_val)
			},
			Bexp::And { l, r }   =>
			{
				let l_val = l.eval_to_bexp_val(func_states, var_states)?;
				let r_val = r.eval_to_bexp_val(func_states, var_states)?;

				Result::Ok(l_val && r_val)
			},
			Bexp::Or  { l, r }   =>
			{
				let l_val = l.eval_to_bexp_val(func_states, var_states)?;
				let r_val = r.eval_to_bexp_val(func_states, var_states)?;

				Result::Ok(l_val || r_val)
			},
			Bexp::Not { e }      =>
			{
				let val = e.eval_to_bexp_val(func_states, var_states)?;

				Result::Ok(!val)
			}
			Bexp::Aeq { l, r }   =>
			{
				let l_val = l.eval_to_aexp_val(func_states, var_states)?;
				let r_val = r.eval_to_aexp_val(func_states, var_states)?;

				Result::Ok(l_val == r_val)
			},
			Bexp::Aneq{ l, r }   =>
			{
				let l_val = l.eval_to_aexp_val(func_states, var_states)?;
				let r_val = r.eval_to_aexp_val(func_states, var_states)?;

				Result::Ok(l_val != r_val)
			},
			Bexp::Lt  { l, r }   =>
			{
				let l_val = l.eval_to_aexp_val(func_states, var_states)?;
				let r_val = r.eval_to_aexp_val(func_states, var_states)?;

				Result::Ok(l_val < r_val)
			},
			Bexp::Lte { l, r }   =>
			{
				let l_val = l.eval_to_aexp_val(func_states, var_states)?;
				let r_val = r.eval_to_aexp_val(func_states, var_states)?;

				Result::Ok(l_val <= r_val)
			},
			Bexp::Gt  { l, r }   =>
			{
				let l_val = l.eval_to_aexp_val(func_states, var_states)?;
				let r_val = r.eval_to_aexp_val(func_states, var_states)?;

				Result::Ok(l_val > r_val)
			},
			Bexp::Gte { l, r }   =>
			{
				let l_val = l.eval_to_aexp_val(func_states, var_states)?;
				let r_val = r.eval_to_aexp_val(func_states, var_states)?;

				Result::Ok(l_val >= r_val)
			},
			Bexp::Var { v }      =>
			{
				let var_opt = var_states.var_read(&v.name);
				match var_opt
				{
					Option::Some(var) =>
						match var
						{
							Option::Some(e_val) => e_val.to_bexp_val(),
							Option::None        => Result::Err(format!("Variable {} hasn't been initialized", v.name)),
						},
					Option::None      => Result::Err(format!("Variable {} hasn't been declared", v.name))
				}
			},
			Bexp::FnCall{ fc }   =>
			{
				let func_call_res = states::func_call(func_states, var_states, fc, true)?;
				match func_call_res
				{
					Option::Some(ret_val) => ret_val.to_bexp_val(),
					Option::None          => Result::Err(format!("Function {} doesn't return a value", fc.name))
				}
			},
		}
	}

	fn simp_eval_to_bexp_val(&self) -> Result<bool, String>
	{
		use bexp::Bexp;

		match self
		{
			Bexp::BoolConst{ v } => Result::Ok(v.clone()),
			_                    => Result::Err(format!("Expecting an evaluted BExp, while an un-evaluated BExp is given."))
		}
	}
}

