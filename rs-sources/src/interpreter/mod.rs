pub struct Program
{
	pub func_states : std::rc::Rc<super::ast::states::FuncStatesStack<states::FuncState> >,
	pub var_states  : std::rc::Rc<super::ast::states::VarStatesStack<exp::ExpValue, states::VarState> >,
}

impl Program
{
	pub fn new() -> Program
	{
		Program
		{
			func_states : std::rc::Rc::new(super::ast::states::FuncStatesStack::new()),
			var_states  : std::rc::Rc::new(super::ast::states::VarStatesStack::new()),
		}
	}
}

pub mod aexp;
pub mod bexp;
pub mod exp;
pub mod states;
pub mod cmd;
