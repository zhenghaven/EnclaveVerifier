use std::fmt;
use std::string::String;
use std::vec::Vec;
use std::rc::Rc;

use super::super::ast;
use ast::func_general;
use ast::data_type;
use ast::var_general;
use ast::states::AnyFunc;
use ast::states::AnyVariable;
use ast::states::FuncStatesStack;
use ast::states::VarStatesStack;
use ast::cmd;

use super::exp::ExpValue;

#[derive(Clone)]
pub struct FuncState
{
	f_pt : Rc<ast::func_general::FnProtoType>,
	cmd  : Rc<ast::cmd::Cmd>,
}

impl FuncState
{

	fn func_call(
		&self,
		func_defined_func_states  : Rc<FuncStatesStack<FuncState> >,
		func_defined_var_states   : Rc<VarStatesStack<ExpValue, VarState> >,
		caller_func_states        : &Rc<FuncStatesStack<FuncState> >,
		caller_defined_var_states : &Rc<VarStatesStack<ExpValue, VarState> >,
		call        : &    func_general::FnCall)
		-> Result<Option<ExpValue>, String>
	{
		use super::exp::CanEvalToExpVal;

		let func_pt = self.get_prototype_ref();

		if func_pt.var_decl_list.len() == call.exp_list.len()
		{
			let mut val_list : Vec<ExpValue> = Vec::new();
			val_list.reserve(call.exp_list.len());

			for e in call.exp_list.iter()
			{
				val_list.push(e.eval_to_exp_val(caller_func_states, caller_defined_var_states)?);
			}

			self.func_call_by_vals(func_defined_func_states, func_defined_var_states, val_list)
		}
		else
		{
			Result::Err(
				format!(
					"Function {} expects {} parameters, but {} are given.",
					func_pt.name, func_pt.var_decl_list.len(), call.exp_list.len()
				)
			)
		}

	}

	fn func_call_by_vals(
		&self,
		func_defined_func_states  : Rc<FuncStatesStack<FuncState> >,
		func_defined_var_states   : Rc<VarStatesStack<ExpValue, VarState> >,
		val_list    : Vec<ExpValue>) -> Result<Option<ExpValue>, String>
	{
		use super::cmd::CanEvalToExpVal;

		let func_pt = self.get_prototype_ref();

		let mut callee_func_states = Rc::new(FuncStatesStack::new_level(func_defined_func_states));
		//let     callee_func_states = Rc::new(FuncStatesStack::new_level(func_defined_func_states));
		let mut callee_var_states  = Rc::new(VarStatesStack::new_level(func_defined_var_states));

		/*
		let callee_func_states_ref = match Rc::get_mut(&mut callee_func_states)
		{
			Some(v) => v,
			None    => return Result::Err(format!("Failed to unwrap the RC."))
		};
		*/

		let callee_var_states_ref = match Rc::get_mut(&mut callee_var_states)
		{
			Some(v) => v,
			None    => return Result::Err(format!("Failed to unwrap the RC."))
		};

		if func_pt.var_decl_list.len() == val_list.len()
		{
			for (var_decl, val) in func_pt.var_decl_list.iter().zip(val_list.into_iter())
			{
				match callee_var_states_ref.decl_var(var_decl.clone())
				{
					Option::Some(ret_decl) =>
						return Result::Err(
							format!("Function parameter {} is declared repeatedly.", ret_decl.name)),
					Option::None           => {},
				}

				match callee_var_states_ref.var_assign(&var_decl.name, val)
				{
					Result::Ok(a_res) => a_res?,
					Result::Err(_) => return Result::Err(
						format!("Cann't find function parameter {} that just declared.", var_decl.name))
				}
			}

			let func_cmd = self.get_cmd_ref();

			match func_cmd.eval_to_exp_val(&mut callee_func_states, &mut callee_var_states)?
			{
				Option::Some(v) => //Commands in the function returns void or something:
					Result::Ok(v),
				Option::None    => //Commands in the function doesn't have return statement. That's fine, it's just a void function
					Result::Ok(Option::None),
			}
		}
		else
		{
			Result::Err(
				format!(
					"Function {} expects {} parameters, but {} are given.",
					func_pt.name, func_pt.var_decl_list.len(), val_list.len()
				)
			)
		}
	}
}

impl AnyFunc for FuncState
{
	fn get_prototype_ref(&self) -> &func_general::FnProtoType
	{
		&self.f_pt
	}

	fn get_cmd_ref(&self) -> &cmd::Cmd
	{
		&self.cmd
	}

	fn from_decl(pt : Rc<func_general::FnProtoType>, cmd : Rc<cmd::Cmd>) -> FuncState
	{
		FuncState { f_pt : pt, cmd : cmd }
	}

	fn to_decl(self) -> (Rc<func_general::FnProtoType>, Rc<cmd::Cmd>)
	{
		(self.f_pt, self.cmd)
	}
}

pub fn func_call(
	func_states : & Rc<FuncStatesStack<FuncState> >,
	var_states  : & Rc<VarStatesStack<ExpValue, VarState> >,
	call        : & func_general::FnCall)
	-> Result<Option<ExpValue>, String>
{
	let callee_opt = FuncStatesStack::search_fn(func_states, &call.name);
	let (func_defined_func_states, func_defined_level) = match callee_opt
	{
		Option::Some(v) => v,
		Option::None    => return Result::Err(format!("The function {} called is undefined.", call.name)),
	};

	let func_defined_func_states_2 = func_defined_func_states.clone();
	let callee = match func_defined_func_states_2.get_fn_at_curr_level(&call.name)
	{
		Option::Some(v) => v,
		Option::None    => return Result::Err(format!("The function {} called is undefined.", call.name)),
	};

	let func_defined_var_states = match VarStatesStack::get_level(var_states, func_defined_level)
	{
		Option::Some(v) => v,
		Option::None    => return Result::Err(format!("Func states stack and var states stack mismatch."))
	};

	callee.func_call(func_defined_func_states, func_defined_var_states, func_states, var_states, call)
}

impl fmt::Display for FuncState
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f, "{}", self.f_pt)
	}
}




pub struct VarState
{
	pub s : Option<ExpValue>,
	pub t : data_type::DataType,
}

impl AnyVariable<ExpValue> for VarState
{
	fn from_decl(decl : var_general::VarDecl) -> VarState
	{
		VarState { s : Option::None, t : decl.var_type }
	}

	fn to_decl(self, name : String) -> var_general::VarDecl
	{
		var_general::VarDecl{ var_type : self.t, name : name }
	}

	fn assign(&mut self, v : ExpValue) -> Result<(), String>
	{
		if v.get_type() == self.t
		{
			self.s = Option::Some(v);

			Result::Ok(())
		}
		else
		{
			Result::Err(format!("Assignment expecting {} type, but {} type is given.", self.t, v.get_type()))
		}

	}

	fn read(&self) -> Option<ExpValue>
	{
		return self.s.clone()
	}

	fn get_type(&self) -> data_type::DataType
	{
		return self.t.clone()
	}
}

impl fmt::Display for VarState
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f, "{}\t-\t", self.t)?;
		match &self.s
		{
			Option::None     => write!(f, "N/A"),
			Option::Some(v)  => write!(f, "{}", v),
		}
	}
}
