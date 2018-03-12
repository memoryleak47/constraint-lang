use ast::Expr;
use ctxt::Ctxt;
use super::{ExecState, Val};

impl ExecState {
	pub fn eval(&mut self, expr: &Expr, ctxt: &Ctxt) -> Option<Val> {
		match expr {
			_ => unimplemented!()
		}
	}
}
