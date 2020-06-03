use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::string::String;
use std::string::ToString;
use std::option::Option;
use std::collections::HashMap;
use std::marker::PhantomData;

pub trait AnyVariable<ValueType>
{
	fn from_decl(decl : super::var_general::VarDecl) -> Self;

	fn assign(&mut self, e : ValueType) -> Result<(), String>;

	fn read(&self) -> Option<ValueType>;

	fn get_type(&self) -> super::data_type::DataType;

	fn to_decl(self, name : String) -> super::var_general::VarDecl;
}

pub trait AnyFunc
{
	fn from_decl(pt : Rc<super::func_general::FnProtoType>, cmd : Rc<super::cmd::Cmd>) -> Self;

	fn to_decl(self) -> (Rc<super::func_general::FnProtoType>, Rc<super::cmd::Cmd>);

	fn get_prototype_ref(&self) -> &super::func_general::FnProtoType;

	fn get_cmd_ref(&self) -> &super::cmd::Cmd;
}

pub struct FuncStates<T : fmt::Display + AnyFunc >
{
	map : HashMap<String, T>,
}

impl<T : fmt::Display + AnyFunc> FuncStates<T>
{
	pub fn new() -> FuncStates<T>
	{
		FuncStates { map : HashMap::new() }
	}

	pub fn decl(&mut self, pt : Rc<super::func_general::FnProtoType>, cmd : Rc<super::cmd::Cmd>) -> Option<(Rc<super::func_general::FnProtoType>, Rc<super::cmd::Cmd>)>
	{
		let mut mangled_fun_name = String::new();
		mangled_fun_name.push_str(&pt.name);
		mangled_fun_name.push('_');

		for func_param_decl in pt.var_decl_list.iter()
		{
			mangled_fun_name.push_str(&func_param_decl.var_type.to_string());
			mangled_fun_name.push('_');
		}

		if !self.map.contains_key(&mangled_fun_name)
		{
			match self.map.insert(mangled_fun_name.clone(), T::from_decl(pt, cmd))
			{
				Option::None => Option::None,
				Option::Some(_) => // An error that should not happen
				{
					panic!("Failed to insert a func whose key is not contained in the HashMap.")
				},
			}

		}
		else
		{
			Option::Some((pt, cmd))
		}
	}

	pub fn get_fn(&self, name : &String) -> Option<&T>
	{
		self.map.get(name)
	}

	pub fn has_func(&self, name : &String) -> bool
	{
		match self.map.get(name)
		{
			Option::Some(_) => true,
			Option::None    => false,
		}
	}
}

impl<T : fmt::Display + AnyFunc> fmt::Display for FuncStates<T>
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		let pad = "                  ";

		for (name, item) in &self.map
		{
			if name.len() > pad.len()
			{
				write!(f, "{}...\t--\t{}\n", &name[0..pad.len()], item)?;
			}
			else
			{
				write!(f, "{}{}\t--\t{}\n", name, &pad[0..(pad.len() - name.len())], item)?;
			}

		}

		write!(f, "")
	}
}

pub struct VarStates<ValueType, T : fmt::Display + AnyVariable<ValueType> >
{
	map : HashMap<String, T>,
	val_stored_type : PhantomData<ValueType>,
}

impl<ValueType, T : fmt::Display + AnyVariable<ValueType> > VarStates<ValueType, T>
{
	pub fn new() -> VarStates<ValueType, T>
	{
		VarStates { map : HashMap::new(), val_stored_type : PhantomData }
	}

	pub fn decl(&mut self, decl : super::var_general::VarDecl) -> Option<super::var_general::VarDecl>
	{
		let var_name = decl.name.clone();

		if !self.map.contains_key(&var_name)
		{
			match self.map.insert(var_name.clone(), T::from_decl(decl))
			{
				Option::None => Option::None,
				Option::Some(_) => // An error that should not happen
				{
					panic!("Failed to insert a val whose key is not contained in the HashMap.")
				}
			}

		}
		else
		{
			Option::Some(decl)
		}
	}

	pub fn assign(&mut self, name : &String, v : ValueType) -> Result<Result<(), String>, ValueType>
	{
		match self.map.get_mut(name)
		{
			Option::Some(var_state) => Result::Ok(var_state.assign(v)),
			Option::None            => Result::Err(v),
		}
	}

	pub fn read(&self, name : &String) -> Option<Option<ValueType> >
	{
		match self.map.get(name)
		{
			Option::Some(var_state) => Option::Some(var_state.read()),
			Option::None            => Option::None,
		}
	}

	pub fn get_type(&self, name : &String) -> Option<super::data_type::DataType>
	{
		match self.map.get(name)
		{
			Option::Some(var_state) => Option::Some(var_state.get_type()),
			Option::None            => Option::None,
		}
	}

	pub fn has_var(&self, name : &String) -> bool
	{
		match self.map.get(name)
		{
			Option::Some(_) => true,
			Option::None    => false,
		}
	}
}

impl<ValueType, T : fmt::Display + AnyVariable<ValueType> > fmt::Display for VarStates<ValueType, T>
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		let pad = "             ";

		for (name, item) in &self.map
		{
			if name.len() > pad.len()
			{
				write!(f, "{}...\t--\t{}\n", &name[0..pad.len()], item)?;
			}
			else
			{
				write!(f, "{}{}\t--\t{}\n", name, &pad[0..(pad.len() - name.len())], item)?;
			}

		}

		write!(f, "")
	}
}

pub struct FuncStatesStack<FnStateType : fmt::Display + AnyFunc >
{
	pub parent : Option<Rc<FuncStatesStack<FnStateType> > >,
	pub state : FuncStates<FnStateType>,
	level_idx : usize,
}

impl<FnStateType : fmt::Display + AnyFunc >
FuncStatesStack<FnStateType>
{
	pub fn new() -> FuncStatesStack<FnStateType>
	{
		FuncStatesStack { parent : Option::None, state : FuncStates::new(), level_idx : 0 }
	}

	pub fn new_level(curr : Rc<FuncStatesStack<FnStateType> >) -> FuncStatesStack<FnStateType>
	{
		let new_level_idx = curr.level_idx + 1;
		FuncStatesStack { parent : Option::Some(curr), state : FuncStates::new(), level_idx : new_level_idx }
	}

	pub fn decl_fn(&mut self, pt : Rc<super::func_general::FnProtoType>, cmd : Rc<super::cmd::Cmd>) -> Option<(Rc<super::func_general::FnProtoType>, Rc<super::cmd::Cmd>)>
	{
		self.state.decl(pt, cmd)
	}

	fn search_fn_internal(curr : &Rc<FuncStatesStack<FnStateType> >, name : &String, level : usize) -> Option<(Rc<FuncStatesStack<FnStateType> >, usize)>
	{
		match curr.state.get_fn(name)
		{
			Option::Some(_) => Option::Some((curr.clone(), level)),
			Option::None    => match &curr.parent
				{
					Option::Some(p) => Self::search_fn_internal(&p, name, level + 1),
					Option::None    => Option::None,
				},
		}
	}

	pub fn search_fn(curr : &Rc<FuncStatesStack<FnStateType> >, name : &String) -> Option<(Rc<FuncStatesStack<FnStateType> >, usize)>
	{
		Self::search_fn_internal(curr, name, 0)
	}

	pub fn get_fn_at_curr_level(&self, name : &String) -> Option<&FnStateType>
	{
		match self.state.get_fn(name)
		{
			Option::Some(res_fn) => Option::Some(res_fn),
			Option::None         => Option::None,
		}
	}
}

impl<FnStateType : fmt::Display + AnyFunc >
fmt::Display for FuncStatesStack<FnStateType>
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match &self.parent
		{
			Option::Some(parent) => { write!(f, "{}", parent)?; },
			Option::None         => {},
		}

		write!(f, "Function States (level={}):\n", self.level_idx)?;
		println!("-----------");
		write!(f, "{}", self.state)
	}
}

pub struct VarStatesStack<ValueType, VarStateType : fmt::Display + AnyVariable<ValueType> >
{
	parent : Option<Rc<RefCell<VarStatesStack<ValueType, VarStateType> > > >,
	pub state : VarStates<ValueType, VarStateType>,
	level_idx : usize,
}

impl<ValueType, VarStateType : fmt::Display + AnyVariable<ValueType> >
VarStatesStack<ValueType, VarStateType>
	where ValueType : std::fmt::Debug
{
	pub fn new() -> VarStatesStack<ValueType, VarStateType>
	{
		VarStatesStack { parent : Option::None, state : VarStates::new(), level_idx : 0 }
	}

	pub fn new_level(curr : Rc<RefCell<VarStatesStack<ValueType, VarStateType> > >) -> VarStatesStack<ValueType, VarStateType>
	{
		let new_level_idx = curr.borrow().level_idx + 1;
		VarStatesStack { parent : Option::Some(curr), state : VarStates::new(), level_idx : new_level_idx }
	}

	pub fn decl_var(&mut self, decl : super::var_general::VarDecl) -> Option<super::var_general::VarDecl>
	{
		self.state.decl(decl)
	}

	pub fn var_assign(&mut self, name : &String, v : ValueType) -> Result<Result<(), String>, ValueType>
	{
		match self.state.assign(name, v)
		{
			Result::Ok (res_v) => Result::Ok(res_v),
			Result::Err(ret_v) => match &mut self.parent
			{
				Option::Some(p) => p.borrow_mut().var_assign(name, ret_v),
				Option::None    => Result::Err(ret_v)
			},
		}
	}

	pub fn var_read(&self, name : &String) -> Option<Option<ValueType> >
	{
		//println!("[DEBUG]: Searching Var: {}", name);
		match self.state.read(name)
		{
			Option::Some(res_v) => Option::Some(res_v),
			Option::None        => match &self.parent
			{
				Option::Some(p) => p.borrow().var_read(name),
				Option::None    => Option::None,
			},
		}
	}

	pub fn var_get_type(&self, name : &String) -> Option<super::data_type::DataType>
	{
		match self.state.get_type(name)
		{
			Option::Some(res_v) => Option::Some(res_v),
			Option::None        => match &self.parent
			{
				Option::Some(p) => p.borrow().var_get_type(name),
				Option::None    => Option::None
			},
		}
	}

	pub fn get_level(curr : &Rc<RefCell<VarStatesStack<ValueType, VarStateType> > >, level : usize) -> Option<Rc<RefCell<VarStatesStack<ValueType, VarStateType> > > >
	{
		if level == 0
		{
			Option::Some(curr.clone())
		}
		else
		{
			match &curr.borrow().parent
			{
				Option::Some(p) => Self::get_level(p, level - 1),
				Option::None    => Option::None,
			}
		}
	}
}

impl<ValueType, VarStateType : fmt::Display + AnyVariable<ValueType> >
fmt::Display for VarStatesStack<ValueType, VarStateType>
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match &self.parent
		{
			Option::Some(parent) => { write!(f, "{}", parent.borrow())?; },
			Option::None         => {},
		}

		write!(f, "Variable States (level={}):\n", self.level_idx)?;
		println!("-----------");
		write!(f, "{}", self.state)
	}
}
