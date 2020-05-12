use crate::ast;

pub fn iterate_through_ast(aexp: ast::aexp::Aexp) -> ()
{
    match aexp {
        ast::aexp::Aexp::IntConst{v} => print!("int({})", v),
        ast::aexp::Aexp::FloConst{v} => print!("float({})", v),
        ast::aexp::Aexp::Var{v} => print!("var({})", v),
        ast::aexp::Aexp::Add{l, r} => {
            print!("Add (");
            iterate_through_ast(*l);
            print!(" + ");
            iterate_through_ast(*r);
            print!(")");
        },
        ast::aexp::Aexp::Sub{l, r} => {
            print!("Sub: (");
            iterate_through_ast(*l);
            print!(" - ");
            iterate_through_ast(*r);
            print!(")");
        },
        ast::aexp::Aexp::Mul{l, r} => {
            print!("Mul:");
            iterate_through_ast(*l);
            iterate_through_ast(*r);
        },
        ast::aexp::Aexp::Div{l, r} => {
            print!("Div (");
            iterate_through_ast(*l);
            print!(" / ");
            iterate_through_ast(*r);
            print!(")");
        },
        ast::aexp::Aexp::Mod{l, r} => {
            print!("Mod (");
            iterate_through_ast(*l);
            print!(" % ");
            iterate_through_ast(*r);
            print!(")");
        },
        ast::aexp::Aexp::FnCall{fc} => {
            print!("{}", fc);
        },
    }
}
