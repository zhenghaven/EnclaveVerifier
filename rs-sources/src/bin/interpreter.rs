use enclave_verifier::ast;
use enclave_verifier::interpreter;
use enclave_verifier::ast::*;

use interpreter::states;

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

	use ast::aexp::constructor_helper::ToAexp;
	use ast::exp::constructor_helper::ToExp;

	let example_prog_1_name = "is_prime";
	let example_prog_1_bytes = read_byte_code_from_file(example_prog_1_name);
	let (_bytes_left_1, example_prog_1) = match cmd::Cmd::from_bytes(&example_prog_1_bytes[..])
	{
		Ok(v) => v,
		Err(why) => panic!("Couldn't construct AST from byte code for {}. {}", example_prog_1_name, why)
	};
	let mut example_prog_1_lines : Vec<IndentString> = vec![];
	example_prog_1.to_indent_lines(&mut example_prog_1_lines);
	println!("========================================================");
	println!("Example program {}:", example_prog_1_name);
	println!("{}\n", indent_lines_to_string(&example_prog_1_lines, '\t'));
	println!("========================================================");


	let mut prog_inter_1 = interpreter::Program::new();

	gen_prog_states(&mut prog_inter_1, &example_prog_1);

	println!("========================================================");
	println!("Program global states:");
	println!("----------------------");
	println!("{}", prog_inter_1.func_states);
	println!("{}", prog_inter_1.var_states);
	println!("========================================================");

	let param_list_1 = vec![211i32.to_aexp().to_exp()];

	make_entry_call(&prog_inter_1, param_list_1);

	let param_list_2 = vec![222i32.to_aexp().to_exp()];

	make_entry_call(&prog_inter_1, param_list_2);
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

	match states::func_call(&prog.func_states, &prog.var_states, &entry_call, false)
	{
		Result::Ok(ok_val) => match ok_val
		{
			Option::Some(v) => println!("Function call {} returned {}", entry_call, v),
			Option::None    => println!("Function call {} didn't return any value.", entry_call)
		},
		Result::Err(why)   => panic!("Entry call failed. {}", why),
	}
}
