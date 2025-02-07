use enclave_verifier::type_checker;

use enclave_verifier::ast;
use enclave_verifier::ast::Deserializible;

use std::env;

/// Read the example program from a bytecode file
/// 
/// `prog_name` is the name of the program, and an suffix `.impc` will be appended
/// automatically to the end of `prog_name`, so it becomes `<prog_name>.impc`.
/// 
/// We assume the program AST always contains declaration of a function
/// called `fn entry(...) -> ...`, which is the entry point of the program.
/// 
/// This function, `read_byte_code_from_file`, will return a vector of bytes,
/// which can then be passed to Cmd::from_bytes (or Exp, Aexp, Bexp) to construct
/// the AST.
/// 
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
	let args : Vec<String> = env::args().collect();
	if args.len() != 2 {
		panic!("Incorrect number of arguments provided.")
	}

	// Read AST from bytecode file.
	let example_prog_1_name = &args[1];
	let example_prog_1_bytes = read_byte_code_from_file(example_prog_1_name);
	let (_bytes_left_1, example_prog_1) = match ast::cmd::Cmd::from_bytes(&example_prog_1_bytes[..])
	{
		Ok(v) => v,
		Err(why) => panic!("Couldn't construct AST from byte code for {}. {}", example_prog_1_name, why)
	};
	let mut example_prog_1_lines : Vec<ast::IndentString> = vec![];
	example_prog_1.to_indent_lines(&mut example_prog_1_lines);
	println!("Example program {}:\n{}\n", example_prog_1_name, ast::indent_lines_to_string(&example_prog_1_lines, '\t'));


    // Run type-checker on this AST.
	println!("Iteration test:");
	let mut glvar_vec: Vec<type_checker::type_checker::VarTypePair> = Vec::new();
	let mut fn_vec:  Vec<type_checker::type_checker::FuncIdentifierTuple> = Vec::new();
	match type_checker::type_checker::gather_fn_types(&example_prog_1, &mut glvar_vec, &mut fn_vec) {
		Err(err) => println!("Failed type checking:\n{}", err),
		_ => (),
	};
	let var_vec: Vec<type_checker::type_checker::VarTypePair> = Vec::new();
	let res = type_checker::type_checker::iterate_through_ast(example_prog_1, true, var_vec, &fn_vec, ast::data_type::DataType::Void);
	match res {
		Ok(_)    => println!("Successful type checking!"),
		Err(err) => println!("Failed type checking:\n{}", err),
	}
}
