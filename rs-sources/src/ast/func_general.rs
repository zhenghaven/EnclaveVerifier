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

	pub fn from_bytes(bytes : &[u8]) -> Result<(&[u8], FnCall), String>
	{
		let (bytes_left_1, name) = super::primit_serialize::string_from_bytes(bytes)?;

		let (bytes_left_2, list_len_u64) = super::primit_serialize::uint64_from_bytes(bytes_left_1)?;

		let list_len = list_len_u64 as usize;

		let mut exp_list : Vec<super::exp::Exp> = vec![];
		let mut bytes_left_list : Vec<&[u8]> = vec![bytes_left_2];
		exp_list.reserve(list_len);
		exp_list.reserve(list_len + 1);

		for i in 0..list_len
		{
			let (bytes_left_i, exp_item) = super::exp::Exp::from_bytes(bytes_left_list[i])?;
			bytes_left_list.push(bytes_left_i);
			exp_list.push(exp_item);
		}

		Result::Ok((bytes_left_list[list_len], FnCall {name : name, exp_list : exp_list}))
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

impl super::Serializible for FnCall
{
	fn to_bytes(&self) -> Result<Vec<u8>, String>
	{
		let mut res = super::primit_serialize::string_to_bytes(&self.name);

		let list_len : u64 = self.exp_list.len() as u64;

		res.append(&mut super::primit_serialize::uint64_to_bytes(&list_len));

		for exp_item in self.exp_list.iter()
		{
			res.append(&mut exp_item.to_bytes()?);
		}

		Result::Ok(res)
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
