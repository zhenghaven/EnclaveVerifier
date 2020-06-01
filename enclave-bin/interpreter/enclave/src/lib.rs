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

fn read_pkey_from_bytes(bytes : &[u8]) -> Result<(&[u8], sgx_types::sgx_ec256_public_t), String>
{
	if bytes.len() < 2 * 32
	{
		return Result::Err(format!("The bytes given is smaller than the size of a EC256 public key."));
	}

	let mut pkey : sgx_types::sgx_ec256_public_t = sgx_types::sgx_ec256_public_t{ gx : [0; 32], gy : [0; 32] };

	pkey.gx.copy_from_slice(&bytes[0..32]);
	pkey.gy.copy_from_slice(&bytes[32..64]);

	Result::Ok((&bytes[64..], pkey))
}

fn read_signature_from_bytes(bytes : &[u8]) -> Result<(&[u8], sgx_types::sgx_ec256_signature_t), String>
{
	if bytes.len() < 2 * 32
	{
		return Result::Err(format!("The bytes given is smaller than the size of a EC256 signature."));
	}

	let mut sign : sgx_types::sgx_ec256_signature_t = sgx_types::sgx_ec256_signature_t{ x : [0; 8], y : [0; 8] };

	let sign_x_bytes: &mut [u8; 32] = unsafe { std::mem::transmute::<&mut [u32; 8], &mut [u8; 32]>(&mut sign.x) };
	let sign_y_bytes: &mut [u8; 32] = unsafe { std::mem::transmute::<&mut [u32; 8], &mut [u8; 32]>(&mut sign.y) };

	sign_x_bytes.copy_from_slice(&bytes[0..32]);
	sign_y_bytes.copy_from_slice(&bytes[32..64]);

	Result::Ok((&bytes[64..], sign))
}

#[no_mangle]
pub extern "C" fn interpret_byte_code(byte_code: *const u8, byte_code_len: usize, param_list: *const u8, param_list_len: usize) -> sgx_status_t
{
	// ------------------------------------------
	// 1. Generate EC key pair:
	// ------------------------------------------
	println!("");

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
	println!("");

	let input_slice = unsafe { std::slice::from_raw_parts(byte_code, byte_code_len) };
	let param_list_input_slice = unsafe { std::slice::from_raw_parts(param_list, param_list_len) };

	println!("[Enclave]: Received input ({} byte(s)).", input_slice.len());

	let (input_bytes_left_1, example_prog) = match ast::cmd::Cmd::from_bytes(input_slice)
	{
		Ok(v)    => v,
		Err(why) =>
		{
			println!("[Enclave-ERROR]: Couldn't construct AST from byte code. {}", why);
			return sgx_status_t::SGX_ERROR_UNEXPECTED;
		}
	};

	let byte_code_slice = &input_slice[0..(input_slice.len() - input_bytes_left_1.len())];

	println!("[Enclave]: Received bytecode ({} byte(s)).", byte_code_slice.len());

	let byte_code_hash = match rsgx_sha256_slice(&byte_code_slice)
	{
		Ok(val)  => val,
		Err(err) => return err,
	};

	println!("[Enclave]: Bytecode hash SHA256(byte_code): {}.", base64::encode(&byte_code_hash));

	// ------------------------------------------
	// 3. Read verifier's public key:
	// ------------------------------------------
	println!("");

	let (input_bytes_left_2, verifier_pkey) = match read_pkey_from_bytes(input_bytes_left_1)
	{
		Ok(v)    => v,
		Err(why) =>
		{
			println!("[Enclave-ERROR]: {}", why);
			return sgx_status_t::SGX_ERROR_UNEXPECTED;
		}
	};

	println!("[Enclave]: Verifier's public key {}{}.", base64::encode(&verifier_pkey.gx), base64::encode(&verifier_pkey.gy));

	// ------------------------------------------
	// 4. Read verifier's signature:
	// ------------------------------------------
	println!("");

	let (_input_bytes_left_3, verifier_sign) = match read_signature_from_bytes(input_bytes_left_2)
	{
		Ok(v)    => v,
		Err(why) =>
		{
			println!("[Enclave-ERROR]: {}", why);
			return sgx_status_t::SGX_ERROR_UNEXPECTED;
		}
	};

	let verifier_sign_x: &[u8; 32] = unsafe { std::mem::transmute::<&[u32; 8], &[u8; 32]>(&verifier_sign.x) };
	let verifier_sign_y: &[u8; 32] = unsafe { std::mem::transmute::<&[u32; 8], &[u8; 32]>(&verifier_sign.y) };

	println!("[Enclave]: Verifier's signature {}{}.", base64::encode(verifier_sign_x), base64::encode(verifier_sign_y));

	// ------------------------------------------
	// 5. Verify verifier's signature:
	// ------------------------------------------
	println!("");

	let sign_vrfy_res = match ecc_ctx.ecdsa_verify_msg(&byte_code_hash, &verifier_pkey, &verifier_sign)
	{
		Ok(val)  => val,
		Err(err) => return err,
	};

	if !sign_vrfy_res
	{
		println!("[Enclave-ERROR]: {}", "Failed to verify the signature from verifier.");
		return sgx_status_t::SGX_ERROR_UNEXPECTED;
	}

	println!("[Enclave]: {}", "Signature from verifier is checked.");


	// ------------------------------------------
	// 6. Prepare entry function call from input bytes:
	// ------------------------------------------
	println!("");

	let (param_list_bytes_left, param_list) = match ast::func_general::FnCall::exp_list_from_bytes(param_list_input_slice)
	{
		Result::Ok(val)  => val,
		Result::Err(why) =>
		{
			println!("[Enclave-ERROR]: {}", why);
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
	// 7. Generate program states:
	// ------------------------------------------
	println!("");

	let mut prog_inter = interpreter::Program::new();

	match gen_prog_states(&mut prog_inter, &example_prog)
	{
		Result::Ok(_)    => {},
		Result::Err(why) =>
		{
			println!("[Enclave-ERROR]: {}", why);
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
	// 8. Make entry function call:
	// ------------------------------------------
	println!("");

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
			println!("[Enclave-ERROR]: {}", why);
			return sgx_status_t::SGX_ERROR_UNEXPECTED;
		},
	};

	let func_call_res_bytes = match func_call_res_to_bytes(&func_call_res)
	{
		Result::Ok(ok_val)  => ok_val,
		Result::Err(why)    =>
		{
			println!("[Enclave-ERROR]: {}", why);
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
	// 9. Generate report:
	// ------------------------------------------
	println!("");

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
