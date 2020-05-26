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

fn construct_example_prog_ifel() -> cmd::Cmd
{
	use aexp::constructor_helper::*;
	use exp::constructor_helper::*;
	use cmd::constructor_helper::*;

    /* Program:
     * fn entry(x: Int32) -> () {
     *   if (x >= 0) && (x <= 9) {
     *     Int32 y;
     *     y = x + 1;
     *     return entry(y);
     *   } else {
     *     return x;
     *   }
     * } */

	// arg list: x: Int32
	let var_decl_list = vec![
		var_general::VarDecl::new(data_type::DataType::Int32, "x".to_string()),
    ];

	//fn entry(x: Int32) -> ()
	let fn_prototype = func_general::FnProtoType::new(data_type::DataType::Int32, "entry".to_string(), var_decl_list);

	// Int32 y; y = x + 1; return entry(y);
	let y_dec = var_dc(var_general::VarDecl::new(data_type::DataType::Int32, "y".to_string()));
	let y_asg = assign(var_general::VarRef::from_str("y"), ("x".to_aexp() + 1i32.to_aexp()).to_exp());
	let ret_t = ret(
		(aexp::Aexp::FnCall{
			fc : func_general::FnCall::new("entry".to_string(), vec!["y".to_aexp().to_exp()])
		}).to_exp()
	);
    let seq_t = seq(y_dec, seq(y_asg, ret_t));

    // return x;
	let ret_f = ret(("x".to_aexp()).to_exp());


	let x_gte_0 = "x".to_aexp().gte(0i32.to_aexp());
	let x_lte_9 = "x".to_aexp().lte(9i32.to_aexp());

	let if_el_cmd = if_el(x_gte_0.and(x_lte_9), seq_t, ret_f);

	let entry_decl = fn_dc(fn_prototype, if_el_cmd);
    entry_decl
}

fn construct_example_prog_overloading() -> cmd::Cmd
{
	use aexp::constructor_helper::*;
	use bexp::constructor_helper::*;
	use exp::constructor_helper::*;
	use cmd::constructor_helper::*;

    /* Program:
     * fn entry() -> () {
     *   Int32 t0 = 5;
     *   Int32 t1 = 10;
     *   Bool  t2 = true;
     *
     *   Int32   o1 = overloaded(t0, t1);
     *   Bool    o2 = overloaded(t0, t2);
     *   Float32 c3 = overloaded(t2, t1);
     * };
     *
     * fn overloaded(x : Int32, y : Int32) -> Int32 {
     *   return x+y
     * };
     *
     * fn overloaded(x : Int32, y : Bool) -> Bool {
     *   while (x > 0) {
     *     x--;
     *     y = !y
     *   };
     *   return y;
     * };
     *
     * fn overloaded(x : Bool, y : Int32) -> Float32 {
     *   if x {
     *     return y;
     *   } else {
     *     return y+1;
     *   }
     * }
     */

	// arg list:
	let var_decl_list = vec![];

	// fn entry() -> ()
	let fn_prototype = func_general::FnProtoType::new(data_type::DataType::Int32, "entry".to_string(), var_decl_list);

	// Int32 t0 = 5;
	let t0_dec = var_dc(var_general::VarDecl::new(data_type::DataType::Int32, "t0".to_string()));
	let t0_asg = assign(var_general::VarRef::from_str("t0"), 5i32.to_aexp().to_exp());

	// Int32 t1 = 10;
	let t1_dec = var_dc(var_general::VarDecl::new(data_type::DataType::Int32, "t1".to_string()));
	let t1_asg = assign(var_general::VarRef::from_str("t1"), 10i32.to_aexp().to_exp());

	// Bool t2 = true;
	let t2_dec = var_dc(var_general::VarDecl::new(data_type::DataType::Bool, "t2".to_string()));
	let t2_asg = assign(var_general::VarRef::from_str("t2"), true.to_bexp().to_exp());

	// Int32 o1 = overloaded(t0, t1);
	let o1_dec = var_dc(var_general::VarDecl::new(data_type::DataType::Int32, "o1".to_string()));
	let o1_asg = assign(var_general::VarRef::from_str("o1"),
		(aexp::Aexp::FnCall{
			fc : func_general::FnCall::new("overloaded".to_string(), vec!["t0".to_aexp().to_exp(), "t1".to_aexp().to_exp()])
		}).to_exp()
	);

	// Bool o2 = overloaded(t0, t2);
	let o2_dec = var_dc(var_general::VarDecl::new(data_type::DataType::Bool, "o2".to_string()));
	let o2_asg = assign(var_general::VarRef::from_str("o2"),
		(aexp::Aexp::FnCall{
			fc : func_general::FnCall::new("overloaded".to_string(), vec!["t0".to_aexp().to_exp(), "t2".to_bexp().to_exp()])
		}).to_exp()
	);

	// Float32 o3 = overloaded(t2, t1);
	let o3_dec = var_dc(var_general::VarDecl::new(data_type::DataType::Float32, "o3".to_string()));
	let o3_asg = assign(var_general::VarRef::from_str("o3"),
		(aexp::Aexp::FnCall{
			fc : func_general::FnCall::new("overloaded".to_string(), vec!["t2".to_bexp().to_exp(), "t1".to_aexp().to_exp()])
		}).to_exp()
	);

	let seq_entry = seq(t0_dec, seq(t0_asg, seq(t1_dec, seq(t1_asg, seq(t2_dec, seq(t2_asg, seq(o1_dec, seq(o1_asg, seq(o2_dec, seq(o2_asg, seq(o3_dec, o3_asg)))))))))));

	let entry_decl = fn_dc(fn_prototype, seq_entry);


	// overloaded(Int32, Int32)
	// arg list: x: Int32, y: Int32
	let var_decl_list_o1 = vec![
		var_general::VarDecl::new(data_type::DataType::Int32, "x".to_string()),
		var_general::VarDecl::new(data_type::DataType::Int32, "y".to_string()),
    ];

	// fn overloaded(x: Int32, y: Int32) -> Int32
	let fn_prototype_o1 = func_general::FnProtoType::new(data_type::DataType::Int32, "overloaded".to_string(), var_decl_list_o1);

	// return x+y
	let ret_o1 = ret(("x".to_aexp() + "y".to_aexp()).to_exp());
	let overloaded1_dec = fn_dc(fn_prototype_o1, ret_o1);


    // overloaded(Int32, Bool)
	// arg list: x: Int32, y: Bool
	let var_decl_list_o2 = vec![
		var_general::VarDecl::new(data_type::DataType::Int32, "x".to_string()),
		var_general::VarDecl::new(data_type::DataType::Bool,  "y".to_string()),
    ];

	// fn overloaded(x: Int32, y: Bool) -> Int32
	let fn_prototype_o2 = func_general::FnProtoType::new(data_type::DataType::Bool, "overloaded".to_string(), var_decl_list_o2);

	//while x > 0 { x = x - 1; y = y == false }
	let x_sub = assign(var_general::VarRef::from_str("x"), ("x".to_aexp() - 1i32.to_aexp()).to_exp());
	let y_not = assign(var_general::VarRef::from_str("y"), ("y".to_bexp().beq(false.to_bexp())).to_exp());
	let while_o2 = wh_lp("x".to_aexp().gt(0i32.to_aexp()), seq(x_sub, y_not));

    // return y
	let ret_o2 = ret(("y".to_bexp()).to_exp());

	let overloaded2_dec = fn_dc(fn_prototype_o2, seq(while_o2, ret_o2));


    // overloaded(Bool, Int32)
	// arg list: x: Bool, y: Int32
	let var_decl_list_o3 = vec![
		var_general::VarDecl::new(data_type::DataType::Bool,  "x".to_string()),
		var_general::VarDecl::new(data_type::DataType::Int32, "y".to_string()),
    ];

	// fn overloaded(x: Bool, y: Int32) -> Float32
	let fn_prototype_o3 = func_general::FnProtoType::new(data_type::DataType::Float32, "overloaded".to_string(), var_decl_list_o3);

	// if x { return y } else { return y+1 }
	let ret_o3t  = ret(("y".to_aexp()).to_exp());
	let ret_o3f  = ret(("y".to_aexp() + 1i32.to_aexp()).to_exp());
	let if_el_o3 = if_el("x".to_bexp(), ret_o3t, ret_o3f);

	let overloaded3_dec = fn_dc(fn_prototype_o3, if_el_o3);

    // Full program
    seq(entry_decl, seq(overloaded1_dec, seq(overloaded2_dec, overloaded3_dec)))
}


fn construct_example_prog_bexps() -> cmd::Cmd
{
	use aexp::constructor_helper::*;
	use bexp::constructor_helper::*;
	use exp::constructor_helper::*;
	use cmd::constructor_helper::*;

	// empty arg list
	let var_decl_list = vec![];

	//fn entry() -> ()
	let fn_prototype = func_general::FnProtoType::new(data_type::DataType::Void, "entry".to_string(), var_decl_list);

	// bool ==
	let beq_var_dec = var_dc(var_general::VarDecl::new(data_type::DataType::Bool, "x0".to_string()));
	let bexp_beq = assign(var_general::VarRef::from_str("x0"), (false.to_bexp().beq(true.to_bexp())).to_exp());
	let seq_beq = seq(beq_var_dec, bexp_beq);

	// bool !=
	let bneq_var_dec = var_dc(var_general::VarDecl::new(data_type::DataType::Bool, "x1".to_string()));
	let bexp_bneq = assign(var_general::VarRef::from_str("x1"), ("x0".to_bexp().bneq(true.to_bexp())).to_exp());
	let seq_bneq = seq(bneq_var_dec, bexp_bneq);

	// &&
	let and_var_dec = var_dc(var_general::VarDecl::new(data_type::DataType::Bool, "x2".to_string()));
	let bexp_and = assign(var_general::VarRef::from_str("x2"), ("x0".to_bexp().and("x1".to_bexp())).to_exp());
	let seq_and = seq(and_var_dec, bexp_and);

	// ||
	let or_var_dec = var_dc(var_general::VarDecl::new(data_type::DataType::Bool, "x3".to_string()));
	let bexp_or = assign(var_general::VarRef::from_str("x3"), ("x1".to_bexp().or("x2".to_bexp())).to_exp());
	let seq_or = seq(or_var_dec, bexp_or);

	// arith ==
	let aeq_var_dec = var_dc(var_general::VarDecl::new(data_type::DataType::Bool, "x4".to_string()));
	let bexp_aeq = assign(var_general::VarRef::from_str("x4"), (0i32.to_aexp().aeq(1i32.to_aexp())).to_exp());
	let seq_aeq = seq(aeq_var_dec, bexp_aeq);

	// arith !=
	let aneq_var_dec = var_dc(var_general::VarDecl::new(data_type::DataType::Bool, "x5".to_string()));
	let bexp_aneq = assign(var_general::VarRef::from_str("x5"), (0i32.to_aexp().aneq(1i32.to_aexp())).to_exp());
	let seq_aneq = seq(aneq_var_dec, bexp_aneq);

	// arith <
	let lt_var_dec = var_dc(var_general::VarDecl::new(data_type::DataType::Bool, "x6".to_string()));
	let bexp_lt = assign(var_general::VarRef::from_str("x6"), (0i32.to_aexp().lt(1i32.to_aexp())).to_exp());
	let seq_lt = seq(lt_var_dec, bexp_lt);

	// arith <=
	let lte_var_dec = var_dc(var_general::VarDecl::new(data_type::DataType::Bool, "x7".to_string()));
	let bexp_lte = assign(var_general::VarRef::from_str("x7"), (0i32.to_aexp().lte(1i32.to_aexp())).to_exp());
	let seq_lte = seq(lte_var_dec, bexp_lte);

	// arith >
	let gt_var_dec = var_dc(var_general::VarDecl::new(data_type::DataType::Bool, "x8".to_string()));
	let bexp_gt = assign(var_general::VarRef::from_str("x8"), (0i32.to_aexp().gt(1i32.to_aexp())).to_exp());
	let seq_gt = seq(gt_var_dec, bexp_gt);

	// arith >=
	let gte_var_dec = var_dc(var_general::VarDecl::new(data_type::DataType::Bool, "x9".to_string()));
	let bexp_gte = assign(var_general::VarRef::from_str("x9"), (0i32.to_aexp().lte(1i32.to_aexp())).to_exp());
	let seq_gte = seq(gte_var_dec, bexp_gte);

	let fn_cmds = seq(seq_beq, seq(seq_bneq, seq(seq_and, seq(seq_or, seq(seq_aeq, seq(seq_aneq, seq(seq_lt, seq(seq_lte, seq(seq_gt, seq_gte)))))))));
	fn_dc(fn_prototype, fn_cmds)
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

	//---------------
	// Example prog 1: is_prime
	//---------------

	let example_prog_1_name = "is_prime";
	let example_prog_1 = construct_example_prog_1();
	let mut example_prog_1_lines : Vec<IndentString> = vec![];
	example_prog_1.to_indent_lines(&mut example_prog_1_lines);
	println!("Example program {}:\n{}\n", example_prog_1_name, indent_lines_to_string(&example_prog_1_lines, '\t'));

	write_byte_code_to_file(&example_prog_1, &example_prog_1_name);

	println!("===================================================\n");

	//---------------
	// Example prog 2: test for bexps
	//---------------

	let example_prog_2_name = "test_bexps";
	let example_prog_2 = construct_example_prog_bexps();
	//let example_prog_1 = construct_example_prog_ifel();
	let mut example_prog_2_lines : Vec<IndentString> = vec![];
	example_prog_2.to_indent_lines(&mut example_prog_2_lines);
	println!("Example program {}:\n{}\n", example_prog_2_name, indent_lines_to_string(&example_prog_2_lines, '\t'));

	write_byte_code_to_file(&example_prog_2, &example_prog_2_name);

	println!("===================================================\n");

	//---------------
	// Example prog 3: test for ifelse
	//---------------

	let example_prog_3_name = "test_ifel";
	let example_prog_3 = construct_example_prog_ifel();
	let mut example_prog_3_lines : Vec<IndentString> = vec![];
	example_prog_3.to_indent_lines(&mut example_prog_3_lines);
	println!("Example program {}:\n{}\n", example_prog_3_name, indent_lines_to_string(&example_prog_3_lines, '\t'));

	write_byte_code_to_file(&example_prog_3, &example_prog_3_name);

	println!("===================================================\n");

	//---------------
	// Example prog 4: test for function overloading
	//---------------

	let example_prog_4_name = "test_overloading";
	let example_prog_4 = construct_example_prog_overloading();
	let mut example_prog_4_lines : Vec<IndentString> = vec![];
	example_prog_4.to_indent_lines(&mut example_prog_4_lines);
	println!("Example program {}:\n{}\n", example_prog_4_name, indent_lines_to_string(&example_prog_4_lines, '\t'));

	write_byte_code_to_file(&example_prog_4, &example_prog_4_name);

	println!("===================================================\n");
}
