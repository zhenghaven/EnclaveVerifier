use crate::ast;

use std::string::String;
use std::string::ToString;
use std::vec::Vec;

/* Members of VarTypePair:
 * 1) Variable name
 * 2) Type of variable
 * 3) Has variable been set yet */
#[derive(Clone)]
pub struct VarTypePair(String, ast::data_type::DataType, bool);


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
                Err(format!("Error: variable '{}' was declared more than once.", var))
            } else {
                var_types.push(VarTypePair(var.name, var.var_type, false));
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

            let (is_prev_decl, decl_type, _set) = get_var_type(&var_types, &(*var).name);

            if is_prev_decl == false {
                Err(format!("Error: an assign uses variable '{}' which has not yet been declared.", var))
            } else {
                let res : Result<ast::data_type::DataType, String>;
                match *e.clone() {
                    ast::exp::Exp::A{e:ae} => {
                        res = check_aexpr_type(&ae, &var_types, fn_types);
                    },
                    ast::exp::Exp::B{e:be} => {
                        res = check_bexpr_type(&be, &var_types, fn_types);
                    },
                }

                match res {
                    Ok(etype) => {
                        if etype == decl_type {
                            // Make sure to set var's pair to show it has been set.
                            for var_info in &mut var_types {
                                if var_info.0 == (*var).name {
                                    var_info.2 = true;
                                }
                            }
                            // Then return updated var_types.
                            Ok(var_types)
                        } else {
                            Err(format!("Error: variable '{}' being assigned to does not have same type as RHS type '{}'.", var, e))
                        }
                    },
                    Err(why) => Err(why)
                }
            }
        },

        // Fn-Call
        ast::cmd::Cmd::FnCall{fc} => {
            /* Note: we support calling functions and not necessarily
             * assigning them to variables (even if they do not have void
             * return type). Thus, here I just check to make sure a matching
             * function declaration exists */
            let mut fncall_arg_types : std::vec::Vec<ast::data_type::DataType> = vec![];
            let mut err = false;
            let mut err_string = "".to_string();
            for arg in &fc.exp_list {
                let arg_res = match arg {
                    ast::exp::Exp::A{e} => {
                        check_aexpr_type(&e, &var_types, fn_types)
                    },
                    ast::exp::Exp::B{e} => {
                        check_bexpr_type(&e, &var_types, fn_types)
                    },
                };
                let arg_type : ast::data_type::DataType;
                arg_type = match arg_res {
                    Ok(etype) => etype,
                    Err(why)  => {
                        err = true;
                        err_string = why;
                        break;
                    },
                };
                fncall_arg_types.push(arg_type);
            };

            if err {
                Err(err_string)
            } else {
                // Check to see if a matching FnDecl exists.
                let (found, _) = get_fn_return_type(fn_types, &fc.name, &fncall_arg_types);
                if found {
                    Ok(var_types)
                } else {
                    Err(format!("Error: function call {}, but no matching declaration", fc.name))
                }
            }
        }

        // If-Else
        ast::cmd::Cmd::IfElse{cond, tr_cmd, fa_cmd} => {
            /* Note: I need to make sure that the scope is preserved.
             * Any variables declared inside tr_cmd or fa_cmd should not
             * be seen outside. */
            /* FIXME: If possible, try to find a way to not have to use .clone().
             * It's costly, but I don't think there's a better way since I can't
             * use references (if tr_cmd alters the reference, fa_cmd shouldn't
             * see that change). For now this works, see if alternative in future. */
            match check_bexpr_type(&cond, &var_types, fn_types) {
                Ok(ast::data_type::DataType::Bool) => {
                    let mut t_has_err = false;
                    let mut f_has_err = false;
                    let mut t_err = "".to_string();
                    let mut f_err = "".to_string();
                    match iterate_through_ast(*tr_cmd, var_types.clone(), fn_types, curr_fn_type) {
                        Ok(_)      => (),
                        Err(why_t) => {
                            t_has_err = true;
                            t_err = why_t;
                        },
                    };
                    match iterate_through_ast(*fa_cmd, var_types.clone(), fn_types, curr_fn_type) {
                        Ok(_)      => (),
                        Err(why_f) => {
                            f_has_err = true;
                            f_err = why_f;
                        },
                    };

                    if t_has_err {
                        Err(t_err)
                    } else if f_has_err {
                        Err(f_err)
                    } else {
                        Ok(var_types)
                    }
                },
                Ok(_) => Err(format!("Error: use of expression '{}' as condition for if-else, but it's not a boolean.", cond)),
                Err(cond_why) => Err(cond_why),
            }
        },

        // While loop
        ast::cmd::Cmd::WhileLoop{cond, lp_cmd} => {
            match check_bexpr_type(&cond, &var_types, fn_types) {
                Ok(ast::data_type::DataType::Bool) => {
                    /* FIXME: This isn't perfect. This will tell if you the types are correct
                     * but just because this passes doesn't mean it is well-formed. For example
                     * "while(true) { skip }" will type-check but isn't well-formed since it will
                     * never terminate. */
                    match iterate_through_ast(*lp_cmd, var_types.clone(), fn_types, curr_fn_type) {
                        Ok(_)    => Ok(var_types),
                        Err(why) => Err(why),
                    }
                },
                Ok(_) => Err(format!("Error: use of expression '{}' as condition for while, but it's not a boolean.", cond)),
                Err(cond_why) => Err(cond_why),
            }
        },

        //Seq
        ast::cmd::Cmd::Seq{fst_cmd, snd_cmd} => {
            /* 1. Check to make sure type checker passes on fst_cmd.
             * 2. Check to make sure type checker passed on snd_cmd.
             * Note: We have to make sure we pass modified var_types
             * from first command result into handling second command. */
            match iterate_through_ast(*fst_cmd, var_types, fn_types, curr_fn_type) {
                Ok(var_types_1)   => {
                    let snd_res = iterate_through_ast(*snd_cmd, var_types_1, fn_types, curr_fn_type);
                    match snd_res {
                        Ok(vt2)  => Ok(vt2),
                        Err(why2)=> Err(why2),
                    }
                },
                Err(why1) => Err(why1),
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
            let mut var_types_clone = var_types.clone();
            for var_decl in &((*prototype).var_decl_list) {
                var_types_clone.push(VarTypePair(var_decl.name.clone(), var_decl.var_type, true));
            }

            let fn_cmd_cp : ast::cmd::Cmd = (*fn_cmd).clone();

            match iterate_through_ast(fn_cmd_cp, var_types_clone, fn_types, (*prototype).ret_type) {
                Ok(_vt)    => Ok(var_types),
                Err(why)   => Err(why),
            }
        },

        // Return
        ast::cmd::Cmd::Return{e} => {
            /* 1. Find type of expression (e) being returned.
             * 2. Make sure e's type matches the current
             *    function's return type. */
            match e {
                None => {
                    if curr_fn_type == ast::data_type::DataType::Void {
                        Ok(var_types)
                    } else {
                        Err(format!("Error: 'return' has void type, which is not function's return type."))
                    }
                },
                Some(expr) => {
                    let res = match *expr.clone() {
                        ast::exp::Exp::A{e} => {
                            check_aexpr_type(&e, &var_types, fn_types)
                        },
                        ast::exp::Exp::B{e} => {
                            check_bexpr_type(&e, &var_types, fn_types)
                        },
                    };
                    match res {
                        Ok(ret_type) => {
                            if ret_type == curr_fn_type {
                                // If return type matches func type, it type checks.
                                Ok(var_types)
                            } else if curr_fn_type == ast::data_type::DataType::Float32 && ret_type == ast::data_type::DataType::Int32 {
                                // If return type = Int32 and func_type = Float32, type check is OK.
                                Ok(var_types)
                            } else {
                                Err(format!("Error: 'return {}' does not have same type as function type.", expr))
                            }
                        },
                        Err(why) => Err(why),
                    }
                },
            }
        },
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

        // Unary bool comparison
        ast::bexp::Bexp::Not{e} => {
            match check_bexpr_type(e, var_types, fn_types) {
                Ok(ast::data_type::DataType::Bool) => {
                    Ok(ast::data_type::DataType::Bool)
                },
                Ok(_) => Err(format!("Error: expression '{}' is not of type bool, but used as such with not operator.", e)),
                Err(why) => Err(why),
            }
        },

        // N-ary bool comparison
        ast::bexp::Bexp::Beq{l, r} | ast::bexp::Bexp::Bneq{l, r} | ast::bexp::Bexp::And{l, r} | ast::bexp::Bexp::Or{l, r} => {
            let l_type = check_bexpr_type(l, var_types, fn_types);
            let r_type = check_bexpr_type(r, var_types, fn_types);

            if l_type.is_ok() && r_type.is_ok() {
                if l_type != Ok(ast::data_type::DataType::Bool) {
                    // l_type is incorrect type.
                    Err(format!("Error: expression '{}' is not of type bool, but used as such in bool comparison.", l))
                } else if r_type != Ok(ast::data_type::DataType::Bool)  {
                    // r_type is incorrect type.
                    Err(format!("Error: expression '{}' is not of type bool, but used as such in bool comparison.", r))
                } else {
                    // l_type and r_type are both of Bool type. Therefore entire expr type is Bool.
                    Ok(ast::data_type::DataType::Bool)
                }
            } else if !l_type.is_ok() {
                // If we reach here, l_type equals some error message.
                l_type
            } else {
                // If we reach here, r_type equals some error message.
                r_type
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
                    Err(format!("Error: expression '{}' is not of type Int32/Float32, but used as such in arith comparison.", l))
                } else if r_type != Ok(ast::data_type::DataType::Int32) && r_type != Ok(ast::data_type::DataType::Float32) {
                    // r_type is incorrect type.
                    Err(format!("Error: expression '{}' is not of type Int32/Float32, but used as such in arith comparison.", r))
                } else {
                    // l_type and r_type are both of Int32/Float32 type. Therefore entire expr type is Bool.
                    Ok(ast::data_type::DataType::Bool)
                }
            } else if !l_type.is_ok() {
                // If we reach here, l_type equals some error message.
                l_type
            } else {
                // If we reach here, r_type equals some error message.
                r_type
            }
        },

        // Variable
        ast::bexp::Bexp::Var{v} => {
            //Check to make sure variable is of type Float32 or Int32.
            let (is_prev_decl, decl_type, set) = get_var_type(var_types, &(*v).name);
            if !is_prev_decl {
                Err(format!("Error: use of variable {} before declared.", v))
            } else if !set {
                Err(format!("Error: use of variable {} before given value.", v))
            } else {
                Ok(decl_type)
            }
        },
        ast::bexp::Bexp::FnCall{fc} => {
            /* Make sure a function matching this function
             * call exists (matching name + arg types). If
             * so, return function type. */

            // Iterate over FnCall's arguments and create a vector holding their types.
            let mut arg_has_err = false;
            let mut arg_err = "".to_string();
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
                    Err(why)  => {
                        arg_has_err = true;
                        arg_err = why;
                        break;
                    },
                };
                fncall_arg_types.push(arg_type);
            };

            if arg_has_err {
                Err(arg_err)
            } else {
                /* Check to see if a function declaration exists with name
                 * fc.name and matching argument type list (compared to
                 * fncall_arg_types). If so, return the type returned
                 * by that function. */
                let (found, fn_ret_type) = get_fn_return_type(fn_types, &fc.name, &fncall_arg_types);
                if found {
                    Ok(fn_ret_type)
                } else {
                    Err(format!("Error: function call '{}', but no matching declaration.", fc))
                }
            }
        },
    }
}

/* This function checks to make sure a given Aexp is well-typed.
 * The primary thing to check is that in a given expression, if
 * a variable is used we want to make sure that that variable
 * is of a type appropriate for the associated Aexp. For instance,
 * if we have "x + 5", we want to make sure x is an Int32 or Float32 type. */
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
                    Err(format!("Error: expression '{}' is not an Int32/Float32 type, but used as such in arith operations.", l))
                } else if r_type != Ok(ast::data_type::DataType::Int32) && r_type != Ok(ast::data_type::DataType::Float32) {
                    // r_type is incorrect type.
                    Err(format!("Error: expression '{}' is not an Int32/Float32 type, but used as such in arith operations.", r))
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
            } else if !l_type.is_ok() {
                // If we reach here, l_type equals some error message.
                l_type
            } else {
                // If we reach here, r_type equals some error message.
                r_type
            }
        },

        // Variable
        ast::aexp::Aexp::Var{v} => {
            //Check to make sure variable is of type Float32 or Int32.
            let (is_prev_decl, decl_type, set) = get_var_type(var_types, &(*v).name);
            if !is_prev_decl {
                Err(format!("Error: use of variable '{}' before declared.", v))
            } else if !set {
                Err(format!("Error: use of variable '{}' before given value.", v))
            } else {
                Ok(decl_type)
            }
        },

        // Function call
        ast::aexp::Aexp::FnCall{fc} => {
            /* Make sure a function matching this function
             * call exists (matching name + arg types). If
             * so, return function type. */

            // Iterate over FnCall's arguments and create a vector holding their types.
            let mut arg_has_err = false;
            let mut arg_err = "".to_string();
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
                    Err(why)  => {
                        arg_has_err = true;
                        arg_err = why;
                        break;
                    },
                };
                fncall_arg_types.push(arg_type);
            };

            if arg_has_err {
                Err(arg_err)
            } else {
                /* Check to see if a function declaration exists with name
                 * fc.name and matching argument type list (compared to
                 * fncall_arg_types). If so, return the type returned
                 * by that function. */
                let (found, fn_ret_type) = get_fn_return_type(fn_types, &fc.name, &fncall_arg_types);
                if found {
                    Ok(fn_ret_type)
                } else {
                    Err(format!("Error: function call '{}', but no matching declaration.", fc))
                }
            }
        },
    }
}

/* This function helps us know if a variable has already been defined
 * and if it was, what its type was. */
fn get_var_type(var_types: &std::vec::Vec<VarTypePair>, var_name: &String) -> (bool, ast::data_type::DataType, bool) {
    let mut var_type = ast::data_type::DataType::Void;
    let mut found = false;
    let mut set = false;
    for pair in var_types {
        if pair.0 == *var_name {
            found = true;
            set = pair.2;
            var_type = pair.1;
            break
        }
    }
    (found, var_type, set)
}

/* This function reads over an AST and ignores everything except for
 * function declarations. Once it sees one, it adds that function's
 * name, argument types, and return type to a vector which holds
 * all the declarations. This vector is the output of this function. */
pub fn gather_fn_types(cmd: &ast::cmd::Cmd, fn_types: &mut std::vec::Vec<FuncIdentifierTuple>) -> () {
    match cmd {
        // Functional Declarations
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

        // Seq
        ast::cmd::Cmd::Seq{fst_cmd, snd_cmd} => {
            // Check sub-sequences to see if any function declarations.
            gather_fn_types(fst_cmd, fn_types);
            gather_fn_types(snd_cmd, fn_types);
            ()
        },

        // If not function dec or seq, do nothing (since no possible function declaration).
        _ => ()
    }
}

/* This is used to tell us what the return type of a function is
 * if we are handling a function call. By passing the function's name
 * and the types of that call's arguments, we can compare that to
 * all the function declarations that exists and confirm an associated
 * declaration exists by looking in our fn_types vector (which holds all
 * the declarations). */
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
