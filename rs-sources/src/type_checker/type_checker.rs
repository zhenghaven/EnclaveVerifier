use crate::ast;

use std::string::String;
use std::vec::Vec;

/* Members of VarTypePair:
 * 1) Variable name
 * 2) Type of variable */
#[derive(Clone)]
pub struct VarTypePair(String, ast::data_type::DataType);


/* Members of FuncIdentifierTuple:
 * 1) Name of function
 * 2) Return type
 * 3) Types of each input argument */
#[derive(Clone)]
pub struct FuncIdentifierTuple(pub String, pub ast::data_type::DataType, pub Vec<ast::data_type::DataType>);

/* This iterates over the commands that make up the program.
 * Depending on what kind of command we're looking at, we
 * will type-check each one of the sub-expressions of the
 * current command based off whether they're AExps or BExps. */
pub fn iterate_through_ast(cmd: ast::cmd::Cmd, mut var_types: std::vec::Vec<VarTypePair>,
                           fn_types: &std::vec::Vec<FuncIdentifierTuple>, curr_fn_type: ast::data_type::DataType)
                           -> Result<std::vec::Vec<VarTypePair>, String> {
    //println!("cmd: {}", cmd);
    match cmd {
        // Skip
        ast::cmd::Cmd::Skip => Ok(var_types),

        // Variable Type Declaration
        ast::cmd::Cmd::VarDecl{d} => {
            //1. Check to see if var's type was already declared.
            //2. If so, error.
            //3. If not, then add to var_types vector and pass to next command.
            let var = *d;
            if (*var_types).iter().any(|i| i.0 == var.name) {
                panic!("Error: variable {} was declared more than once.", var.name)
            } else {
                var_types.push(VarTypePair(var.name, var.var_type));
                Ok(var_types)
            }
        },

        // Variable assignment
        ast::cmd::Cmd::Assign{var, e} => {
            /* 1. Figure out type of LHS, make sure it was previously declared.
             * 2. Figure out type of RHS + make sure it is well-typed.
             * 3. Check to make sure LHS var is:
             *   a. Already declared.
             *   b. Type(LHS) = Type(RHS). */

            let (is_prev_decl, decl_type) = get_var_type(&var_types, &(*var).name);

            if is_prev_decl == false {
                Err(format!("{}", "Error: an assign uses variable () which has not yet been declared."))
            } else {
                match *e {
                    ast::exp::Exp::A{e} => {
                        let res = check_aexpr_type(&e, &var_types, fn_types);
                        if res.is_ok() {
                            if res == Ok(decl_type) {
                                Ok(var_types)
                            } else {
                                Err(format!("{}", "Error: variable being assigned to does not have same type as RHS type."))
                            }
                        } else {
                            Err(format!("{}", "Error: assign failed on not well-typed bexp."))
                            //panic!("Error: bexpr {} checked is not well typed.", e);
                        }
                    },
                    ast::exp::Exp::B{e} => {
                        let res = check_bexpr_type(&e, &var_types, fn_types);
                        if res.is_ok() {
                            if res == Ok(decl_type) {
                                Ok(var_types)
                            } else {
                                Err(format!("{}", "Error: variable being assigned to does not have same type as RHS type."))
                            }
                        } else {
                            Err(format!("{}", "Error: assign failed on not well-typed bexp."))
                            //panic!("Error: bexpr {} checked is not well typed.", e);
                        }
                    },
                }
            }
        },

        // If-Else
        ast::cmd::Cmd::IfElse{cond, tr_cmd, fa_cmd} => {
            /* Note: I need to make sure that the scope is preserved.
             * Any variables declared inside tr_cmd or fa_cmd should not
             * be seen outside. */
            /* FIXME: If possible, try to find a way to not have to use .clone().
             * It's costly, but I don't think there's a better way since I can't
             * use references (if tr_cmd alters the reference, fa_cmd shouldn't
             * see that change). For now this works, see if alternative in future. */
            let cond_res = match check_bexpr_type(&cond, &var_types, fn_types) {
                Ok(cond_type) => cond_type,
                Err(cond_why) => panic!("{}", cond_why),
            };
            if cond_res != ast::data_type::DataType::Bool {
                Err(format!("{}", "Error: use of expression () as condition for if-else, but it's not a boolean."))
            } else {
                match iterate_through_ast(*tr_cmd, var_types.clone(), fn_types, curr_fn_type) {
                    Ok(_)      => (),
                    Err(why_t) => panic!("{}", why_t),
                };
                match iterate_through_ast(*fa_cmd, var_types.clone(), fn_types, curr_fn_type) {
                    Ok(_)      => (),
                    Err(why_f) => panic!("{}", why_f),
                };
                Ok(var_types)
            }
        },

        // While loop
        /*ast::cmd::Cmd::WhileLoop{cond, lp_cmd} => Ok(var_types),
            Result::Err("Error: not implemented yet". to_string()),*/

        //Seq
        ast::cmd::Cmd::Seq{fst_cmd, snd_cmd} => {
            /* Note: We have to make sure we pass modified var_types from
             * first command result into handling second command. */
            let fst_res = iterate_through_ast(*fst_cmd, var_types, fn_types, curr_fn_type);
            let var_types_1 = match fst_res {
                Ok(vt1)   => vt1,
                Err(why1) => panic!("{}", why1)
            };
            let snd_res = iterate_through_ast(*snd_cmd, var_types_1, fn_types, curr_fn_type);
            match snd_res {
                Ok(vt2)  => Ok(vt2),
                Err(why2)=> panic!("{}", why2)
            }
        },

        // Function declaration (note: we already populated fn_types with this)
        ast::cmd::Cmd::FnDecl{prototype, fn_cmd} => {
            /* 1. Iterate over prototype's var_decl_list. Add each
             *    arg's type to var_types (for just the fn's
             *    command's scope). Presumably, var_types should
             *    only have global var's prior to this.
             * 2. With var_types populated with fn's arg types,
             *    make sure the fn's commands are well-typed.
             * 3. When all is done, we need to make sure any
             *    return's type matches fn's type. (can probably be done in return cmd instead) */
            for var_decl in &((*prototype).var_decl_list) {
                var_types.push(VarTypePair(var_decl.name.clone(), var_decl.var_type));
            }

            match iterate_through_ast(*fn_cmd, var_types.clone(), fn_types, (*prototype).ret_type) {
                Ok(_vt)    => Ok(var_types),
                Err(why) => panic!("{}", why)
            }
        },

        // Return
        ast::cmd::Cmd::Return{e} => {
            match *e {
                ast::exp::Exp::A{e} => {
                    let res = check_aexpr_type(&e, &var_types, fn_types);
                    if res.is_ok() {
                        if res == Ok(curr_fn_type) {
                            Ok(var_types)
                        } else {
                            Err(format!("{}", "Error: 'return ()' does not have same type as function type."))
                        }
                    } else {
                        Err(format!("{}", "Error: 'return ()' is not well-formed."))
                    }
                },
                ast::exp::Exp::B{e} => {
                    let res = check_bexpr_type(&e, &var_types, fn_types);
                    if res.is_ok() {
                        if res == Ok(curr_fn_type) {
                            Ok(var_types)
                        } else {
                            Err(format!("{}", "Error: 'return ()' does not have same type as function type."))
                        }
                    } else {
                        Err(format!("{}", "Error: 'return ()' is not well-formed."))
                    }
                },
            }
        },
        _ => Err(format!("{}", "Error: cmd not yet implemented"))
    }
}

/* This function checks to make sure a given Bexp is well-typed.
 * The primary thing to check is that in a given expression, if
 * a variable is used we want to make sure that that variable
 * is of a type appropriate for the associated Bexp. For instance,
 * if we have "x != 5", we want to make sure x is an Int32 type. */
fn check_bexpr_type(bexp: &ast::bexp::Bexp, var_types: &std::vec::Vec<VarTypePair>,
                    fn_types: &std::vec::Vec<FuncIdentifierTuple>) -> Result<ast::data_type::DataType, String> {
    match bexp {
        // Bool const (true/false)
        ast::bexp::Bexp::BoolConst{v : _} => Ok(ast::data_type::DataType::Bool),

        // Bool comparison
        ast::bexp::Bexp::Beq{l, r} | ast::bexp::Bexp::Bneq{l, r} | ast::bexp::Bexp::And{l, r} | ast::bexp::Bexp::Or{l, r} => {
            let l_type = check_bexpr_type(l, var_types, fn_types);
            let r_type = check_bexpr_type(r, var_types, fn_types);
            if l_type.is_ok() && r_type.is_ok() {
                if l_type != Ok(ast::data_type::DataType::Bool) {
                    // l_type is incorrect type.
                    panic!("Error: expression {} is not of type bool, but used as such in bool comparison.", l);
                } else if r_type != Ok(ast::data_type::DataType::Bool)  {
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
                Err(format!("{}", "Can't reach here."))
            }
        },

        // Arith comparison
        ast::bexp::Bexp::Aeq{l, r} | ast::bexp::Bexp::Aneq{l, r} | ast::bexp::Bexp::Lt{l ,r} |
        ast::bexp::Bexp::Lte{l, r} | ast::bexp::Bexp::Gt{l, r}   | ast::bexp::Bexp::Gte{l, r}  => {
            let l_type = check_aexpr_type(l, var_types, fn_types);
            let r_type = check_aexpr_type(r, var_types, fn_types);
            if l_type.is_ok() && r_type.is_ok() {
                if l_type != Ok(ast::data_type::DataType::Int32) && l_type != Ok(ast::data_type::DataType::Float32) {
                    // l_type is incorrect type.
                    panic!("Error: expression {} is not of type Int32/Float32, but used as such in arith comparison.", l);
                } else if r_type != Ok(ast::data_type::DataType::Int32) && r_type != Ok(ast::data_type::DataType::Float32) {
                    // r_type is incorrect type.
                    panic!("Error: expression {} is not of type Int32/Float32, but used as such in arith comparison.", r);
                } else {
                    // l_type and r_type are both of Int32/Float32 type. Therefore entire expr type is Bool.
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
                Err(format!("{}", "Can't reach here."))
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
        ast::bexp::Bexp::FnCall{fc} => {
            /* Make sure a function matching this function
             * call exists (matching name + arg types). If
             * so, return function type. */

            // Iterate over FnCall's arguments and create a vector holding their types.
            let mut fncall_arg_types : std::vec::Vec<ast::data_type::DataType> = vec![];
            for arg in &fc.exp_list {
                let arg_type : ast::data_type::DataType;
                let arg_res = match arg {
                    ast::exp::Exp::A{e} => {
                        check_aexpr_type(&e, &var_types, fn_types)
                    },
                    ast::exp::Exp::B{e} => {
                        check_bexpr_type(&e, &var_types, fn_types)
                    },
                };
                arg_type = match arg_res {
                    Ok(etype) => etype,
                    Err(why)  => panic!("{}", why),
                };
                fncall_arg_types.push(arg_type);
            };

            /* Check to see if a function declaration exists with name
             * fc.name and matching argument type list (compared to
             * fncall_arg_types). If so, return the type returned
             * by that function. */
            let (found, fn_ret_type) = get_fn_return_type(fn_types, &fc.name, &fncall_arg_types);
            if found {
                Ok(fn_ret_type)
            } else {
                Err(format!("{}", "Error: function call (), but no matching declaration."))
            }
        },
    }
}

fn check_aexpr_type(aexp: &ast::aexp::Aexp, var_types: &std::vec::Vec<VarTypePair>,
                    fn_types: &std::vec::Vec<FuncIdentifierTuple>) -> Result<ast::data_type::DataType, String> {
    match aexp {
        // Int const
        ast::aexp::Aexp::IntConst{v : _} => Ok(ast::data_type::DataType::Int32),

        // Float const
        ast::aexp::Aexp::FloConst{v : _} => Ok(ast::data_type::DataType::Float32),

        // Arith operations
        ast::aexp::Aexp::Add{l, r} | ast::aexp::Aexp::Sub{l, r} | ast::aexp::Aexp::Mul{l, r} |
        ast::aexp::Aexp::Div{l, r} | ast::aexp::Aexp::Mod{l, r} => {
            let l_type = check_aexpr_type(l, var_types, fn_types);
            let r_type = check_aexpr_type(r, var_types, fn_types);
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
                Err(format!("{}", "Can't reach here."))
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

        // Function call
        ast::aexp::Aexp::FnCall{fc} => {
            /* Make sure a function matching this function
             * call exists (matching name + arg types). If
             * so, return function type. */

            // Iterate over FnCall's arguments and create a vector holding their types.
            let mut fncall_arg_types : std::vec::Vec<ast::data_type::DataType> = vec![];
            for arg in &fc.exp_list {
                let arg_type : ast::data_type::DataType;
                let arg_res = match arg {
                    ast::exp::Exp::A{e} => {
                        check_aexpr_type(&e, &var_types, fn_types)
                    },
                    ast::exp::Exp::B{e} => {
                        check_bexpr_type(&e, &var_types, fn_types)
                    },
                };
                arg_type = match arg_res {
                    Ok(etype) => etype,
                    Err(why)  => panic!("{}", why),
                };
                fncall_arg_types.push(arg_type);
            };

            /* Check to see if a function declaration exists with name
             * fc.name and matching argument type list (compared to
             * fncall_arg_types). If so, return the type returned
             * by that function. */
            let (found, fn_ret_type) = get_fn_return_type(fn_types, &fc.name, &fncall_arg_types);
            if found {
                Ok(fn_ret_type)
            } else {
                Err(format!("{}", "Error: function call (), but no matching declaration."))
            }
        },
    }
}

/* This function helps us know if a variable has already been defined
 * and if it was, what its type was. */
fn get_var_type(var_types: &std::vec::Vec<VarTypePair>, var_name: &String) -> (bool, ast::data_type::DataType) {
    let mut var_type = ast::data_type::DataType::Void;
    let mut found = false;
    for pair in var_types {
        if pair.0 == *var_name {
            found = true;
            var_type = pair.1;
            break
        }
    }
    (found, var_type)
}

pub fn gather_fn_types(cmd: &ast::cmd::Cmd, fn_types: &mut std::vec::Vec<FuncIdentifierTuple>) -> () {
    match cmd {
        ast::cmd::Cmd::FnDecl{prototype, fn_cmd : _} => {
            /* If a function declaration is found, we'll need to record
             * the function's name, its return type, and the type of
             * all of the function's arguments. */
            let mut arg_type_list : Vec<ast::data_type::DataType> = vec![];
            for var_decl in &((*prototype).var_decl_list) {
                // Add the type of each argument to arg_type_list.
                arg_type_list.push(var_decl.var_type);
            }
            (*fn_types).push(FuncIdentifierTuple((*prototype).name.clone(), (*prototype).ret_type, arg_type_list))
        },
        ast::cmd::Cmd::Seq{fst_cmd, snd_cmd} => {
            // Check sub-sequences to see if any function declarations.
            gather_fn_types(&(*fst_cmd), fn_types);
            gather_fn_types(&(*snd_cmd), fn_types);
            ()
        },
        // In every other case, do nothing (since no possible function declaration).
        _ => ()
    }
}

fn get_fn_return_type(fn_types: &std::vec::Vec<FuncIdentifierTuple>, fn_name: &String,
                      arg_types : &Vec<ast::data_type::DataType>) -> (bool, ast::data_type::DataType) {
    let mut found = false;
    let mut f_type : ast::data_type::DataType = ast::data_type::DataType::Void;

    /* Iterate through fn_types to see if a function
     * declaration even exists under the name fn_name. */
    for func in fn_types {
        // If we find a function declaration that matches this function call.
        if func.0 == *fn_name && func.2.len() == arg_types.len() {
            let matching = arg_types.iter().zip(&func.2).filter(|&(t1, t2)| t1 == t2).count();
            if matching == arg_types.len() {
                f_type = func.1;
                found = true;
                break;
            }
        }
    }
    (found, f_type)
}
