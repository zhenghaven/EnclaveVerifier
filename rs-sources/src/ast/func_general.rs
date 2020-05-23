use std::fmt;

use std::vec::Vec;
use std::string::String;

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

impl super::Serializible for FnProtoType
{
	/// Serialize the AST (of FnProtoType type) into serials of bytes, and return the vector of bytes.
	///
	/// Please refer to the documentation on the trait for detail.
	///
	/// # FnProtoType layout
	/// ```
	///            | Datatype - 1 byte | string - 10+ bytes | uint64 - 9 Bytes | VarDecl::bytes | ...
	/// ```
	///
	fn to_bytes(&self) -> Result<Vec<u8>, String>
	{
		// 1. ret type
		let mut res = self.ret_type.to_bytes()?;

		// 2. func name
		res.append(&mut (super::primit_serialize::string_to_bytes(&self.name)));

		// 3. var list len
		let var_decl_list_len_u64 : u64 = self.var_decl_list.len() as u64;
		res.append(&mut (super::primit_serialize::uint64_to_bytes(&var_decl_list_len_u64)));

		// 4. var list
		for var_decl_item in self.var_decl_list.iter()
		{
			res.append(&mut (var_decl_item.to_bytes()?));
		}

		Result::Ok(res)
	}
}

impl super::Deserializible<FnProtoType> for FnProtoType
{
	fn from_bytes(bytes : &[u8]) -> Result<(&[u8], FnProtoType), String>
	{
		// 1. ret type
		let (bytes_left_1, parsed_ret_type) = super::data_type::DataType::from_bytes(bytes)?;

		// 2. func name
		let (bytes_left_2, parsed_name) = super::primit_serialize::string_from_bytes(bytes_left_1)?;

		// 3. var list len
		let (bytes_left_3, var_decl_list_len_u64) = super::primit_serialize::uint64_from_bytes(bytes_left_2)?;
		let var_decl_list_len = var_decl_list_len_u64 as usize;

		// 4. var list
		let mut parsed_var_decl_list : Vec<super::var_general::VarDecl> = vec![];
		let mut bytes_left_list : Vec<&[u8]> = vec![bytes_left_3];
		parsed_var_decl_list.reserve(var_decl_list_len);
		bytes_left_list.reserve(var_decl_list_len + 1);

		for i in 0..var_decl_list_len
		{
			let (bytes_left_i, var_decl_item) = super::var_general::VarDecl::from_bytes(bytes_left_list[i])?;
			parsed_var_decl_list.push(var_decl_item);
			bytes_left_list.push(bytes_left_i);
		}

		Result::Ok((bytes_left_list[var_decl_list_len],
			FnProtoType{ret_type : parsed_ret_type, name : parsed_name, var_decl_list : parsed_var_decl_list}
		))
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

impl super::Serializible for FnCall
{
	/// Serialize the AST (of FnCall type) into serials of bytes, and return the vector of bytes.
	///
	/// Please refer to the documentation on the trait for detail.
	///
	/// # FnCall layout
	/// ```
	///            | string - 10+ bytes | uint64 - 9 Bytes | Exp::bytes | ...
	/// ```
	///
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

impl super::Deserializible<FnCall> for FnCall
{
	fn from_bytes(bytes : &[u8]) -> Result<(&[u8], FnCall), String>
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
