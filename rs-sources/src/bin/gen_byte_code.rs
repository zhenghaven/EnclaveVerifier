extern crate enclave_verifier;

use enclave_verifier::ast;

fn construct_example_prog() -> ast::aexp::Aexp
{
	use enclave_verifier::ast::aexp::constructor_helper;
	use constructor_helper::ConstType;

	10i32.to_aexp() + 20i32.to_aexp() - 5.10f32.to_aexp() * 2i32.to_aexp() / 1i32.to_aexp() % 2i32.to_aexp()
}

fn main()
{
	let exp = construct_example_prog();
	let string = ast::aexp::to_string(&exp);

	println!("{}", string);
}
