extern crate sgx_types;
extern crate sgx_urts;
use sgx_types::*;
use sgx_urts::SgxEnclave;

use std::env;

extern crate enclave_verifier;

static ENCLAVE_FILE: &'static str = "enclave.signed.so";

extern {
	fn interpret_byte_code(eid: sgx_enclave_id_t, retval: *mut sgx_status_t,
		byte_code: *const u8, byte_code_len: usize,
		param_list: *const u8, param_list_len: usize,) -> sgx_status_t;
}

fn read_byte_code_from_file(byte_code_dir : &str, prog_name : &str, suffix : &str) -> Vec<u8>
{
	use std::fs::File;
	use std::path::Path;
	use std::io::prelude::*;

	let file_path_string = format!("{}/{}.{}", byte_code_dir, prog_name, suffix);
	let file_path = Path::new(&file_path_string);

	let mut file = match File::open(&file_path)
	{
		Err(why) => panic!("couldn't open {}: {}", file_path.display(), why),
		Ok(file) => file,
	};

	let mut byte_code : Vec<u8> = vec![];

	match file.read_to_end(&mut byte_code)
	{
		Ok(_) => {},
		Err(why) => panic!("couldn't read from {}: {}", file_path.display(), why),
	}

	println!("[App]: Bytecode file read {} bytes total for program {}.", byte_code.len(), prog_name);

	byte_code
}

fn make_encl_func_call(enclave : &SgxEnclave, prog_bytes : &[u8], param_list_bytes : &[u8]) -> sgx_status_t
{
	let mut retval = sgx_status_t::SGX_SUCCESS;

	let result = unsafe {
		interpret_byte_code(enclave.geteid(),
		&mut retval,
		prog_bytes.as_ptr() as * const u8,
		prog_bytes.len(),
		param_list_bytes.as_ptr() as * const u8,
		param_list_bytes.len())
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
			println!("[App]: ECALL Enclave returned {}!", retval.as_str());
			return retval;
		}
	};

	sgx_status_t::SGX_SUCCESS
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

fn main()
{

	let byte_code_dir : &'static str = "../../../rs-sources";

	let args : Vec<String> = env::args().collect();
	if args.len() != 3
	{
		panic!("[App]: Incorrect number of arguments provided.")
	}

	let example_prog_name = &args[1];
	let example_param_name = &args[2];
	let example_prog_bytes = read_byte_code_from_file(byte_code_dir, example_prog_name, "vimpc");
	let example_param_bytes = read_byte_code_from_file(byte_code_dir, example_param_name, "param");

	println!("[App]: Read bytecode file ({} byte(s)).", example_prog_bytes.len());

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

	make_encl_func_call(&enclave, &example_prog_bytes, &example_param_bytes);

	enclave.destroy();
}
