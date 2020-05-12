extern crate enclave_verifier;

use enclave_verifier::ast;
use enclave_verifier::type_checker;

fn construct_example_prog() -> ast::aexp::Aexp
{
	use enclave_verifier::ast::*;
	use aexp::constructor_helper::*;

	let fun_exp_list_1 = vec![exp::Exp::A {e : (10i32.to_aexp() + 20i32.to_aexp())}];
	let fun_call_1 = func_general::FnCall::new("foo".to_string(), fun_exp_list_1);

	let fun_exp_list_2 =
		vec![
			exp::Exp::A {e : (5i32.to_aexp() - 2i32.to_aexp())},
			exp::Exp::A {e : (2.5f32.to_aexp() * 2i32.to_aexp())},];
	let fun_call_2 = func_general::FnCall::new("bar".to_string(), fun_exp_list_2);

	aexp::Aexp::FnCall{fc : fun_call_1} + aexp::Aexp::FnCall{fc : fun_call_2} / 1i32.to_aexp() % 2i32.to_aexp() + "x".to_aexp()
}

fn main()
{
	let exp = construct_example_prog();
	println!("{}", exp);

    println!("\nIteration test:\n");
	let exp2 = construct_example_prog();
    type_checker::type_checker::iterate_through_ast(exp2);
}
