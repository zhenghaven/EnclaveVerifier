use std::fmt;
use std::string::String;
use std::string::ToString;
use std::vec::Vec;
use std::rc::Rc;
use std::cell::RefCell;

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
use super::aexp::CanConvertToAexpVal;

#[derive(Clone)]
pub struct FuncState
{
	f_pt : Rc<ast::func_general::FnProtoType>,
	cmd  : Rc<ast::cmd::Cmd>,
}

impl FuncState
{

	fn func_call_by_vals(
		&self,
		func_defined_func_states  : Rc<FuncStatesStack<FuncState> >,
		func_defined_var_states   : Rc<RefCell<VarStatesStack<ExpValue, VarState> > >,
		val_list    : Vec<ExpValue>) -> Result<Option<ExpValue>, String>
	{
		use super::cmd::CanEvalToExpVal;

		let func_pt = self.get_prototype_ref();

		let mut callee_func_states = Rc::new(FuncStatesStack::new_level(func_defined_func_states));
		let mut callee_var_states  = Rc::new(RefCell::new(VarStatesStack::new_level(func_defined_var_states)));

		if func_pt.var_decl_list.len() == val_list.len()
		{
			for (var_decl, val) in func_pt.var_decl_list.iter().zip(val_list.into_iter())
			{
				match callee_var_states.borrow_mut().decl_var(var_decl.clone())
				{
					Option::Some(ret_decl) =>
						return Result::Err(
							format!("Function parameter {} is declared repeatedly.", ret_decl.name)),
					Option::None           => {},
				}

				match callee_var_states.borrow_mut().var_assign(&var_decl.name, val)
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

impl fmt::Display for FuncState
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f, "{}", self.f_pt)
	}
}

pub fn get_mangled_func_name_from_name_n_exp_val(func_name : &String, exp_val_list : &Vec<ExpValue>) -> String
{
	let mut mangled_fun_name = String::new();
	mangled_fun_name.push_str(func_name);
	mangled_fun_name.push('_');

	for e in exp_val_list.iter()
	{
		mangled_fun_name.push_str(&e.get_type().to_string());
		mangled_fun_name.push('_');
	}

	mangled_fun_name
}

pub fn func_call(
	func_states : & Rc<FuncStatesStack<FuncState> >,
	var_states  : & Rc<RefCell<VarStatesStack<ExpValue, VarState> > >,
	call        : & func_general::FnCall,
	call_allow_com : bool)
	-> Result<Option<ExpValue>, String>
{
	use super::exp::CanEvalToExpVal;

	//println!("[DEBUG]: Making func call {}", call);

	let mut val_list : Vec<ExpValue> = Vec::new();
	val_list.reserve(call.exp_list.len());

	for e in call.exp_list.iter()
	{
		if call_allow_com
		{
			val_list.push(e.eval_to_exp_val(func_states, var_states)?);
		}
		else
		{
			val_list.push(e.simp_eval_to_exp_val()?);
		}
	}

	let mangled_func_name = get_mangled_func_name_from_name_n_exp_val(&call.name, &val_list);

	let callee_opt = FuncStatesStack::search_fn(func_states, &mangled_func_name);
	let (func_defined_func_states, func_defined_level) = match callee_opt
	{
		Option::Some(v) => v,
		Option::None    => return Result::Err(format!("The function {} called is undefined.", mangled_func_name)),
	};

	let func_defined_func_states_2 = func_defined_func_states.clone();
	let callee = match func_defined_func_states_2.get_fn_at_curr_level(&mangled_func_name)
	{
		Option::Some(v) => v,
		Option::None    => return Result::Err(format!("The function {} called is undefined.", mangled_func_name)),
	};

	let func_defined_var_states = match VarStatesStack::get_level(var_states, func_defined_level)
	{
		Option::Some(v) => v,
		Option::None    => return Result::Err(format!("Func states stack and var states stack mismatch."))
	};

	//println!("[DEBUG]: Making func call ... // {} // {}", callee.get_prototype_ref(), mangled_func_name);

	callee.func_call_by_vals(func_defined_func_states, func_defined_var_states, val_list)
}



#[derive(Debug)]
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
		else if v.get_type() == data_type::DataType::Int32 && self.t == data_type::DataType::Float32
		{
			self.s = Option::Some(ExpValue::from_aexp_val((v.to_aexp_val()?).promote_to_flo32()));

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
