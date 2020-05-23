use std::string::String;
use std::vec::Vec;

pub enum PrimtType
{
	U,
	I,
	F,
	O,
}

pub enum PrimtSize
{
	S1,
	S8,
	S16,
	S32,
	S64,
	S128,
}

pub enum ObjType
{
	StringType,
}

pub fn get_primt_type_id(t : PrimtType) -> u8
{
	match t
	{
		PrimtType::U => 0x10,
		PrimtType::I => 0x20,
		PrimtType::F => 0x40,
		PrimtType::O => 0x80,
	}
}

pub fn get_primt_size_id(s : PrimtSize) -> u8
{
	match s
	{
		PrimtSize::S1   => 0,
		PrimtSize::S8   => 1,
		PrimtSize::S16  => 2,
		PrimtSize::S32  => 3,
		PrimtSize::S64  => 4,
		PrimtSize::S128 => 5,
	}
}

pub fn get_obj_type_id(o : ObjType) -> u8
{
	match o
	{
		ObjType::StringType  => 0,
	}
}

pub fn int32_to_bytes(v : &i32) -> Vec<u8>
{
	let id : u8 = get_primt_type_id(PrimtType::I) | get_primt_size_id(PrimtSize::S32);
	let mut res : Vec<u8> = vec![id];
	res.append(&mut v.to_le_bytes().to_vec()); //[u8; 4]

	res //5 Bytes
}

pub fn uint64_to_bytes(v : &u64) -> Vec<u8>
{
	let id : u8 = get_primt_type_id(PrimtType::U) | get_primt_size_id(PrimtSize::S64);
	let mut res : Vec<u8> = vec![id];
	res.append(&mut v.to_le_bytes().to_vec()); //[u8; 8]

	res //9 Bytes
}

pub fn flo32_to_bytes(v : &f32) -> Vec<u8>
{
	let id : u8 = get_primt_type_id(PrimtType::F) | get_primt_size_id(PrimtSize::S32);
	let mut res : Vec<u8> = vec![id];
	res.append(&mut v.to_le_bytes().to_vec()); //[u8; 4]

	res //5 Bytes
}

pub fn bool_to_bytes(v : &bool) -> Vec<u8>
{
	let id : u8 = get_primt_type_id(PrimtType::U) | get_primt_size_id(PrimtSize::S1);
	let val : u8 = if *v { 1 } else { 0 };
	let res : Vec<u8> = vec![id, val];

	res //2 Bytes
}

pub fn string_to_bytes(s : &String) -> Vec<u8>
{
	let id : u8 = get_primt_type_id(PrimtType::O) | get_obj_type_id(ObjType::StringType);
	let string_len : u64 = s.len() as u64;

	let mut res : Vec<u8> = vec![id];

	res.append(&mut uint64_to_bytes(&string_len));
	res.append(&mut s.clone().into_bytes());

	res
}

pub fn int32_from_bytes(bytes : &[u8]) -> Result<(&[u8], i32), String>
{
	if bytes.len() >= 5
	{
		let id_found : &u8 = &bytes[0];
		if ((id_found & get_primt_type_id(PrimtType::I)) != 0) &&
			((id_found & 0x0fu8) == get_primt_size_id(PrimtSize::S32))
		{

			let mut value_bytes : [u8; 4] = [0; 4];
			assert_eq!(value_bytes.len(), bytes[1..5].len());
			value_bytes.copy_from_slice(&bytes[1..5]);

			let res : i32 = i32::from_le_bytes(value_bytes);

			Result::Ok((&bytes[5..], res))
		}
		else
		{
			Result::Err(format!("{}", "Failed to parse i32 from bytes. Primitive type mismatch."))
		}
	}
	else
	{
		Result::Err(format!("{}", "Failed to parse i32 from bytes. Primitive size mismatch!"))
	}
}

pub fn uint64_from_bytes(bytes : &[u8]) -> Result<(&[u8], u64), String>
{
	if bytes.len() >= 9
	{
		let id_found : &u8 = &bytes[0];
		if ((id_found & get_primt_type_id(PrimtType::U)) != 0) &&
			((id_found & 0x0fu8) == get_primt_size_id(PrimtSize::S64))
		{

			let mut value_bytes : [u8; 8] = [0; 8];
			assert_eq!(value_bytes.len(), bytes[1..9].len());
			value_bytes.copy_from_slice(&bytes[1..9]);

			let res : u64 = u64::from_le_bytes(value_bytes);

			Result::Ok((&bytes[9..], res))
		}
		else
		{
			Result::Err(format!("{}", "Failed to parse u64 from bytes. Primitive type mismatch!"))
		}
	}
	else
	{
		Result::Err(format!("{}", "Failed to parse u64 from bytes. Primitive size mismatch!"))
	}
}

pub fn flo32_from_bytes(bytes : &[u8]) -> Result<(&[u8], f32), String>
{
	if bytes.len() >= 5
	{
		let id_found : &u8 = &bytes[0];
		if ((id_found & get_primt_type_id(PrimtType::F)) != 0) &&
			((id_found & 0x0fu8) == get_primt_size_id(PrimtSize::S32))
		{

			let mut value_bytes : [u8; 4] = [0; 4];
			assert_eq!(value_bytes.len(), bytes[1..5].len());
			value_bytes.copy_from_slice(&bytes[1..5]);

			let res : f32 = f32::from_le_bytes(value_bytes);

			Result::Ok((&bytes[5..], res))
		}
		else
		{
			Result::Err(format!("{}", "Failed to parse f32 from bytes. Primitive type mismatch."))
		}
	}
	else
	{
		Result::Err(format!("{}", "Failed to parse f32 from bytes. Primitive size mismatch!"))
	}
}

pub fn bool_from_bytes(bytes : &[u8]) -> Result<(&[u8], bool), String>
{
	if bytes.len() >= 2
	{
		let id_found : &u8 = &bytes[0];
		if ((id_found & get_primt_type_id(PrimtType::U)) != 0) &&
			((id_found & 0x0fu8) == get_primt_size_id(PrimtSize::S1))
		{
			let res : bool = if bytes[1] == 0 { false } else { true };

			Result::Ok((&bytes[2..], res))
		}
		else
		{
			Result::Err(format!("{}", "Failed to parse bool from bytes. Primitive type mismatch!"))
		}
	}
	else
	{
		Result::Err(format!("{}", "Failed to parse bool from bytes. Primitive size mismatch!"))
	}
}

pub fn string_from_bytes(bytes : &[u8]) -> Result<(&[u8], String), String>
{
	if bytes.len() >= 10
	{
		let id_found : &u8 = &bytes[0];
		if ((id_found & get_primt_type_id(PrimtType::O)) != 0) &&
			((id_found & 0x0fu8) == get_obj_type_id(ObjType::StringType))
		{
			let (bytes_left, string_len_u64) = uint64_from_bytes(&bytes[1..])?;
			let string_len = string_len_u64 as usize;

			if bytes_left.len() >= string_len
			{
				let res : String = match String::from_utf8(bytes_left[..string_len].to_vec())
				{
					Ok(string) => string,
					Err(_err) => return Result::Err(format!("{}", "Failed to parse string from bytes. from_utf8 returns error."))
				};

				Result::Ok((&bytes_left[string_len..], res))
			}
			else
			{
				Result::Err(format!("{}", "Failed to parse string from bytes. String length mismatch."))
			}
		}
		else
		{
			Result::Err(format!("{}", "Failed to parse string from bytes. Primitive type mismatch."))
		}
	}
	else
	{
		Result::Err(format!("{}", "Failed to parse string from bytes. Primitive size mismatch."))
	}
}
