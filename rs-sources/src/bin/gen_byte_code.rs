extern crate enclave_verifier;

use enclave_verifier::ast::*;

fn construct_example_prog() -> exp::Exp
{
	use aexp::constructor_helper::*;
	use exp::constructor_helper::*;

	let fun_exp_list_1 = vec![(10i32.to_aexp() + 20i32.to_aexp()).to_exp()];
	let fun_call_1 = func_general::FnCall::new("foo".to_string(), fun_exp_list_1);

	let fun_exp_list_2 = vec![
			(5i32.to_aexp() - 2i32.to_aexp()).to_exp(),
			(2.5f32.to_aexp() * 2i32.to_aexp()).to_exp(),
		];
	let fun_call_2 = func_general::FnCall::new("bar".to_string(), fun_exp_list_2);

	(aexp::Aexp::FnCall{fc : fun_call_1} + aexp::Aexp::FnCall{fc : fun_call_2} / 1i32.to_aexp() % 2i32.to_aexp() + "x".to_aexp()).to_exp()
}

fn main()
{
	println!("");

	let example_exp = construct_example_prog();
	println!("Example Exp:\n{}\n", example_exp);

	let byte_code = match example_exp.to_bytes()
	{
		Ok(val) => val,
		Err(err_msg) => panic!(err_msg)
	};

	println!("Bytecode:\n{:?} \n\n", byte_code);

	let (bytes_left, exp_from_byte) = match exp::Exp::from_bytes(&byte_code[..])
	{
		Ok(val) => val,
		Err(err_msg) => panic!(err_msg)
	};


	println!("Byte left:\n{:?} \n\n", bytes_left);
	println!("Exp from bytecode:\n{}\n", exp_from_byte);

	println!("");
}
