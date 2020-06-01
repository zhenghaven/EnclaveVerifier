#![crate_name = "enclave_vrfy_type_checker"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
extern crate sgx_trts;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

use sgx_types::*;

//use std::string::String;
use std::vec::Vec;

extern crate enclave_verifier;

use enclave_verifier::ast;
use enclave_verifier::ast::Deserializible;
use enclave_verifier::type_checker;

#[no_mangle]
pub extern "C" fn type_check_byte_code(
	byte_code: *const u8, byte_code_len: usize,
	out_bytes_read: * mut u64,
	out_pkey_x: * mut u8, out_pkey_y: * mut u8,
	out_sign_x: * mut u32, out_sign_y: * mut u32) -> sgx_status_t
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

	let out_pkey_x_slice = unsafe { std::slice::from_raw_parts_mut(out_pkey_x, 32) };
	let out_pkey_y_slice = unsafe { std::slice::from_raw_parts_mut(out_pkey_y, 32) };
	out_pkey_x_slice.copy_from_slice(&encl_pub_key.gx);
	out_pkey_y_slice.copy_from_slice(&encl_pub_key.gy);

	// ------------------------------------------
	// 2. Process input bytes:
	// ------------------------------------------
	println!("");

	let input_slice = unsafe { std::slice::from_raw_parts(byte_code, byte_code_len) };

	println!("[Enclave]: Received input ({} byte(s)).", input_slice.len());

	let (input_bytes_left, example_prog) = match ast::cmd::Cmd::from_bytes(input_slice)
	{
		Ok(v)    => v,
		Err(why) =>
		{
			println!("[Enclave-ERROR]: Couldn't construct AST from byte code. {}", why);
			return sgx_status_t::SGX_ERROR_UNEXPECTED;
		}
	};

	let prog_bytes_read_len : usize = input_slice.len() - input_bytes_left.len();
	println!("[Enclave]: Received bytecode ({} byte(s)).", prog_bytes_read_len);
	let out_bytes_read_slice = unsafe { std::slice::from_raw_parts_mut(out_bytes_read, 4) };
	out_bytes_read_slice[0] = prog_bytes_read_len as u64;

	let byte_code_slice = &input_slice[0..(input_slice.len() - input_bytes_left.len())];

	let byte_code_hash = match sgx_tcrypto::rsgx_sha256_slice(&byte_code_slice)
	{
		Ok(val)  => val,
		Err(err) => return err,
	};

	println!("[Enclave]: Bytecode hash SHA256(byte_code): {}.", base64::encode(&byte_code_hash));

	let mut example_prog_lines : Vec<ast::IndentString> = vec![];
	example_prog.to_indent_lines(&mut example_prog_lines);
	println!("[Enclave]: Example program:\n{}\n", ast::indent_lines_to_string(&example_prog_lines, '\t'));


	// ------------------------------------------
	// 3. Verification:
	// ------------------------------------------
	println!("");

	println!("[Enclave]: Iteration test:");
	let var_vec: Vec<type_checker::type_checker::VarTypePair> = Vec::new();
	let mut fn_vec:  Vec<type_checker::type_checker::FuncIdentifierTuple> = Vec::new();
	type_checker::type_checker::gather_fn_types(&example_prog, &mut fn_vec);
	let res = type_checker::type_checker::iterate_through_ast(example_prog, var_vec, &fn_vec, ast::data_type::DataType::Void);
	match res
	{
		Ok(_)    => println!("[Enclave]: Successful type checking!"),
		Err(why) =>
		{
			println!("[Enclave]: Failed type checking:\n{}", why);
			return sgx_status_t::SGX_ERROR_UNEXPECTED;
		}
	}


	// ------------------------------------------
	// 4. Generate signature:
	// ------------------------------------------
	println!("");

	let sign = match ecc_ctx.ecdsa_sign_slice(&byte_code_hash, &encl_prv_key)
	{
		Ok(val)  => val,
		Err(err) => return err,
	};

	let sign_x: &[u8; 32] = unsafe { std::mem::transmute::<&[u32; 8], &[u8; 32]>(&sign.x) };
	let sign_y: &[u8; 32] = unsafe { std::mem::transmute::<&[u32; 8], &[u8; 32]>(&sign.y) };

	println!("[Enclave]: report signature: {}{}.", base64::encode(&sign_x), base64::encode(&sign_y));

	let out_sign_x_slice = unsafe { std::slice::from_raw_parts_mut(out_sign_x, 8) };
	let out_sign_y_slice = unsafe { std::slice::from_raw_parts_mut(out_sign_y, 8) };

	out_sign_x_slice.copy_from_slice(&sign.x);
	out_sign_y_slice.copy_from_slice(&sign.y);

	sgx_status_t::SGX_SUCCESS
}
