use std::rc::Rc;
use std::cell::RefCell;
use std::string::String;

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
		var_states  : & mut Rc<RefCell<VarStatesStack<ExpValue, VarState> > >)
		-> Result<Option<Option<ExpValue> >, String>;
}

impl CanEvalToExpVal for cmd::Cmd
{
	fn eval_to_exp_val(
		&self,
		func_states : & mut Rc<FuncStatesStack<FuncState> >,
		var_states  : & mut Rc<RefCell<VarStatesStack<ExpValue, VarState> > >)
		-> Result<Option<Option<ExpValue> >, String>
	{
		use cmd::Cmd;
		use exp::CanEvalToExpVal;
		use bexp::CanEvalToBexpVal;

		// println!("[DEBUG]: Executing cmd: {}", self);

		match self
		{
			Cmd::Skip                              => {},
			Cmd::VarDecl  { d }                    =>
			{
				{
					match var_states.borrow_mut().decl_var((**d).clone())
					{
						Option::None         => {},
						Option::Some(ret_decl) =>
							return Result::Err(format!("Failed to declare variable {}; It probably already declared at current stack.", ret_decl.name)),
					}
				}
			},
			Cmd::Assign   { var, e }               =>
			{
				let e_val = e.eval_to_exp_val(func_states, var_states)?;

				{
					let assi_ret = var_states.borrow_mut().var_assign(&var.name, e_val);
					match assi_ret
					{
						Result::Ok(assi_res) => assi_res?,
						Result::Err(_)       =>
							return Result::Err(format!("Failed to assign: Variable {} hasn't been declared.", var.name)),
					}
				}
			},
			Cmd::FnCall   { fc }                   =>
			{
				states::func_call(func_states, var_states, fc, true)?;
			},
			Cmd::IfElse   { cond, tr_cmd, fa_cmd } =>
			{
				let cond_val = cond.eval_to_bexp_val(func_states, var_states)?;

				if cond_val
				{
					let mut inner_func_states = Rc::new(FuncStatesStack::new_level(func_states.clone()));
					let mut inner_var_states  = Rc::new(RefCell::new(VarStatesStack::new_level(var_states.clone())));

					return tr_cmd.eval_to_exp_val(&mut inner_func_states, &mut inner_var_states)
				}
				else
				{
					let mut inner_func_states = Rc::new(FuncStatesStack::new_level(func_states.clone()));
					let mut inner_var_states  = Rc::new(RefCell::new(VarStatesStack::new_level(var_states.clone())));

					return fa_cmd.eval_to_exp_val(&mut inner_func_states, &mut inner_var_states)
				}
			},
			Cmd::WhileLoop{ cond, lp_cmd }         =>
			{
				let mut cond_val = cond.eval_to_bexp_val(func_states, var_states)?;

				// {
				// 	println!("[DEBUG]: Original Var states:\n{}\n-----END-----", var_states.borrow());
				// }

				while cond_val
				{
					let mut inner_func_states = Rc::new(FuncStatesStack::new_level(func_states.clone()));
					let mut inner_var_states  = Rc::new(RefCell::new(VarStatesStack::new_level(var_states.clone())));

					// {
					// 	println!("[DEBUG]: New Var states:\n{}\n-----END-----", inner_var_states.borrow());
					// }

					match lp_cmd.eval_to_exp_val(&mut inner_func_states, &mut inner_var_states)?
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

				match func_states_ref.decl_fn(prototype.clone(), fn_cmd.clone())
				{
					Option::None            => {},
					Option::Some((ret_pt, _)) =>
						return Result::Err(format!("Function named {} has already been declared.", ret_pt.name)),
				}
			},
			Cmd::Return   { e }                    =>
			{
				match e
				{
					Option::Some(e_v) => return Result::Ok(Option::Some(Option::Some(e_v.eval_to_exp_val(func_states, var_states)?))),
					Option::None      => return Result::Ok(Option::Some(Option::None)),
				}

			},
		}

		return Result::Ok(Option::None)
	}
}
