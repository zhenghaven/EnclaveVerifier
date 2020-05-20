use crate::ast;

pub struct VarTypePair(String, ast::data_type::DataType);

/* This iterates over the commands that make up the program.
 * Depending on what kind of command we're looking at, we
 * will type-check each one of the sub-expressions of the
 * current command based off whether they're AExps or BExps. */
pub fn iterate_through_ast(cmd: ast::cmd::Cmd, var_types: &mut std::vec::Vec<VarTypePair>) -> Result<&std::vec::Vec<VarTypePair>, String> {
    println!("cmd: {}", cmd);
    match cmd {
        ast::cmd::Cmd::Skip => Result::Ok(var_types),
        ast::cmd::Cmd::VarDecl{d} => {
            //1. Check to see if var's type was already declared.
            //2. If so, error.
            //3. If not, then add to var_types vector and pass to next command.
            let var = *d;
            if (*var_types).iter().any(|i| i.0 == var.name) {
                //FIXME: Find a way to add variable name to error message.
                Result::Err("Error: variable () was declared more than once.". to_string())
            } else {
                (*var_types).push(VarTypePair(var.name, var.var_type));
                Result::Ok(var_types)
            }
        },
        ast::cmd::Cmd::Assign{var, e} => {
            //1. Figure out type of LHS, make sure it was previously declared.
            //1. Figure out type of RHS + make sure it is well-typed.
            //2. Check to make sure LHS var is:
            //   a. Already declared.
            //   b. Type(LHS) = Type(RHS)

            //FIXME: Currently I iterate over, but don't do anything with this.
            let (prev_decl, decl_type) = get_var_type(var_types, (*var).name);

            if prev_decl == false {
                Result::Err("Error: An assign uses variable () which has not yet been declared.". to_string())
            } else {
                match *e {
                    ast::exp::Exp::A{e} => {
                        let tmp = check_aexpr_type(e);
                        Result::Err("bleh". to_string())
                    },
                    ast::exp::Exp::B{e} => {
                        let tmp = check_bexpr_type(e, var_types);
                        if tmp.is_ok() {
                            if tmp == Result::Ok(decl_type) {
                                Result::Ok(var_types)
                            } else {
                                Result::Err("Error: assign's LHS type does not match RHS type.". to_string())
                            }
                        } else {
                            Result::Err("Error: assign failed on not well-typed bexp.". to_string())
                            //panic!("Error: bexpr {} checked is not well typed.", e);
                        }
                    },
                }
            }
        }
        ast::cmd::Cmd::IfElse{cond, tr_cmd, fa_cmd} => {
            //check_bexpr_type(*cond);
            Result::Err("Error: not implemented yet". to_string())
        },
        ast::cmd::Cmd::WhileLoop{cond, lp_cmd} =>
            Result::Err("Error: not implemented yet". to_string()),
        ast::cmd::Cmd::Seq{fst_cmd, snd_cmd} =>
            Result::Err("Error: not implemented yet". to_string()),
        ast::cmd::Cmd::FnDecl{prototype, fn_cmd} =>
            Result::Err("Error: not implemented yet". to_string()),
        ast::cmd::Cmd::Return{e} =>
            Result::Err("Error: not implemented yet". to_string()),
    }
}

/* This function checks to make sure a given Bexp is well-typed.
 * The primary thing to check is that in a given expression, if
 * a variable is used we want to make sure that that variable
 * is of a type appropriate for the associated Bexp. For instance,
 * if we have "x != 5", we want to make sure x is an Int32 type. */
fn check_bexpr_type(bexp: ast::bexp::Bexp, var_types: &std::vec::Vec<VarTypePair>) -> Result<ast::data_type::DataType/*Type*/, String> {
    match bexp {
        ast::bexp::Bexp::BoolConst{v} => Result::Ok(ast::data_type::DataType::Bool),
        ast::bexp::Bexp::Beq{l, r} => {
            let l_type = check_bexpr_type(*l, var_types);
            let r_type = check_bexpr_type(*r, var_types);
            if l_type.is_ok() && r_type.is_ok() {
                if l_type != Result::Ok(ast::data_type::DataType::Bool) {
                    // l_type is incorrect type.
                    Result::Err("Error: expression () is not of type bool (but used in a '==')". to_string())
                } else if r_type != Result::Ok(ast::data_type::DataType::Bool)  {
                    // r_type is incorrect type.
                    Result::Err("Error: expression () is not of type bool (but used in a '==')". to_string())
                } else {
                    /* l_type and r_type are both of Bool type.
                     * Therefore '==' expr type is Bool. */
                    Result::Ok(ast::data_type::DataType::Bool)
                }
            } else {
                assert!(false);//FIXME: Make error message. If both things did not type well, error!
                Result::Err("bleh". to_string())
            }
        },
        /*ast::bexp::Bexp::Bneq{l, r} => false,
        ast::bexp::Bexp::And{l , r} => false,
        ast::bexp::Bexp::Or{l, r} => false,
        ast::bexp::Bexp::Aeq{l ,r} => false,
        ast::bexp::Bexp::Aneq{l ,r} => false,
        ast::bexp::Bexp::Lt{l ,r} => false,
        ast::bexp::Bexp::Lte{l ,r} => false,
        ast::bexp::Bexp::Gt{l ,r} => false,
        ast::bexp::Bexp::Gte{l ,r} => false,
        ast::bexp::Bexp::Var{v} => false,
        ast::bexp::Bexp::FnCall{fc} => false,*/
        _ => Result::Err("Error: bexp not currently implemented". to_string()),
    }
}

fn check_aexpr_type(aexp: ast::aexp::Aexp) -> bool {
    match aexp {
        ast::aexp::Aexp::IntConst{v} => true,
        ast::aexp::Aexp::FloConst{v} => true, //print!("float({})", v),
        ast::aexp::Aexp::Var{v} => true, //print!("var({})", v),
        ast::aexp::Aexp::Add{l, r} => {
            print!("Add (");
            check_aexpr_type(*l);
            print!(" + ");
            check_aexpr_type(*r);
            print!(")");
            true
        },
        ast::aexp::Aexp::Sub{l, r} => {
            print!("Sub: (");
            check_aexpr_type(*l);
            print!(" - ");
            check_aexpr_type(*r);
            print!(")");
            true
        },
        ast::aexp::Aexp::Mul{l, r} => {
            print!("Mul:");
            check_aexpr_type(*l);
            check_aexpr_type(*r);
            true
        },
        ast::aexp::Aexp::Div{l, r} => {
            print!("Div (");
            check_aexpr_type(*l);
            print!(" / ");
            check_aexpr_type(*r);
            print!(")");
            true
        },
        ast::aexp::Aexp::Mod{l, r} => {
            print!("Mod (");
            check_aexpr_type(*l);
            print!(" % ");
            check_aexpr_type(*r);
            print!(")");
            true
        },
        ast::aexp::Aexp::FnCall{fc} => {
            print!("{}", fc);
            true
        },
    }
}

/* This function helps us know if a variable has already been defined
 * and if it was, what its type was. */
pub fn get_var_type(var_types: &std::vec::Vec<VarTypePair>, var_name: String) -> (bool, ast::data_type::DataType) {
    let mut var_type = ast::data_type::DataType::Void;
    let mut found = false;
    println!("Vector length: {}", (*var_types).len());
    for pair in var_types {
        if pair.0 == var_name {
            found = true;
            var_type = pair.1;
            break
        }
    }
    (found, var_type)
}
