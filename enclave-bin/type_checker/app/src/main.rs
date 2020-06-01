extern crate sgx_types;
extern crate sgx_urts;
use sgx_types::*;
use sgx_urts::SgxEnclave;

use std::env;

static ENCLAVE_FILE: &'static str = "enclave.signed.so";

extern {
	fn type_check_byte_code(eid: sgx_enclave_id_t, retval: *mut sgx_status_t,
		byte_code: *const u8, byte_code_len: usize,
		out_bytes_read: * mut u64,
		out_pkey_x: * mut u8, out_pkey_y: * mut u8,
		out_sign_x: * mut u32, out_sign_y: * mut u32) -> sgx_status_t;
}

fn read_byte_code_from_file(byte_code_dir : &str, prog_name : &str) -> Vec<u8>
{
	use std::fs::File;
	use std::path::Path;
	use std::io::prelude::*;

	let file_path_string = format!("{}/{}.{}", byte_code_dir, prog_name, "impc");
	let file_path = Path::new(&file_path_string);

	let mut file = match File::open(&file_path)
	{
		Err(why) => panic!("[App]: couldn't open {}: {}", file_path.display(), why),
		Ok(file) => file,
	};

	let mut byte_code : Vec<u8> = vec![];

	match file.read_to_end(&mut byte_code)
	{
		Ok(_) => {},
		Err(why) => panic!("[App]: couldn't read from {}: {}", file_path.display(), why),
	}

	println!("[App]: Bytecode file read {} bytes total for program {}.", byte_code.len(), prog_name);

	byte_code
}

fn write_verified_byte_code(byte_code_dir : &str, prog_name : &str, code : &[u8], pkey_x : &[u8; 32], pkey_y : &[u8; 32], sign_x : &[u32; 8], sign_y : &[u32; 8])
{
	use std::fs::File;
	use std::path::Path;
	use std::io::prelude::*;

	let file_path_string = format!("{}/{}.{}", byte_code_dir, prog_name, "vimpc");
	let file_path = Path::new(&file_path_string);

	let mut file = match File::create(&file_path)
	{
		Err(why) => panic!("[App]: couldn't create {}: {}", file_path.display(), why),
		Ok(file) => file,
	};

	println!("[App]: Writing verified program {}.", prog_name);

	match file.write_all(code)
	{
		Ok(_) => {},
		Err(why) => panic!("[App]: couldn't write to {}: {}", file_path.display(), why),
	}
	println!("[App]: Written bytecode {} bytes.", code.len());

	match file.write_all(pkey_x)
	{
		Ok(_) => {},
		Err(why) => panic!("[App]: couldn't write to {}: {}", file_path.display(), why),
	}
	println!("[App]: Written public key x {} bytes.", pkey_x.len());

	match file.write_all(pkey_y)
	{
		Ok(_) => {},
		Err(why) => panic!("[App]: couldn't write to {}: {}", file_path.display(), why),
	}
	println!("[App]: Written public key y {} bytes.", pkey_y.len());

	let sign_x_bytes: &[u8; 32] = unsafe { std::mem::transmute::<&[u32; 8], &[u8; 32]>(sign_x) };
	let sign_y_bytes: &[u8; 32] = unsafe { std::mem::transmute::<&[u32; 8], &[u8; 32]>(sign_y) };

	match file.write_all(sign_x_bytes)
	{
		Ok(_) => {},
		Err(why) => panic!("[App]: couldn't write to {}: {}", file_path.display(), why),
	}
	println!("[App]: Written signature x {} bytes.", sign_x_bytes.len());

	match file.write_all(sign_y_bytes)
	{
		Ok(_) => {},
		Err(why) => panic!("[App]: couldn't write to {}: {}", file_path.display(), why),
	}
	println!("[App]: Written signature x {} bytes.", sign_y_bytes.len());

}

fn init_enclave() -> SgxResult<SgxEnclave>
{
	let mut launch_token: sgx_launch_token_t = [0; 1024];
	let mut launch_token_updated: i32 = 0;

	let debug = 1;
	let mut misc_attr = sgx_misc_attribute_t {secs_attr: sgx_attributes_t { flags:0, xfrm:0}, misc_select:0};
	SgxEnclave::create(ENCLAVE_FILE,
						debug,
						&mut launch_token,
						&mut launch_token_updated,
						&mut misc_attr)
}

fn do_type_check(
	enclave : &SgxEnclave,
	prog_bytes : &[u8], out_bytes_read : &mut usize,
	out_pkey_x : &mut [u8; 32], out_pkey_y : &mut [u8; 32],
	out_sign_x : &mut [u32; 8], out_sign_y : &mut [u32; 8]) -> sgx_status_t
{
	let mut retval = sgx_status_t::SGX_ERROR_UNEXPECTED;
	let out_len : [u64; 4] = [0; 4];
	let out_pkey_x_tmp : [u8; 32] = [0; 32];
	let out_pkey_y_tmp : [u8; 32] = [0; 32];
	let out_sign_x_tmp : [u32; 8] = [0; 8];
	let out_sign_y_tmp : [u32; 8] = [0; 8];

	let result = unsafe {
		type_check_byte_code(enclave.geteid(),
		&mut retval,
		prog_bytes.as_ptr() as * const u8,
		prog_bytes.len(),
		out_len.as_ptr() as * mut u64,
		out_pkey_x_tmp.as_ptr() as * mut u8,
		out_pkey_y_tmp.as_ptr() as * mut u8,
		out_sign_x_tmp.as_ptr() as * mut u32,
		out_sign_y_tmp.as_ptr() as * mut u32)
	};

	match result
	{
		sgx_status_t::SGX_SUCCESS => {},
		_ =>
		{
			println!("[App]: ECALL Enclave Failed {}!", result.as_str());
			return result;
		}
	};

	match retval
	{
		sgx_status_t::SGX_SUCCESS => {},
		_ =>
		{
			println!("[App]: ECALL Enclave Failed {}!", retval.as_str());
			return retval;
		}
	};

	*out_bytes_read = out_len[0] as usize;
	out_pkey_x.copy_from_slice(&out_pkey_x_tmp);
	out_pkey_y.copy_from_slice(&out_pkey_y_tmp);
	out_sign_x.copy_from_slice(&out_sign_x_tmp);
	out_sign_y.copy_from_slice(&out_sign_y_tmp);

	result
}

fn main()
{
	let byte_code_dir : &'static str = "../../../rs-sources";

	let args : Vec<String> = env::args().collect();
	if args.len() != 2
	{
		panic!("[App]: Incorrect number of arguments provided.")
	}

	let enclave = match init_enclave() {
		Ok(r) => {
		println!("[App]: Init Enclave Successful {}!", r.geteid());
		r
	},
		Err(x) => {
			println!("[App]: Init Enclave Failed {}!", x.as_str());
			return;
		},
	};

	let example_prog_name = &args[1];
	let example_prog_bytes = read_byte_code_from_file(byte_code_dir, example_prog_name);
	let mut pkey_x : [u8; 32] = [0; 32];
	let mut pkey_y : [u8; 32] = [0; 32];
	let mut sign_x : [u32; 8] = [0u32; 8];
	let mut sign_y : [u32; 8] = [0u32; 8];
	let mut out_bytes_read : usize = 0;
	do_type_check(&enclave, &example_prog_bytes, &mut out_bytes_read, &mut pkey_x, &mut pkey_y, &mut sign_x, &mut sign_y);
	write_verified_byte_code(byte_code_dir, example_prog_name, &example_prog_bytes[0..out_bytes_read], &pkey_x, &pkey_y, &sign_x, &sign_y);

	enclave.destroy();
}
