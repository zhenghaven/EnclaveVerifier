use std::rc::Rc;

use super::super::ast::cmd;
use super::super::ast::states::FuncStatesStack;
use super::super::ast::states::VarStatesStack;

use super::exp::ExpValue;
use super::states::FuncState;
use super::states::VarState;

use super::exp;
use super::bexp;
use super::states;

pub trait CanEvalToExpVal
{
	fn eval_to_exp_val(
		&self,
		func_states : & mut Rc<FuncStatesStack<FuncState> >,
		var_states  : & mut Rc<VarStatesStack<ExpValue, VarState> >)
		-> Result<Option<ExpValue>, String>;
}

impl CanEvalToExpVal for cmd::Cmd
{
	fn eval_to_exp_val(
		&self,
		func_states : & mut Rc<FuncStatesStack<FuncState> >,
		var_states  : & mut Rc<VarStatesStack<ExpValue, VarState> >)
		-> Result<Option<ExpValue>, String>
	{
		use cmd::Cmd;
		use exp::CanEvalToExpVal;
		use bexp::CanEvalToBexpVal;

		match self
		{
			Cmd::Skip                              => {},
			Cmd::VarDecl  { d }                    =>
			{
				let var_states_ref = match Rc::get_mut(var_states)
				{
					Some(v) => v,
					None    => return Result::Err(format!("Failed to unwrap the RC."))
				};

				match var_states_ref.decl_var((**d).clone())
				{
					Option::None         => {},
					Option::Some(ret_decl) =>
						return Result::Err(format!("Failed to declare variable {}; It probably already declared at current stack.", ret_decl.name)),
				}
			},
			Cmd::Assign   { var, e }               =>
			{
				let e_val = e.eval_to_exp_val(func_states, var_states)?;

				let var_states_ref = match Rc::get_mut(var_states)
				{
					Some(v) => v,
					None    => return Result::Err(format!("Failed to unwrap the RC."))
				};

				let assi_ret = var_states_ref.var_assign(&var.name, e_val);
				match assi_ret
				{
					Result::Ok(assi_res) => assi_res?,
					Result::Err(_)       =>
						return Result::Err(format!("Variable {} hasn't been declared", var.name)),
				}
			},
			Cmd::FnCall   { fc }                   =>
			{
				states::func_call(func_states, var_states, fc)?;
			},
			Cmd::IfElse   { cond, tr_cmd, fa_cmd } =>
			{
				let cond_val = cond.eval_to_bexp_val(func_states, var_states)?;

				if cond_val
				{
					return tr_cmd.eval_to_exp_val(func_states, var_states)
				}
				else
				{
					return fa_cmd.eval_to_exp_val(func_states, var_states)
				}
			},
			Cmd::WhileLoop{ cond, lp_cmd }         =>
			{
				let mut cond_val = cond.eval_to_bexp_val(func_states, var_states)?;

				while cond_val
				{
					match lp_cmd.eval_to_exp_val(func_states, var_states)?
					{
						Option::None    => {},
						Option::Some(v) => { return Result::Ok(Option::Some(v)) },
					}

					cond_val = cond.eval_to_bexp_val(func_states, var_states)?;
				}
			},
			Cmd::Seq      { fst_cmd, snd_cmd }     =>
			{
				match fst_cmd.eval_to_exp_val(func_states, var_states)?
				{
					Option::Some(ret_val) => return Result::Ok(Option::Some(ret_val)),
					Option::None          =>
					{
						return snd_cmd.eval_to_exp_val(func_states, var_states)
					}
				}
			},
			Cmd::FnDecl   { prototype, fn_cmd }    =>
			{
				let func_states_ref = match Rc::get_mut(func_states)
				{
					Some(v) => v,
					None    => return Result::Err(format!("Failed to unwrap the RC."))
				};

				match func_states_ref.decl_fn((**prototype).clone(), (**fn_cmd).clone())
				{
					Option::None            => {},
					Option::Some((ret_pt, _)) =>
						return Result::Err(format!("Function named {} has already been declared", ret_pt.name)),
				}
			},
			Cmd::Return   { e }                    =>
			{
				return Result::Ok(Option::Some(e.eval_to_exp_val(func_states, var_states)?))
			},
		}

		return Result::Ok(Option::None)
	}
}
