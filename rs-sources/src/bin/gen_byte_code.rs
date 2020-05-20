extern crate enclave_verifier;

use enclave_verifier::ast::*;

/// Example program 1 - function 1
fn is_divisible(x : i32, factor : i32) -> bool
{
	return (x % factor) == 0;
}

/// Example program 1 - function 2
fn is_prime(x : i32) -> bool
{
	if x <= 1
	{
		return false;
	}

	let mut is_prime : bool;
	is_prime = true;

	let mut test_num : i32;
	test_num = x / 2;

	while is_prime && (test_num > 1)
	{
		if is_divisible(x, test_num)
		{
			is_prime = false;
		}
		
		test_num = test_num - 1;
	}

	return is_prime;
}

fn construct_example_prog_bexps() -> cmd::Cmd
{
    use bexp::constructor_helper::*;
    use exp::constructor_helper::*;
    use cmd::constructor_helper::*;

	let eq_var_dec = var_dc(var_general::VarDecl::new(data_type::DataType::Bool, "x".to_string()));
    let bexp_eq = assign(var_general::VarRef::from_str("x"), (false.to_bexp().beq(true.to_bexp())).to_exp());
    let prog = seq(eq_var_dec, seq(bexp_eq, skip()));
    prog
}

fn construct_example_prog_1() -> cmd::Cmd
{
	use aexp::constructor_helper::*;
	use bexp::constructor_helper::*;
	use exp::constructor_helper::*;
	use cmd::constructor_helper::*;

	// --- function 1 - is_divisible
	
	//x : i32, factor : i32
	let var_decl_list_1 = vec![
		var_general::VarDecl::new(data_type::DataType::Int32, "x".to_string()),
		var_general::VarDecl::new(data_type::DataType::Int32, "factor".to_string())
	];

	//fn is_divisible(x : i32, factor : i32) -> bool
	let fn_prototype_1 = func_general::FnProtoType::new(data_type::DataType::Bool, "is_divisible".to_string(), var_decl_list_1);

	//return (x % factor) == 0;
	let fn_cmd_1 = ret(("x".to_aexp() % "factor".to_aexp()).aeq(0i32.to_aexp()).to_exp());

	//Constrcut Function ---
	let fn_1 = fn_dc(fn_prototype_1, fn_cmd_1);


	// --- function 2 - is_prime

	//x : i32
	let var_decl_list_2 = vec![
		var_general::VarDecl::new(data_type::DataType::Int32, "x".to_string()),
	];

	//fn is_prime(x : i32) -> bool
	let fn_prototype_2 = func_general::FnProtoType::new(data_type::DataType::Bool, "entry".to_string(), var_decl_list_2);

	//if x <= 1 { return false; } else { skip; }
	let fn_cmd_2_seq_1 = if_el("x".to_aexp().lte(1i32.to_aexp()), ret(false.to_bexp().to_exp()), skip());
	//let mut is_prime : bool;
	let fn_cmd_2_seq_2 = var_dc(var_general::VarDecl::new(data_type::DataType::Bool, "is_prime".to_string()));
	//is_prime = true;
	let fn_cmd_2_seq_3 = assign(var_general::VarRef::from_str("is_prime"), true.to_bexp().to_exp());
	//let mut test_num : i32;
	let fn_cmd_2_seq_4 = var_dc(var_general::VarDecl::new(data_type::DataType::Int32, "test_num".to_string()));
	//test_num = x / 2;
	let fn_cmd_2_seq_5 = assign(var_general::VarRef::from_str("test_num"), ("x".to_aexp() / 2i32.to_aexp()).to_exp());
	//if is_divisible(x, test_num) { is_prime = false; }
	let fn_cmd_2_seq_6 = if_el(
		bexp::Bexp::FnCall{
			fc : func_general::FnCall::new("is_divisible".to_string(), vec!["x".to_aexp().to_exp(), "test_num".to_aexp().to_exp()])},
		assign(var_general::VarRef::from_str("is_prime"), false.to_bexp().to_exp()),
		skip());
	//test_num = test_num - 1;
	let fn_cmd_2_seq_7 = assign(var_general::VarRef::from_str("test_num"), ("test_num".to_aexp() - 1i32.to_aexp()).to_exp());
	//while is_prime && (test_num > 1) { seq{fn_cmd_2_seq_x, fn_cmd_2_seq_x} }
	let fn_cmd_2_seq_8 = wh_lp("is_prime".to_bexp().and("test_num".to_aexp().gt(1i32.to_aexp())), seq(fn_cmd_2_seq_6, fn_cmd_2_seq_7));
	//return is_prime;
	let fn_cmd_2_seq_9 = ret("is_prime".to_bexp().to_exp());

	//Constrcut Seq ---
	let fn_cmd_2 = seq(
		fn_cmd_2_seq_1, seq(
			fn_cmd_2_seq_2, seq(
				fn_cmd_2_seq_3, seq(
					fn_cmd_2_seq_4, seq(
						fn_cmd_2_seq_5, seq(
							fn_cmd_2_seq_8, seq(
								fn_cmd_2_seq_9, skip())))))));

	//Constrcut Function ---
	let fn_2 = fn_dc(fn_prototype_2, fn_cmd_2);

	// --- Constrcut Program ---
	let prog = seq(fn_1, fn_2);

	prog
}

fn write_byte_code_to_file(code : &cmd::Cmd, prog_name : &str)
{
	use std::fs::File;
	use std::path::Path;
	use std::io::prelude::*;

	let file_path_string = format!("{}.{}", prog_name, "impc");
	let file_path = Path::new(&file_path_string);

	let mut file = match File::create(&file_path)
	{
        Err(why) => panic!("couldn't create {}: {}", file_path.display(), why),
        Ok(file) => file,
	};
	
	let byte_code = match code.to_bytes()
	{
		Err(why) => panic!("Couldn't generate byte code for {}. {}", prog_name, why),
		Ok(byte_code) => byte_code
	};

	println!("Bytecode generated for {} ({} bytes total).", prog_name, byte_code.len());

	match file.write_all(&byte_code)
	{
        Err(why) => panic!("couldn't write to {}: {}", file_path.display(), why),
        Ok(_) => println!("successfully wrote to {}", file_path.display()),
    }
}

fn main()
{
	println!("");

	println!("Example function test result is_prime(x = 211): {}\n", is_prime(211i32));
	println!("Example function test result is_prime(x = 222): {}\n", is_prime(222i32));

	let example_prog_1_name = "is_prime";
    //let example_prog_1 = construct_example_prog_bexps();
	let example_prog_1 = construct_example_prog_1();
	let mut example_prog_1_lines : Vec<IndentString> = vec![];
	example_prog_1.to_indent_lines(&mut example_prog_1_lines);
	println!("Example program {}:\n{}\n", example_prog_1_name, indent_lines_to_string(&example_prog_1_lines, '\t'));

	write_byte_code_to_file(&example_prog_1, &example_prog_1_name);

	println!("");
}
