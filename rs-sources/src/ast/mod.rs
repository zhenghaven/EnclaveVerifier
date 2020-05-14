
/// Serialize the command or expression.
/// 
/// # Exp / Cmd layout
/// ```
/// 	| type - 1 Byte | value - TBD Bytes |
/// ```
/// * type : 0 - cmd, 1 - aexp, 2 - bexp
/// * value : follow next section
/// 
/// # Cmd value layout
/// ```
/// 	| type - 1 Byte | sub-values - TBD Bytes |
/// ```
/// 
/// # Aexp value layout
/// ```
/// 	| type - 1 Byte | sub-values - TBD Bytes |
/// ```
/// 
/// # Bexp value layout
/// ```
/// 	| type - 1 Byte | sub-values - TBD Bytes |
/// ```
/// 
pub trait Serializible
{
	fn to_bytes(&self) -> Result<Vec<u8>, String>;
}

pub trait Deserializible<T>
{
	fn from_bytes(bytes : &[u8]) -> Result<(&[u8], T), String>;
}

pub enum IndentString
{
	Enter,
	Stay(String),
	Exit,
}

pub fn indent_lines_to_string(in_lines : &Vec<IndentString>, indent_ch : char) -> String
{
	let mut res : String = String::new();
	let mut indent : String = String::new();

	for line in in_lines.iter()
	{
		match line
		{
			IndentString::Enter =>
			{
				res = res + &indent + "{\n";
				indent.push(indent_ch);
			},
			IndentString::Stay(s) =>
			{
				res = res + &indent + s + "\n";
			},
			IndentString::Exit =>
			{
				indent.pop();
				res = res + &indent + "}\n";
			},
		}
	}

	res
}

pub mod primit_serialize;

pub mod data_type;
pub mod var_general;
pub mod exp;
pub mod func_general;
pub mod aexp;
pub mod bexp;
pub mod cmd;
