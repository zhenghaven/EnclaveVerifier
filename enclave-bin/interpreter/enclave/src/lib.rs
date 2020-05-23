#![crate_name = "enclave_vrfy_interpreter"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
extern crate sgx_trts;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

extern crate enclave_verifier;

use sgx_types::*;
//use sgx_types::metadata::*;
//use sgx_trts::enclave;
//use sgx_trts::{is_x86_feature_detected, is_cpu_feature_supported};
//use std::string::String;
use std::vec::Vec;
//use std::io::{self, Write};
//use std::slice;
//use std::backtrace::{self, PrintFormat};

use enclave_verifier::ast::*;

#[no_mangle]
pub extern "C" fn interpret_byte_code(byte_code: *const u8, some_len: usize) -> sgx_status_t
{
	let byte_code_slice = unsafe { std::slice::from_raw_parts(byte_code, some_len) };

	println!("[Enclave]: Received bytecode ({} byte(s)).", byte_code_slice.len());

	let (_bytes_left, example_prog) = match cmd::Cmd::from_bytes(byte_code_slice)
	{
		Ok(v) => v,
		Err(why) => panic!("[Enclave]: Couldn't construct AST from byte code. {}", why)
	};
	let mut example_prog_lines : Vec<IndentString> = vec![];
	example_prog.to_indent_lines(&mut example_prog_lines);
	println!("[Enclave]: Example program:\n{}\n", indent_lines_to_string(&example_prog_lines, '\t'));

	sgx_status_t::SGX_SUCCESS
}
