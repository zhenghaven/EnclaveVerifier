#![crate_name = "enclave_vrfy_interpreter"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
extern crate sgx_trts;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

use std::vec::Vec;

use sgx_types::*;

extern crate enclave_verifier;

use enclave_verifier::ast;
use enclave_verifier::ast::Deserializible;
use enclave_verifier::interpreter;

#[no_mangle]
pub extern "C" fn interpret_byte_code(byte_code: *const u8, some_len: usize) -> sgx_status_t
{
	let byte_code_slice = unsafe { std::slice::from_raw_parts(byte_code, some_len) };

	println!("[Enclave]: Received bytecode ({} byte(s)).", byte_code_slice.len());

	let (_bytes_left, example_prog) = match ast::cmd::Cmd::from_bytes(byte_code_slice)
	{
		Ok(v) => v,
		Err(why) => panic!("[Enclave]: Couldn't construct AST from byte code. {}", why)
	};
	let mut example_prog_lines : Vec<ast::IndentString> = vec![];
	example_prog.to_indent_lines(&mut example_prog_lines);
	println!("[Enclave]: Example program:\n{}\n", ast::indent_lines_to_string(&example_prog_lines, '\t'));


	let mut prog_inter = interpreter::Program::new();

	gen_prog_states(&mut prog_inter, &example_prog);

	println!("========================================================");
	println!("Program global states:");
	println!("----------------------");
	println!("{}", prog_inter.func_states);
	println!("{}", prog_inter.var_states);
	println!("========================================================");

	use ast::aexp::constructor_helper::ToAexp;
	use ast::exp::constructor_helper::ToExp;

	let param_list_1 = vec![211i32.to_aexp().to_exp()];

	make_entry_call(&prog_inter, param_list_1);

	let param_list_2 = vec![222i32.to_aexp().to_exp()];

	make_entry_call(&prog_inter, param_list_2);


	sgx_status_t::SGX_SUCCESS
}

pub fn gen_prog_states(prog : &mut interpreter::Program, prog_cmd : &ast::cmd::Cmd)
{
	use interpreter::cmd::CanEvalToExpVal;

	let prog_root_res = match prog_cmd.eval_to_exp_val(&mut prog.func_states, &mut prog.var_states)
	{
		Result::Ok(ok_v) => ok_v,
		Result::Err(why) => panic!("{}", why)
	};

	match prog_root_res
	{
		Option::Some(v) => match v
		{
			Option::Some(v2) => panic!("Program root shouldn't contain return statement; it returned {}.", v2),
			Option::None     => panic!("Program root shouldn't contain return statement; even it's a void return."),
		},
		Option::None    => {},
	}
}

pub fn make_entry_call(prog : &interpreter::Program, param_list : Vec<ast::exp::Exp>)
{
	let entry_call = ast::func_general::FnCall::new(format!("entry"), param_list);

	match interpreter::states::func_call(&prog.func_states, &prog.var_states, &entry_call)
	{
		Result::Ok(ok_val) => match ok_val
		{
			Option::Some(v) => println!("Function call {} returned {}", entry_call, v),
			Option::None    => println!("Function call {} didn't return any value.", entry_call)
		},
		Result::Err(why)   => panic!("Entry call failed. {}", why),
	}
}
