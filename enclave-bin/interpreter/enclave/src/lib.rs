#![crate_name = "enclave_vrfy_interpreter"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
extern crate sgx_tcrypto;
extern crate sgx_trts;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

use std::vec::Vec;
use std::string::String;

use sgx_types::*;
use sgx_tcrypto::*;

extern crate base64;

extern crate enclave_verifier;

use enclave_verifier::ast;
use enclave_verifier::ast::Deserializible;
use enclave_verifier::interpreter;

pub fn concat_vec<T>(mut a : Vec<T>, mut b : Vec<T>) -> Vec<T>
{
	a.append(&mut b);
	a
}

pub fn func_call_res_to_bytes(res : &Option<interpreter::exp::ExpValue>) -> Result<Vec<u8>, String>
{
	let mut res_vec : Vec<u8> = Vec::new();

	match res
	{
		Option::Some(v) =>
		{
			res_vec.push(1u8);
			res_vec.append(&mut (v.to_bytes()?));
		},
		Option::None    =>
		{
			res_vec.push(0u8);
		},
	}

	Result::Ok(res_vec)
}

#[no_mangle]
pub extern "C" fn interpret_byte_code(byte_code: *const u8, byte_code_len: usize, param_list: *const u8, param_list_len: usize) -> sgx_status_t
{
	// ------------------------------------------
	// 1. Generate EC key pair:
	// ------------------------------------------
	let ecc_ctx : sgx_tcrypto::SgxEccHandle = sgx_tcrypto::SgxEccHandle::new();

	match ecc_ctx.open()
	{
		Result::Ok(_)    => {},
		Result::Err(err) => { return err; },
	};

	let (encl_prv_key, encl_pub_key) = match ecc_ctx.create_key_pair()
	{
		Result::Ok(val)  => val,
		Result::Err(err) => { return err; },
	};

	println!("[Enclave]: Enclave public key {}{}.", base64::encode(&encl_pub_key.gx), base64::encode(&encl_pub_key.gy));

	// ------------------------------------------
	// 2. Process input bytes:
	// ------------------------------------------

	let input_slice = unsafe { std::slice::from_raw_parts(byte_code, byte_code_len) };
	let param_list_input_slice = unsafe { std::slice::from_raw_parts(param_list, param_list_len) };

	println!("[Enclave]: Received input ({} byte(s)).", input_slice.len());

	let (input_bytes_left, example_prog) = match ast::cmd::Cmd::from_bytes(input_slice)
	{
		Ok(v)    => v,
		Err(why) =>
		{
			print!("[Enclave-ERROR]: Couldn't construct AST from byte code. {}", why);
			return sgx_status_t::SGX_ERROR_UNEXPECTED;
		}
	};

	let byte_code_slice = &input_slice[0..(input_slice.len() - input_bytes_left.len())];

	println!("[Enclave]: Received bytecode ({} byte(s)).", byte_code_slice.len());

	let byte_code_hash = match rsgx_sha256_slice(&byte_code_slice)
	{
		Ok(val)  => val,
		Err(err) => return err,
	};

	println!("[Enclave]: Bytecode hash SHA256(byte_code): {}.", base64::encode(&byte_code_hash));


	// ------------------------------------------
	// 3. Prepare entry function call from input bytes:
	// ------------------------------------------

	let (param_list_bytes_left, param_list) = match ast::func_general::FnCall::exp_list_from_bytes(param_list_input_slice)
	{
		Result::Ok(val)  => val,
		Result::Err(why) =>
		{
			print!("[Enclave-ERROR]: . {}", why);
			return sgx_status_t::SGX_ERROR_UNEXPECTED;
		}
	};

	let param_list_slice = &param_list_input_slice[0..(param_list_input_slice.len() - param_list_bytes_left.len())];

	let param_list_hash = match rsgx_sha256_slice(&param_list_slice)
	{
		Ok(val)  => val,
		Err(err) => return err,
	};

	println!("[Enclave]: Parameter list hash SHA256(param_list): {}.", base64::encode(&param_list_hash));

	let entry_call = ast::func_general::FnCall::new(format!("entry"), param_list);


	// ------------------------------------------
	// 4. Generate program states:
	// ------------------------------------------

	let mut prog_inter = interpreter::Program::new();

	match gen_prog_states(&mut prog_inter, &example_prog)
	{
		Result::Ok(_)    => {},
		Result::Err(why) =>
		{
			print!("[Enclave-ERROR]: . {}", why);
			return sgx_status_t::SGX_ERROR_UNEXPECTED;
		}
	};

	println!("[Enclave]:");
	println!("========================================================");
	println!("Program global states:");
	println!("----------------------");
	println!("{}", prog_inter.func_states);
	println!("{}", prog_inter.var_states);
	println!("========================================================");


	// ------------------------------------------
	// 5. Make entry function call:
	// ------------------------------------------

	let func_call_res = match make_entry_call(&prog_inter, &entry_call)
	{
		Result::Ok(ok_val)  => match &ok_val
		{
			Option::Some(v) =>
			{
				println!("[Enclave]: Function call {} returned {}", entry_call, v);
				ok_val
			},
			Option::None    =>
			{
				println!("[Enclave]: Function call {} didn't return any value.", entry_call);
				ok_val
			}
		},
		Result::Err(why)    =>
		{
			print!("[Enclave-ERROR]: . {}", why);
			return sgx_status_t::SGX_ERROR_UNEXPECTED;
		},
	};

	let func_call_res_bytes = match func_call_res_to_bytes(&func_call_res)
	{
		Result::Ok(ok_val)  => ok_val,
		Result::Err(why)    =>
		{
			print!("[Enclave-ERROR]: . {}", why);
			return sgx_status_t::SGX_ERROR_UNEXPECTED;
		},
	};

	let func_call_res_hash = match rsgx_sha256_slice(&func_call_res_bytes)
	{
		Ok(val)  => val,
		Err(err) => return err,
	};

	println!("[Enclave]: Entry call result hash SHA256(func_ret): {}.", base64::encode(&func_call_res_hash));


	// ------------------------------------------
	// 6. Generate report:
	// ------------------------------------------

	println!("[Enclave]: <{}> --- <{}> ---> <{}>.", base64::encode(&param_list_hash), base64::encode(&byte_code_hash), base64::encode(&func_call_res_hash));

	let mut combined_bytes : Vec<u8> = Vec::new();

	combined_bytes.append(&mut param_list_hash.to_vec());
	combined_bytes.append(&mut byte_code_hash.to_vec());
	combined_bytes.append(&mut func_call_res_hash.to_vec());

	let combined_bytes_hash = match rsgx_sha256_slice(&combined_bytes)
	{
		Ok(val)  => val,
		Err(err) => return err,
	};

	println!("[Enclave]: report hash SHA256(SHA256(param_list) | SHA256(byte_code) | SHA256(func_ret)): {}.", base64::encode(&combined_bytes_hash));

	let sign = match ecc_ctx.ecdsa_sign_slice(&combined_bytes_hash, &encl_prv_key)
	{
		Ok(val)  => val,
		Err(err) => return err,
	};

	let sign_x: &[u8; 32] = unsafe { std::mem::transmute::<&[u32; 8], &[u8; 32]>(&sign.x) };
	let sign_y: &[u8; 32] = unsafe { std::mem::transmute::<&[u32; 8], &[u8; 32]>(&sign.y) };

	println!("[Enclave]: report signature: {}{}.", base64::encode(&sign_x), base64::encode(&sign_y));

	sgx_status_t::SGX_SUCCESS
}

pub fn gen_prog_states(prog : &mut interpreter::Program, prog_cmd : &ast::cmd::Cmd) -> Result<(), String>
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
			Option::Some(v2) => Result::Err(format!("Program root shouldn't contain return statement; it returned {}.", v2)),
			Option::None     => Result::Err(format!("Program root shouldn't contain return statement; even it's a void return.")),
		},
		Option::None    => Result::Ok(()),
	}
}

pub fn make_entry_call(prog : &interpreter::Program, entry_call : &ast::func_general::FnCall) -> Result<Option<interpreter::exp::ExpValue>, String>
{
	interpreter::states::func_call(&prog.func_states, &prog.var_states, entry_call)
}
