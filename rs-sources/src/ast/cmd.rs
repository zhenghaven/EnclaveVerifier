use std::fmt;

use std::vec::Vec;
use std::string::String;

use std::boxed::Box;

pub enum Cmd
{
	Skip,
	VarDecl   {d : Box<super::var_general::VarDecl>},
	Assign    {var : Box<super::var_general::VarRef>, e : Box<super::exp::Exp>},
	IfElse    {cond : Box<super::bexp::Bexp>, tr_cmd : Box<Cmd>, fa_cmd : Box<Cmd>},
	WhileLoop {cond : Box<super::bexp::Bexp>, lp_cmd : Box<Cmd>},
	Seq       {fst_cmd : Box<Cmd>, snd_cmd : Box<Cmd>},
	FnDecl    {prototype : Box<super::func_general::FnProtoType>, fn_cmd : Box<Cmd>},
	Return    {e : Box<super::exp::Exp>},
}

impl Cmd
{
	fn to_byte_id(&self) -> ByteId
	{
		match self
		{
			Cmd::Skip                               => ByteId::Skip,
			Cmd::VarDecl{d:_}                       => ByteId::VarDecl,
			Cmd::Assign{var:_, e:_}                 => ByteId::Assign,
			Cmd::IfElse{cond:_, tr_cmd:_, fa_cmd:_} => ByteId::IfElse,
			Cmd::WhileLoop{cond:_, lp_cmd:_}        => ByteId::WhileLoop,
			Cmd::Seq{fst_cmd:_, snd_cmd:_}          => ByteId::Seq,
			Cmd::FnDecl{prototype:_, fn_cmd:_}      => ByteId::FnDecl,
			Cmd::Return{e:_}                        => ByteId::Return,
		}
	}

	pub fn to_indent_lines(&self, out_lines : &mut Vec<super::IndentString>)
	{
		match self
		{
			Cmd::Skip                         => {},
			Cmd::VarDecl{d}                   => out_lines.push(super::IndentString::Stay(format!("let {};", d))),
			Cmd::Assign{var, e}               => out_lines.push(super::IndentString::Stay(format!("{} = {};", var, e))),
			Cmd::IfElse{cond, tr_cmd, fa_cmd} =>
			{
				out_lines.push(super::IndentString::Stay(format!("if {}", cond)));
				out_lines.push(super::IndentString::Enter);
				tr_cmd.to_indent_lines(out_lines);
				out_lines.push(super::IndentString::Exit);

				match *(*fa_cmd)
				{
					Cmd::Skip => {},
					_         =>
					{
						out_lines.push(super::IndentString::Stay(format!("else")));
						out_lines.push(super::IndentString::Enter);
						fa_cmd.to_indent_lines(out_lines);
						out_lines.push(super::IndentString::Exit);
					},
				}
			},
			Cmd::WhileLoop{cond, lp_cmd}      =>
			{
				out_lines.push(super::IndentString::Stay(format!("while {}", cond)));
				out_lines.push(super::IndentString::Enter);
				lp_cmd.to_indent_lines(out_lines);
				out_lines.push(super::IndentString::Exit);
			},
			Cmd::Seq{fst_cmd, snd_cmd}        =>
			{
				fst_cmd.to_indent_lines(out_lines);
				snd_cmd.to_indent_lines(out_lines);
			},
			Cmd::FnDecl{prototype, fn_cmd}    =>
			{
				out_lines.push(super::IndentString::Stay(format!("{}", prototype)));
				out_lines.push(super::IndentString::Enter);
				fn_cmd.to_indent_lines(out_lines);
				out_lines.push(super::IndentString::Exit);
			},
			Cmd::Return{e}                    => out_lines.push(super::IndentString::Stay(format!("return {};", e))),
		}
	}
}

impl super::Serializible for Cmd
{
	/// Serialize the AST (of Cmd type) into serials of bytes, and return the vector of bytes.
	///
	/// Please refer to the documentation on the trait for detail.
	///
	/// # Cmd layout
	/// ```
	/// Skip:       | type=0 - 1 Byte |
	/// VarDecl:    | type=1 - 1 Byte | VarDecl::bytes     |
	/// Assign:     | type=2 - 1 Byte | VarRef::bytes      |  Exp::bytes  |
	/// IfElse:     | type=3 - 1 Byte | Bexp::bytes        |  Cmd::bytes  |  Cmd::bytes  |
	/// WhileLoop:  | type=4 - 1 Byte | Bexp::bytes        |  Cmd::bytes  |
	/// Seq:        | type=5 - 1 Byte | Cmd::bytes         |  Cmd::bytes  |
	/// FnDecl:     | type=6 - 1 Byte | FnProtoType::bytes |  Cmd::bytes  |
	/// Return:     | type=7 - 1 Byte | Exp::bytes         |
	/// ```
	///
	fn to_bytes(&self) -> Result<Vec<u8>, String>
	{
		let mut res : Vec<u8> = vec![self.to_byte_id().to_byte()];

		match self
		{
			Cmd::Skip =>
			{
				Result::Ok(res)
			},
			Cmd::VarDecl{d} =>
			{
				res.append(&mut (d.to_bytes()?));
				Result::Ok(res)
			},
			Cmd::Assign{var, e} =>
			{
				res.append(&mut (var.to_bytes()?));
				res.append(&mut (e.to_bytes()?));

				Result::Ok(res)
			},
			Cmd::IfElse{cond, tr_cmd, fa_cmd} =>
			{
				res.append(&mut (cond.to_bytes()?));
				res.append(&mut (tr_cmd.to_bytes()?));
				res.append(&mut (fa_cmd.to_bytes()?));

				Result::Ok(res)
			},
			Cmd::WhileLoop{cond, lp_cmd} =>
			{
				res.append(&mut (cond.to_bytes()?));
				res.append(&mut (lp_cmd.to_bytes()?));

				Result::Ok(res)
			},
			Cmd::Seq{fst_cmd, snd_cmd} =>
			{
				res.append(&mut (fst_cmd.to_bytes()?));
				res.append(&mut (snd_cmd.to_bytes()?));

				Result::Ok(res)
			},
			Cmd::FnDecl{prototype, fn_cmd} =>
			{
				res.append(&mut (prototype.to_bytes()?));
				res.append(&mut (fn_cmd.to_bytes()?));

				Result::Ok(res)
			},
			Cmd::Return{e} =>
			{
				res.append(&mut (e.to_bytes()?));

				Result::Ok(res)
			},
		}
	}
}

impl super::Deserializible<Cmd> for Cmd
{
	fn from_bytes(bytes : &[u8]) -> Result<(&[u8], Cmd), String>
	{
		use constructor_helper::*;
		if bytes.len() > 0
		{
			let type_id = ByteId::from_byte(&bytes[0])?;
			match type_id
			{
				ByteId::Skip      =>
				{
					Result::Ok((&bytes[1..], skip()))
				},
				ByteId::VarDecl   =>
				{
					let (bytes_left_1, parsed_var_dc) = super::var_general::VarDecl::from_bytes(&bytes[1..])?;

					Result::Ok((bytes_left_1, var_dc(parsed_var_dc)))
				},
				ByteId::Assign    =>
				{
					let (bytes_left_1, parsed_var_ref) = super::var_general::VarRef::from_bytes(&bytes[1..])?;
					let (bytes_left_2, parsed_e) = super::exp::Exp::from_bytes(bytes_left_1)?;

					Result::Ok((bytes_left_2, assign(parsed_var_ref, parsed_e)))
				},
				ByteId::IfElse    =>
				{
					let (bytes_left_1, parsed_cond) = super::bexp::Bexp::from_bytes(&bytes[1..])?;
					let (bytes_left_2, parsed_tr_cmd) = super::cmd::Cmd::from_bytes(bytes_left_1)?;
					let (bytes_left_3, parsed_fa_cmd) = super::cmd::Cmd::from_bytes(bytes_left_2)?;

					Result::Ok((bytes_left_3, if_el(parsed_cond, parsed_tr_cmd, parsed_fa_cmd)))
				},
				ByteId::WhileLoop =>
				{
					let (bytes_left_1, parsed_cond) = super::bexp::Bexp::from_bytes(&bytes[1..])?;
					let (bytes_left_2, parsed_lp_cmd) = super::cmd::Cmd::from_bytes(bytes_left_1)?;

					Result::Ok((bytes_left_2, wh_lp(parsed_cond, parsed_lp_cmd)))
				},
				ByteId::Seq       =>
				{
					let (bytes_left_1, parsed_fst_cmd) = super::cmd::Cmd::from_bytes(&bytes[1..])?;
					let (bytes_left_2, parsed_snd_cmd) = super::cmd::Cmd::from_bytes(bytes_left_1)?;

					Result::Ok((bytes_left_2, seq(parsed_fst_cmd, parsed_snd_cmd)))
				},
				ByteId::FnDecl    =>
				{
					let (bytes_left_1, parsed_prototype) = super::func_general::FnProtoType::from_bytes(&bytes[1..])?;
					let (bytes_left_2, parsed_fn_cmd) = super::cmd::Cmd::from_bytes(bytes_left_1)?;

					Result::Ok((bytes_left_2, fn_dc(parsed_prototype, parsed_fn_cmd)))
				},
				ByteId::Return    =>
				{
					let (bytes_left_1, parsed_e) = super::exp::Exp::from_bytes(&bytes[1..])?;

					Result::Ok((bytes_left_1, ret(parsed_e)))
				},
			}
		}
		else
		{
			Result::Err(format!("{}", "Failed to parse Cmd. Bytes are shorter than expected."))
		}
	}
}

impl fmt::Display for Cmd
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		match self
		{
			Cmd::Skip                         => write!(f, ""),
			Cmd::VarDecl{d}                   => write!(f, "let {};", d),
			Cmd::Assign{var, e}               => write!(f, "{} = {};", var, e),
			Cmd::IfElse{cond, tr_cmd, fa_cmd} =>
			{
				match *(*fa_cmd)
				{
					Cmd::Skip => write!(f, "if {0}\n{2}\n{1}\n{3}", cond, tr_cmd, "{", "}"),
					_         => write!(f, "if {0}\n{3}\n{1}\n{4} else {3}\n{2}\n{4}", cond, tr_cmd, fa_cmd, "{", "}"),
				}
			},
			Cmd::WhileLoop{cond, lp_cmd}      => write!(f, "while {0}\n{2}\n{1}\n{3}", cond, lp_cmd, "{", "}"),
			Cmd::Seq{fst_cmd, snd_cmd}        => write!(f, "{}\n{}", fst_cmd, snd_cmd),
			Cmd::FnDecl{prototype, fn_cmd}    => write!(f, "{0}\n{2}\n{1}\n{3}", prototype, fn_cmd, "{", "}"),
			Cmd::Return{e}                    => write!(f, "return {};", e),
		}
	}
}

pub mod constructor_helper
{
	use std::boxed::Box;

	pub fn skip() -> super::Cmd
	{
		super::Cmd::Skip
	}

	pub fn var_dc(d : super::super::var_general::VarDecl) -> super::Cmd
	{
		super::Cmd::VarDecl {d : Box::new(d)}
	}

	pub fn assign(var : super::super::var_general::VarRef, e : super::super::exp::Exp) -> super::Cmd
	{
		super::Cmd::Assign {var : Box::new(var), e : Box::new(e)}
	}

	pub fn if_el(cond : super::super::bexp::Bexp, tr_cmd : super::Cmd, fa_cmd : super::Cmd) -> super::Cmd
	{
		super::Cmd::IfElse {cond : Box::new(cond), tr_cmd : Box::new(tr_cmd), fa_cmd : Box::new(fa_cmd)}
	}

	pub fn wh_lp(cond : super::super::bexp::Bexp, lp_cmd : super::Cmd) -> super::Cmd
	{
		super::Cmd::WhileLoop {cond : Box::new(cond), lp_cmd : Box::new(lp_cmd)}
	}

	pub fn seq(fst_cmd : super::Cmd, snd_cmd : super::Cmd) -> super::Cmd
	{
		super::Cmd::Seq {fst_cmd : Box::new(fst_cmd), snd_cmd : Box::new(snd_cmd)}
	}

	pub fn fn_dc(prototype : super::super::func_general::FnProtoType, fn_cmd : super::Cmd) -> super::Cmd
	{
		super::Cmd::FnDecl {prototype : Box::new(prototype), fn_cmd : Box::new(fn_cmd)}
	}

	pub fn ret(e : super::super::exp::Exp) -> super::Cmd
	{
		super::Cmd::Return {e : Box::new(e)}
	}
}

enum ByteId
{
	Skip,
	VarDecl,
	Assign,
	IfElse,
	WhileLoop,
	Seq,
	FnDecl,
	Return,
}

impl ByteId
{
	fn to_byte(&self) -> u8
	{
		match self
		{
			ByteId::Skip      => 0u8,
			ByteId::VarDecl   => 1u8,
			ByteId::Assign    => 2u8,
			ByteId::IfElse    => 3u8,
			ByteId::WhileLoop => 4u8,
			ByteId::Seq       => 5u8,
			ByteId::FnDecl    => 6u8,
			ByteId::Return    => 7u8,
		}
	}

	fn from_byte(b : &u8) -> Result<ByteId, String>
	{
		match b
		{
			0u8 => Result::Ok(ByteId::Skip),
			1u8 => Result::Ok(ByteId::VarDecl),
			2u8 => Result::Ok(ByteId::Assign),
			3u8 => Result::Ok(ByteId::IfElse),
			4u8 => Result::Ok(ByteId::WhileLoop),
			5u8 => Result::Ok(ByteId::Seq),
			6u8 => Result::Ok(ByteId::FnDecl),
			7u8 => Result::Ok(ByteId::Return),
			_   => Result::Err(format!("{}", "Unrecognized type ID from byte for Cmd."))
		}
	}
}
