use std::fmt;

use std::option::Option;
use std::collections::HashMap;
use std::vec::Vec;

pub trait ToFromFnDecl<T>
{
	fn from_decl(pt : super::func_general::FnProtoType, cmd : super::cmd::Cmd) -> T;

	fn to_decl(self) -> (super::func_general::FnProtoType, super::cmd::Cmd);
}

pub struct FuncState
{
	pub f_pt : super::func_general::FnProtoType,
	pub cmd : super::cmd::Cmd,
}

impl ToFromFnDecl<FuncState> for FuncState
{
	fn from_decl(pt : super::func_general::FnProtoType, cmd : super::cmd::Cmd) -> FuncState
	{
		FuncState { f_pt : pt, cmd : cmd }
	}

	fn to_decl(self) -> (super::func_general::FnProtoType, super::cmd::Cmd)
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

pub struct VarState
{
	pub s : Option<super::exp::Exp>,
	pub t : super::data_type::DataType,
}

pub trait ToFromVarDecl<T>
{
	fn from_decl(decl : super::var_general::VarDecl) -> T;

	fn to_decl(self, name : String) -> super::var_general::VarDecl;
}

impl VarState
{
	pub fn get_mut(&mut self) -> Option<&mut super::exp::Exp>
	{
		match &mut self.s
		{
			Option::None    => Option::None,
			Option::Some(v) => Option::Some(v),
		}
	}

	pub fn get(&self) -> Option<&super::exp::Exp>
	{
		match &self.s
		{
			Option::None    => Option::None,
			Option::Some(v) => Option::Some(v),
		}
	}
}

impl ToFromVarDecl<VarState> for VarState
{
	fn from_decl(decl : super::var_general::VarDecl) -> VarState
	{
		VarState { s : Option::None, t : decl.var_type }
	}

	fn to_decl(self, name : String) -> super::var_general::VarDecl
	{
		super::var_general::VarDecl{ var_type : self.t, name : name }
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

pub struct FuncStates<T : fmt::Display>
{
	map : HashMap<String, T>,
}

impl<T : fmt::Display + ToFromFnDecl<T> > FuncStates<T>
{
	pub fn new() -> FuncStates<T>
	{
		FuncStates { map : HashMap::new() }
	}

	pub fn decl(&mut self, pt : super::func_general::FnProtoType, cmd : super::cmd::Cmd) -> Result<&mut T, (super::func_general::FnProtoType, super::cmd::Cmd)>
	{
		let fun_name = pt.name.clone();

		if !self.map.contains_key(&fun_name)
		{
			match self.map.insert(fun_name.clone(), T::from_decl(pt, cmd))
			{
				Option::None => //inserted successfully
				{
					match self.map.get_mut(&fun_name)
					{
						Option::Some(v)
							=> Result::Ok(v),
						Option::None // An error that should not happen
							//There is nothing we can return here, just panic!
							=> panic!("Failed to retrieve a val that just inserted to the HashMap.")
					}
				},
				Option::Some(v) => // An error that should not happen
				{
					Result::Err(v.to_decl())
				}
			}

		}
		else
		{
			Result::Err((pt, cmd))
		}
	}

	pub fn get_mut(&mut self, name : &String) -> Option<&mut T>
	{
		self.map.get_mut(name)
	}

	pub fn get(&self, name : &String) -> Option<&T>
	{
		self.map.get(name)
	}
}

impl<T : fmt::Display> fmt::Display for FuncStates<T>
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

pub struct VarStates<T : fmt::Display + ToFromVarDecl<T> >
{
	map : HashMap<String, T>,
}

impl<T : fmt::Display + ToFromVarDecl<T> > VarStates<T>
{
	pub fn new() -> VarStates<T>
	{
		VarStates { map : HashMap::new() }
	}

	pub fn decl(&mut self, decl : super::var_general::VarDecl) -> Result<&mut T, super::var_general::VarDecl>
	{
		let var_name = decl.name.clone();

		if !self.map.contains_key(&var_name)
		{
			match self.map.insert(var_name.clone(), T::from_decl(decl))
			{
				Option::None => //inserted successfully
				{
					match self.map.get_mut(&var_name)
					{
						Option::Some(v)
							=> Result::Ok(v),
						Option::None // An error that should not happen
							//There is nothing we can return here, just panic!
							=> panic!("Failed to retrieve a val that just inserted to the HashMap.")
					}
				},
				Option::Some(v) => // An error that should not happen
				{
					Result::Err(v.to_decl(var_name))
				}
			}

		}
		else
		{
			Result::Err(decl)
		}
	}

	pub fn get_mut(&mut self, name : &String) -> Option<&mut T>
	{
		self.map.get_mut(name)
	}

	pub fn get(&self, name : &String) -> Option<&T>
	{
		self.map.get(name)
	}
}

impl<T : fmt::Display + ToFromVarDecl<T> > fmt::Display for VarStates<T>
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

pub struct States<FnStateType : fmt::Display + ToFromFnDecl<FnStateType>, VarStateType : fmt::Display + ToFromVarDecl<VarStateType> >
{
	f : FuncStates<FnStateType>,
	v : VarStates<VarStateType>,
}

impl<FnStateType : fmt::Display + ToFromFnDecl<FnStateType>, VarStateType : fmt::Display + ToFromVarDecl<VarStateType> >
States<FnStateType, VarStateType>
{
	pub fn new() -> States<FnStateType, VarStateType>
	{
		States { f : FuncStates::new(), v : VarStates::new() }
	}

	pub fn decl_fn(&mut self, pt : super::func_general::FnProtoType, cmd : super::cmd::Cmd) -> Result<&mut FnStateType, (super::func_general::FnProtoType, super::cmd::Cmd)>
	{
		self.f.decl(pt, cmd)
	}

	pub fn get_fn_mut(&mut self, name : &String) -> Option<&mut FnStateType>
	{
		self.f.get_mut(name)
	}

	pub fn get_fn(&self, name : &String) -> Option<&FnStateType>
	{
		self.f.get(name)
	}

	pub fn decl_var(&mut self, decl : super::var_general::VarDecl) -> Result<&mut VarStateType, super::var_general::VarDecl>
	{
		self.v.decl(decl)
	}

	pub fn get_var_mut(&mut self, name : &String) -> Option<&mut VarStateType>
	{
		self.v.get_mut(name)
	}

	pub fn get_var(&self, name : &String) -> Option<&VarStateType>
	{
		self.v.get(name)
	}
}

impl<FnStateType : fmt::Display + ToFromFnDecl<FnStateType>, VarStateType : fmt::Display + ToFromVarDecl<VarStateType> >
fmt::Display for States<FnStateType, VarStateType>
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f, "Function States:\n")?;
		write!(f, "{}", self.f)?;
		write!(f, "Variable States:\n")?;
		write!(f, "{}", self.v)
	}
}

pub struct StatesStack
<FnStateType : fmt::Display + ToFromFnDecl<FnStateType>, VarStateType : fmt::Display + ToFromVarDecl<VarStateType> >
{
	stack : Vec<States<FnStateType, VarStateType> >,
}

impl<FnStateType : fmt::Display + ToFromFnDecl<FnStateType>, VarStateType : fmt::Display + ToFromVarDecl<VarStateType> >
StatesStack<FnStateType, VarStateType>
{
	pub fn new() -> StatesStack<FnStateType, VarStateType>
	{
		StatesStack { stack : Vec::new() }
	}

	pub fn push(&mut self)
	{
		self.stack.push(States::new());
	}

	pub fn pop(&mut self)
	{
		self.stack.pop();
	}

	pub fn get_states_mut(&mut self) -> Option<&mut States<FnStateType, VarStateType> >
	{
		self.stack.last_mut()
	}

	pub fn get_states(&self) -> Option<&States<FnStateType, VarStateType> >
	{
		self.stack.last()
	}

	pub fn decl_fn(&mut self, pt : super::func_general::FnProtoType, cmd : super::cmd::Cmd) -> Result<&mut FnStateType, (super::func_general::FnProtoType, super::cmd::Cmd)>
	{
		match self.stack.last_mut()
		{
			Option::None    => Result::Err((pt, cmd)),
			Option::Some(v) => v.decl_fn(pt, cmd),
		}
	}

	pub fn get_fn_mut(&mut self, name : &String) -> Option<&mut FnStateType>
	{
		for s in self.stack.iter_mut().rev()
		{
			match s.get_fn_mut(name)
			{
				Option::Some(v) => return Option::Some(v),
				Option::None    => {},
			}
		}

		Option::None
	}

	pub fn get_fn(&self, name : &String) -> Option<&FnStateType>
	{
		for s in self.stack.iter().rev()
		{
			match s.get_fn(name)
			{
				Option::Some(v) => return Option::Some(v),
				Option::None    => {},
			}
		}

		Option::None
	}

	pub fn decl_var(&mut self, decl : super::var_general::VarDecl) -> Result<&mut VarStateType, super::var_general::VarDecl>
	{
		match self.stack.last_mut()
		{
			Option::None    => Result::Err(decl),
			Option::Some(v) => v.decl_var(decl),
		}
	}

	pub fn get_var_mut(&mut self, name : &String) -> Option<&mut VarStateType>
	{
		for s in self.stack.iter_mut().rev()
		{
			match s.get_var_mut(name)
			{
				Option::Some(v) => return Option::Some(v),
				Option::None    => {},
			}
		}

		Option::None
	}

	pub fn get_var(&self, name : &String) -> Option<&VarStateType>
	{
		for s in self.stack.iter().rev()
		{
			match s.get_var(name)
			{
				Option::Some(v) => return Option::Some(v),
				Option::None    => {},
			}
		}

		Option::None
	}
}

impl<FnStateType : fmt::Display + ToFromFnDecl<FnStateType>, VarStateType : fmt::Display + ToFromVarDecl<VarStateType> >
fmt::Display for StatesStack<FnStateType, VarStateType>
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		for i in 0..self.stack.len()
		{
			write!(f, "State Level {}\n", i)?;
			write!(f, "{}", self.stack[i])?;
		}

		write!(f, "")
	}
}
