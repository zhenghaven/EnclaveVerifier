use std::ops;
use std::cmp;
use std::fmt;
use std::rc::Rc;
use std::vec::Vec;
use std::string::String;

use super::super::ast::aexp;
use super::super::ast::data_type;
use super::super::ast::states::FuncStatesStack;
use super::super::ast::states::VarStatesStack;

use super::exp::ExpValue;
use super::states;
use super::states::FuncState;
use super::states::VarState;

#[derive(Clone)]
pub enum AexpValue
{
	Int32(i32),
	Float32(f32),
}

impl AexpValue
{
	pub fn get_data_type(&self) -> super::super::ast::data_type::DataType
	{
		match self
		{
			AexpValue::Int32(_)   => super::super::ast::data_type::DataType::Int32,
			AexpValue::Float32(_) => super::super::ast::data_type::DataType::Float32,
		}
	}

	pub fn to_aexp(self) -> aexp::Aexp
	{
		match self
		{
			AexpValue::Int32(val)   => aexp::Aexp::IntConst { v: val },
			AexpValue::Float32(val) => aexp::Aexp::FloConst { v: val },
		}
	}

	pub fn get_type(&self) -> data_type::DataType
	{
		match self
		{
			AexpValue::Int32(_) => data_type::DataType::Int32,
			AexpValue::Float32(_) => data_type::DataType::Float32,
		}
	}

	fn to_bytes_byte(&self) -> u8
	{
		match self
		{
			AexpValue::Int32(_)   => 0u8,
			AexpValue::Float32(_) => 1u8,
		}
	}

	pub fn to_bytes(&self) -> Result<Vec<u8>, String>
	{
		let mut res_vec : Vec<u8> = vec![self.to_bytes_byte()];

		match self
		{
			AexpValue::Int32(val)   =>
			{
				res_vec.append(&mut super::super::ast::primit_serialize::int32_to_bytes(&val))
			},
			AexpValue::Float32(val) =>
			{
				res_vec.append(&mut super::super::ast::primit_serialize::flo32_to_bytes(&val))
			},
		}

		Result::Ok(res_vec)
	}

	fn from_bytes_byte(bytes : &[u8]) -> Result<(&[u8], AexpValue), String>
	{
		if bytes.len() > 1
		{
			match bytes[0]
			{
				0u8 =>
				{
					let (bytes_left, res_val) = super::super::ast::primit_serialize::int32_from_bytes(&bytes[1..])?;
					return Result::Ok((bytes_left, AexpValue::Int32(res_val)));
				},
				1u8 =>
				{
					let (bytes_left, res_val) = super::super::ast::primit_serialize::flo32_from_bytes(&bytes[1..])?;
					return Result::Ok((bytes_left, AexpValue::Float32(res_val)));
				},
				_   => {},
			}
		}

		Result::Err(format!("Failed to deserialize ExpValue."))
	}

	pub fn from_bytes(bytes : &[u8]) -> Result<(&[u8], AexpValue), String>
	{
		Self::from_bytes_byte(bytes)
	}

	pub fn promote_to_flo32(self) -> AexpValue
	{
		match self
		{
			AexpValue::Int32(v)   => AexpValue::Float32(v as f32),
			AexpValue::Float32(_) => self,
		}
	}
}

impl super::exp::CanConvertToExpVal for AexpValue
{
	fn to_exp_val(self) -> super::exp::ExpValue
	{
		super::exp::ExpValue::A(self)
	}
}

impl fmt::Display for AexpValue
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match &self
		{
			AexpValue::Int32(val)   => write!(f, "{}", val),
			AexpValue::Float32(val) => write!(f, "{}", val),
		}
	}
}

pub trait CanConvertToAexpVal
{
	fn to_aexp_val(self) -> Result<super::aexp::AexpValue, String>;
}

impl ops::Add<AexpValue> for AexpValue
{
	type Output = AexpValue;

	fn add(self, rhs : AexpValue) -> AexpValue
	{
		match self
		{
			AexpValue::Int32(lv)   =>
				match rhs
				{
					AexpValue::Int32(rv)   => AexpValue::Int32(lv + rv),
					AexpValue::Float32(rv) => AexpValue::Float32((lv as f32) + rv),
				},
			AexpValue::Float32(lv) =>
				match rhs
				{
					AexpValue::Int32(rv)   => AexpValue::Float32(lv + (rv as f32)),
					AexpValue::Float32(rv) => AexpValue::Float32(lv + rv),
				},
		}
	}
}

impl ops::Sub<AexpValue> for AexpValue
{
	type Output = AexpValue;

	fn sub(self, rhs : AexpValue) -> AexpValue
	{
		match self
		{
			AexpValue::Int32(lv)   =>
				match rhs
				{
					AexpValue::Int32(rv)   => AexpValue::Int32(lv - rv),
					AexpValue::Float32(rv) => AexpValue::Float32((lv as f32) - rv),
				},
			AexpValue::Float32(lv) =>
				match rhs
				{
					AexpValue::Int32(rv)   => AexpValue::Float32(lv - (rv as f32)),
					AexpValue::Float32(rv) => AexpValue::Float32(lv - rv),
				},
		}
	}
}

impl ops::Mul<AexpValue> for AexpValue
{
	type Output = AexpValue;

	fn mul(self, rhs : AexpValue) -> AexpValue
	{
		match self
		{
			AexpValue::Int32(lv)   =>
				match rhs
				{
					AexpValue::Int32(rv)   => AexpValue::Int32(lv * rv),
					AexpValue::Float32(rv) => AexpValue::Float32((lv as f32) * rv),
				},
			AexpValue::Float32(lv) =>
				match rhs
				{
					AexpValue::Int32(rv)   => AexpValue::Float32(lv * (rv as f32)),
					AexpValue::Float32(rv) => AexpValue::Float32(lv * rv),
				},
		}
	}
}

impl ops::Div<AexpValue> for AexpValue
{
	type Output = AexpValue;

	fn div(self, rhs : AexpValue) -> AexpValue
	{
		match self
		{
			AexpValue::Int32(lv)   =>
				match rhs
				{
					AexpValue::Int32(rv)   => AexpValue::Int32(lv / rv),
					AexpValue::Float32(rv) => AexpValue::Float32((lv as f32) / rv),
				},
			AexpValue::Float32(lv) =>
				match rhs
				{
					AexpValue::Int32(rv)   => AexpValue::Float32(lv / (rv as f32)),
					AexpValue::Float32(rv) => AexpValue::Float32(lv / rv),
				},
		}
	}
}

impl ops::Rem<AexpValue> for AexpValue
{
	type Output = AexpValue;

	fn rem(self, rhs : AexpValue) -> AexpValue
	{
		match self
		{
			AexpValue::Int32(lv)   =>
				match rhs
				{
					AexpValue::Int32(rv)   => AexpValue::Int32(lv % rv),
					AexpValue::Float32(rv) => AexpValue::Float32((lv as f32) % rv),
				},
			AexpValue::Float32(lv) =>
				match rhs
				{
					AexpValue::Int32(rv)   => AexpValue::Float32(lv % (rv as f32)),
					AexpValue::Float32(rv) => AexpValue::Float32(lv % rv),
				},
		}
	}
}

impl cmp::PartialOrd for super::aexp::AexpValue
{
	fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering>
	{
        Some(self.cmp(other))
    }
}

impl cmp::Ord for super::aexp::AexpValue
{
	fn cmp(&self, rhs: &Self) -> cmp::Ordering
	{
		match self
		{
			AexpValue::Int32(lv)   =>
				match rhs
				{
					AexpValue::Int32(rv)   => lv.cmp(rv),
					AexpValue::Float32(rv) =>
						if (*lv as f32) < *rv { cmp::Ordering::Less } else { if (*lv as f32) > *rv { cmp::Ordering::Greater } else { cmp::Ordering::Equal } },
				},
			AexpValue::Float32(lv) =>
				match rhs
				{
					AexpValue::Int32(rv)   =>
						if *lv < (*rv as f32) { cmp::Ordering::Less } else { if *lv > (*rv as f32) { cmp::Ordering::Greater } else { cmp::Ordering::Equal } },
					AexpValue::Float32(rv) =>
						if lv < rv { cmp::Ordering::Less } else { if lv > rv { cmp::Ordering::Greater } else { cmp::Ordering::Equal } },
				},
		}
    }
}

impl cmp::PartialEq for super::aexp::AexpValue
{
	fn eq(&self, rhs: &Self) -> bool
	{
		match self
		{
			AexpValue::Int32(lv)   =>
				match rhs
				{
					AexpValue::Int32(rv)   => (lv == rv),
					AexpValue::Float32(rv) => ((*lv as f32) == *rv),
				},
			AexpValue::Float32(lv) =>
				match rhs
				{
					AexpValue::Int32(rv)   => (*lv == (*rv as f32)),
					AexpValue::Float32(rv) => (lv == rv),
				},
		}
    }
}

impl cmp::Eq for super::aexp::AexpValue {}

pub trait CanEvalToAexpVal
{
	fn eval_to_aexp_val(
		&self,
		func_states : & Rc<FuncStatesStack<FuncState> >,
		var_states  : & Rc<VarStatesStack<ExpValue, VarState> >)
		-> Result<AexpValue, String>;

	fn simp_eval_to_aexp_val(&self) -> Result<AexpValue, String>;
}

impl CanEvalToAexpVal for aexp::Aexp
{
	fn eval_to_aexp_val(
		&self,
		func_states : & Rc<FuncStatesStack<FuncState> >,
		var_states  : & Rc<VarStatesStack<ExpValue, VarState> >)
		-> Result<AexpValue, String>
	{
		use aexp::Aexp;

		match self
		{
			Aexp::IntConst{v} => Result::Ok(AexpValue::Int32  (v.clone())),
			Aexp::FloConst{v} => Result::Ok(AexpValue::Float32(v.clone())),
			Aexp::Add{l, r} =>
			{
				let l_val = l.eval_to_aexp_val(func_states, var_states)?;
				let r_val = r.eval_to_aexp_val(func_states, var_states)?;

				Result::Ok(l_val + r_val)
			},
			Aexp::Sub{l, r} =>
			{
				let l_val = l.eval_to_aexp_val(func_states, var_states)?;
				let r_val = r.eval_to_aexp_val(func_states, var_states)?;

				Result::Ok(l_val - r_val)
			},
			Aexp::Mul{l, r} =>
			{
				let l_val = l.eval_to_aexp_val(func_states, var_states)?;
				let r_val = r.eval_to_aexp_val(func_states, var_states)?;

				Result::Ok(l_val * r_val)
			},
			Aexp::Div{l, r} =>
			{
				let l_val = l.eval_to_aexp_val(func_states, var_states)?;
				let r_val = r.eval_to_aexp_val(func_states, var_states)?;

				Result::Ok(l_val / r_val)
			},
			Aexp::Mod{l, r} =>
			{
				let l_val = l.eval_to_aexp_val(func_states, var_states)?;
				let r_val = r.eval_to_aexp_val(func_states, var_states)?;

				Result::Ok(l_val % r_val)
			},
			Aexp::Var{v} =>
			{
				let var_opt = var_states.var_read(&v.name);
				match var_opt
				{
					Option::Some(var) =>
						match var
						{
							Option::Some(e_val) => e_val.to_aexp_val(),
							Option::None        => Result::Err(format!("Variable {} hasn't been initialized", v.name)),
						},
					Option::None      => Result::Err(format!("Variable {} hasn't been declared", v.name))
				}
			},
			Aexp::FnCall{fc} =>
			{
				let func_call_res = states::func_call(func_states, var_states, fc, true)?;
				match func_call_res
				{
					Option::Some(ret_val) => ret_val.to_aexp_val(),
					Option::None          => Result::Err(format!("Function {} doesn't return a value", fc.name))
				}
			},
		}
	}

	fn simp_eval_to_aexp_val(&self) -> Result<AexpValue, String>
	{
		use aexp::Aexp;

		match self
		{
			Aexp::IntConst{v} => Result::Ok (AexpValue::Int32  (v.clone())),
			Aexp::FloConst{v} => Result::Ok (AexpValue::Float32(v.clone())),
			_                 => Result::Err(format!("Expecting an evaluted AExp, while an un-evaluated AExp is given."))
		}
	}
}
