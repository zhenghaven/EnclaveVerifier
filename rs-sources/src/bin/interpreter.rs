use enclave_verifier::interpreter;

use enclave_verifier::ast::*;

fn read_byte_code_from_file(prog_name : &str) -> Vec<u8>
{
	use std::fs::File;
	use std::path::Path;
	use std::io::prelude::*;

	let file_path_string = format!("{}.{}", prog_name, "impc");
	let file_path = Path::new(&file_path_string);

	let mut file = match File::open(&file_path)
	{
        Err(why) => panic!("couldn't create {}: {}", file_path.display(), why),
        Ok(file) => file,
	};
	
	let mut byte_code : Vec<u8> = vec![];

	match file.read_to_end(&mut byte_code)
	{
		Ok(_) => {},
		Err(why) => panic!("couldn't read from {}: {}", file_path.display(), why),
	}

	println!("Bytecode file read {} bytes total for program {}.", byte_code.len(), prog_name);

	byte_code
}

fn main()
{
	let example_prog_1_name = "is_prime";
	let example_prog_1_bytes = read_byte_code_from_file(example_prog_1_name);
	let (_bytes_left_1, example_prog_1) = match cmd::Cmd::from_bytes(&example_prog_1_bytes[..])
	{
		Ok(v) => v,
		Err(why) => panic!("Couldn't construct AST from byte code for {}. {}", example_prog_1_name, why)
	};
	let mut example_prog_1_lines : Vec<IndentString> = vec![];
	example_prog_1.to_indent_lines(&mut example_prog_1_lines);
	println!("Example program {}:\n{}\n", example_prog_1_name, indent_lines_to_string(&example_prog_1_lines, '\t'));

	interpreter::eval();
}
