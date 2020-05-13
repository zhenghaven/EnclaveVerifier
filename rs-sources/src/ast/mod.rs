
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

pub mod primit_serialize;

pub mod data_type;
pub mod var_general;
pub mod exp;
pub mod func_general;
pub mod aexp;
pub mod bexp;
