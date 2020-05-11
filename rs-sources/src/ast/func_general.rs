use std::fmt;

pub struct FnProtoType
{
	ret_type : super::data_type::DataType,
	name : String,
	var_decl_list : Vec<super::var_general::VarDecl>,
}

impl FnProtoType
{
	pub fn new(ret_type : super::data_type::DataType, name : String, var_decl_list : Vec<super::var_general::VarDecl>) -> FnProtoType
	{
		FnProtoType {ret_type : ret_type, name : name, var_decl_list : var_decl_list}
	}

	pub fn fmt_var_decl_list(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		let mut is_first = true;

		for var_decl in self.var_decl_list.iter()
		{
			if is_first
			{
				is_first = false;
				write!(f, "{}", var_decl)?;
			}
			else
			{
				write!(f, ", {}", var_decl)?;
			}
		}

		write!(f, "")
	}
}

impl fmt::Display for FnProtoType
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f, "fn {}(", self.name)?;
		self.fmt_var_decl_list(f)?;
		write!(f, ") -> {}", self.ret_type)
	}
}

pub struct FnCall
{
	name : String,
	exp_list : Vec<super::exp::Exp>,
}

impl FnCall
{
	pub fn new(name : String, exp_list : Vec<super::exp::Exp>) -> FnCall
	{
		FnCall {name : name, exp_list : exp_list}
	}

	pub fn fmt_exp_list(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		let mut is_first = true;

		for exp in self.exp_list.iter()
		{
			if is_first
			{
				is_first = false;
				write!(f, "{}", exp)?;
			}
			else
			{
				write!(f, ", {}", exp)?;
			}
		}

		write!(f, "")
	}
}

impl fmt::Display for FnCall
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		write!(f, "{}(", self.name)?;
		self.fmt_exp_list(f)?;
		write!(f, ")")
	}
}
