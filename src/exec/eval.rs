use ast::{Expr, PostOp, Op2};
use ctxt::Ctxt;
use super::{ExecState, Val};

impl ExecState {
	pub fn eval(&mut self, expr: &Expr, ctxt: &Ctxt) -> Option<Val> {
		match expr {
			&Expr::Null => Some(Val::Null),
			&Expr::Num(x) => Some(Val::Num(x)),
			&Expr::String(ref x) => Some(Val::String(x.clone())),
			&Expr::Bool(x) => Some(Val::Bool(x)),
			&Expr::Array(ref x) => Some(Val::Array(
				x.iter()
					.map(|e| self.eval(e, ctxt).unwrap()).collect()
			)),
			&Expr::Tuple(ref x) => Some(Val::Tuple(
				x.iter()
					.map(|e| self.eval(e, ctxt).unwrap()).collect()
			)),
			&Expr::Object(ref x) => Some(Val::Object(
				x.iter()
					.map(|(key, e)|
						(key.to_string(), Box::new(self.eval(e, ctxt).unwrap()))
					).collect()
			)),
			&Expr::PostOp(ref fun, PostOp::FunCall(ref args)) => {
				if let &Expr::Var(ref s) = &**fun {
					if s == "print" {
						for arg in args {
							let val = self.eval(&arg, ctxt).unwrap();
							println!("{:?}", val);
						}
						return None;
					}
				}
				unimplemented!()
			},
			&Expr::Op2(ref a, ref op, ref b) => {
				let a = self.eval(a, ctxt).unwrap();
				let b = self.eval(b, ctxt).unwrap();

				let a = if let Val::Num(x) = a { x } else { panic!("addition on non-num value"); };
				let b = if let Val::Num(x) = b { x } else { panic!("addition on non-num value"); };

				Some(Val::Num(match op {
					&Op2::Plus => a + b,
					&Op2::Minus => a - b,
					&Op2::Mul => a * b,
					&Op2::Div => a / b,
					&Op2::Mod => a % b,
				}))
			},
			// &Expr::Fun { .. } => Some(Val::Fun(x)),
			&Expr::Var(ref name) => {
				self.get_var(name)
					.and_then(|i| self.heap.get(i))
					.map(|x| x.clone())
			},
			_ => unimplemented!(),
		}
	}
}
