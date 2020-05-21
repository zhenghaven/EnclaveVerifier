use crate::ast;

pub struct VarTypePair(String, ast::data_type::DataType);

/* This iterates over the commands that make up the program.
 * Depending on what kind of command we're looking at, we
 * will type-check each one of the sub-expressions of the
 * current command based off whether they're AExps or BExps. */
pub fn iterate_through_ast(cmd: ast::cmd::Cmd, var_types: &mut std::vec::Vec<VarTypePair>) -> Result<&mut std::vec::Vec<VarTypePair>, String> {
    println!("cmd: {}", cmd);
    match cmd {
        ast::cmd::Cmd::Skip => Result::Ok(var_types),
        ast::cmd::Cmd::VarDecl{d} => {
            //1. Check to see if var's type was already declared.
            //2. If so, error.
            //3. If not, then add to var_types vector and pass to next command.
            let var = *d;
            if (*var_types).iter().any(|i| i.0 == var.name) {
                panic!("Error: variable {} was declared more than once.", var.name)
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

            let (is_prev_decl, decl_type) = get_var_type(var_types, &(*var).name);

            if is_prev_decl == false {
                Err("Error: an assign uses variable () which has not yet been declared.". to_string())
            } else {
                match *e {
                    ast::exp::Exp::A{e} => {
                        let res = check_aexpr_type(&e, var_types);
                        if res.is_ok() {
                            if res == Result::Ok(decl_type) {
                                Ok(var_types)
                            } else {
                                Err("Error: assign's LHS type does not match RHS type.". to_string())
                            }
                        } else {
                            Err("Error: assign failed on not well-typed bexp.". to_string())
                            //panic!("Error: bexpr {} checked is not well typed.", e);
                        }
                    },
                    ast::exp::Exp::B{e} => {
                        let res = check_bexpr_type(&e, var_types);
                        if res.is_ok() {
                            if res == Result::Ok(decl_type) {
                                Ok(var_types)
                            } else {
                                Err("Error: assign's LHS type does not match RHS type.". to_string())
                            }
                        } else {
                            Err("Error: assign failed on not well-typed bexp.". to_string())
                            //panic!("Error: bexpr {} checked is not well typed.", e);
                        }
                    },
                }
            }
        },
        /*ast::cmd::Cmd::IfElse{cond, tr_cmd, fa_cmd} => {
            //check_bexpr_type(*cond);
            Result::Err("Error: not implemented yet". to_string())
        },
        ast::cmd::Cmd::WhileLoop{cond, lp_cmd} =>
            Result::Err("Error: not implemented yet". to_string()),*/

        //Seq
        ast::cmd::Cmd::Seq{fst_cmd, snd_cmd} => {
            let fst_res = iterate_through_ast(*fst_cmd, var_types);
            let var_types_1 = match fst_res {
                Ok(vt1)   => vt1,
                Err(why1) => panic!("Error: {}", why1)
            };
            let snd_res = iterate_through_ast(*snd_cmd, var_types_1);
            match snd_res {
                Ok(vt2)  => Result::Ok(vt2),
                Err(why2)=> panic!("Error: {}", why2)
            }
        },

        /*ast::cmd::Cmd::FnDecl{prototype, fn_cmd} =>
            Result::Err("Error: not implemented yet". to_string()),
        ast::cmd::Cmd::Return{e} =>
            Result::Err("Error: not implemented yet". to_string()),*/
        _ => Err("Error: cmd not yet implemented". to_string())
    }
}

/* This function checks to make sure a given Bexp is well-typed.
 * The primary thing to check is that in a given expression, if
 * a variable is used we want to make sure that that variable
 * is of a type appropriate for the associated Bexp. For instance,
 * if we have "x != 5", we want to make sure x is an Int32 type. */
fn check_bexpr_type(bexp: &ast::bexp::Bexp, var_types: &std::vec::Vec<VarTypePair>) -> Result<ast::data_type::DataType, String> {
    match bexp {
        // Bool const (true/false)
        ast::bexp::Bexp::BoolConst{v : _} => Ok(ast::data_type::DataType::Bool),

        // Bool comparison
        ast::bexp::Bexp::Beq{l, r} | ast::bexp::Bexp::Bneq{l, r} | ast::bexp::Bexp::And{l, r} | ast::bexp::Bexp::Or{l, r} => {
            let l_type = check_bexpr_type(l, var_types);
            let r_type = check_bexpr_type(r, var_types);
            if l_type.is_ok() && r_type.is_ok() {
                if l_type != Ok(ast::data_type::DataType::Bool) {
                    // l_type is incorrect type.
                    panic!("Error: expression {} is not of type bool, but used as such in bool comparison.", l);
                } else if r_type != Result::Ok(ast::data_type::DataType::Bool)  {
                    // r_type is incorrect type.
                    panic!("Error: expression {} is not of type bool, but used as such in bool comparison.", r);
                } else {
                    // l_type and r_type are both of Bool type. Therefore entire expr type is Bool.
                    Ok(ast::data_type::DataType::Bool)
                }
            } else {
                //If one of the checks created an error, print out why.
                match l_type {
                    Err(whyl) => panic!("{}", whyl),
                    _         => (),
                };
                match r_type {
                    Err(why2) => panic!("{}", why2),
                    _         => (),
                };
                assert!(false);//If we are in this else, we will never reach here.
                Err("Can't reach here.". to_string())
            }
        },

        // Arith comparison
        ast::bexp::Bexp::Aeq{l, r} | ast::bexp::Bexp::Aneq{l, r} | ast::bexp::Bexp::Lt{l ,r} |
        ast::bexp::Bexp::Lte{l, r} | ast::bexp::Bexp::Gt{l, r}   | ast::bexp::Bexp::Gte{l, r}  => {
            let l_type = check_aexpr_type(l, var_types);
            let r_type = check_aexpr_type(r, var_types);
            if l_type.is_ok() && r_type.is_ok() {
                if l_type != Ok(ast::data_type::DataType::Int32) && l_type != Ok(ast::data_type::DataType::Float32) {
                    // l_type is incorrect type.
                    panic!("Error: expression {} is not of type Int32/Float32, but used as such in arith comparison.", l);
                } else if r_type != Ok(ast::data_type::DataType::Int32) && r_type != Ok(ast::data_type::DataType::Float32) {
                    // r_type is incorrect type.
                    panic!("Error: expression {} is not of type Int32/Float32, but used as such in arith comparison.", r);
                } else {
                    /* l_type and r_type are both of Int32 or Float32 type.
                     * If both are Int32, expr type is Int32.
                     * Otherwise expr type is Float32. */
                    if l_type == Ok(ast::data_type::DataType::Int32) && r_type == Ok(ast::data_type::DataType::Int32) {
                        Ok(ast::data_type::DataType::Int32)
                    } else {
                        Ok(ast::data_type::DataType::Float32)
                    }
                }
            } else {
                //If one of the checks created an error, print out why.
                match l_type {
                    Err(whyl) => panic!("{}", whyl),
                    _         => (),
                };
                match r_type {
                    Err(why2) => panic!("{}", why2),
                    _         => (),
                };
                assert!(false);//If we are in this else, we will never reach here.
                Err("Can't reach here.". to_string())
            }
        },

        // Variable
        ast::bexp::Bexp::Var{v} => {
            //Check to make sure variable is of type Float32 or Int32.
            let (is_prev_decl, decl_type) = get_var_type(var_types, &(*v).name);
            if is_prev_decl {
                Ok(decl_type)
            } else {
                panic!("Error: use of variable {} before declared.", (*v).name)
            }
        },
        /*ast::bexp::Bexp::FnCall{fc} => false,*/
        _ => Err("Error: bexp not currently implemented". to_string()),
    }
}

fn check_aexpr_type(aexp: &ast::aexp::Aexp, var_types: &std::vec::Vec<VarTypePair>) -> Result<ast::data_type::DataType, String> {
    match aexp {
        // Int const
        ast::aexp::Aexp::IntConst{v : _} => Ok(ast::data_type::DataType::Int32),

        // Float const
        ast::aexp::Aexp::FloConst{v : _} => Ok(ast::data_type::DataType::Float32),

        // Arith operations
        ast::aexp::Aexp::Add{l, r} | ast::aexp::Aexp::Sub{l, r} | ast::aexp::Aexp::Mul{l, r} |
        ast::aexp::Aexp::Div{l, r} | ast::aexp::Aexp::Mod{l, r} => {
            let l_type = check_aexpr_type(l, var_types);
            let r_type = check_aexpr_type(r, var_types);
            if l_type.is_ok() && r_type.is_ok() {
                if l_type != Ok(ast::data_type::DataType::Int32) && l_type != Ok(ast::data_type::DataType::Float32) {
                    // l_type is incorrect type.
                    panic!("Error: expression {} is not an int/flo type, but used as such in arith operations.", l);
                } else if r_type != Ok(ast::data_type::DataType::Int32) && r_type != Ok(ast::data_type::DataType::Float32) {
                    // r_type is incorrect type.
                    panic!("Error: expression {} is not an int/flo type, but used as such in arith operations.", r);
                } else {
                    /* l_type and r_type are either Int32 or Float32.
                     * If both are Int32, then expr type is Int32.
                     * Otherwise, expr type is Float32. */
                    if l_type == Ok(ast::data_type::DataType::Int32) && r_type == Ok(ast::data_type::DataType::Int32) {
                        Ok(ast::data_type::DataType::Int32)
                    } else {
                        Ok(ast::data_type::DataType::Float32)
                    }
                }
            } else {
                //If one of the checks created an error, print out why.
                match l_type {
                    Err(whyl) => panic!("{}", whyl),
                    _         => (),
                };
                match r_type {
                    Err(why2) => panic!("{}", why2),
                    _         => (),
                };
                assert!(false);//If we are in this else, we will never reach here.
                Err("Can't reach here.". to_string())
            }
        },

        // Variable
        ast::aexp::Aexp::Var{v} => {
            //Check to make sure variable is of type Float32 or Int32.
            let (is_prev_decl, decl_type) = get_var_type(var_types, &(*v).name);
            if is_prev_decl {
                Ok(decl_type)
            } else {
                panic!("Error: use of variable {} before declared.", (*v).name)
            }
        },
        /*ast::aexp::Aexp::FnCall{fc} => {
            print!("{}", fc);
            true
        },*/
        _ => Err("Error: aexp not currently implemented". to_string()),
    }
}

/* This function helps us know if a variable has already been defined
 * and if it was, what its type was. */
pub fn get_var_type(var_types: &std::vec::Vec<VarTypePair>, var_name: &String) -> (bool, ast::data_type::DataType) {
    let mut var_type = ast::data_type::DataType::Void;
    let mut found = false;
    println!("Vector length: {}", (*var_types).len());
    for pair in var_types {
        if pair.0 == *var_name {
            found = true;
            var_type = pair.1;
            break
        }
    }
    (found, var_type)
}
