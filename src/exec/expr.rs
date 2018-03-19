use ast::{CtrlFlow, Expr, PostOp, Op2};
use ctxt::Ctxt;
use super::{ExecState, Val};

impl ExecState {
	pub fn exec_expr(&mut self, expr: &Expr, ctxt: &Ctxt) -> Option<Val> {
		match expr {
			&Expr::Null => Some(Val::Null),
			&Expr::Num(x) => Some(Val::Num(x)),
			&Expr::String(ref x) => Some(Val::String(x.clone())),
			&Expr::Bool(x) => Some(Val::Bool(x)),
			&Expr::Array(ref x) => Some(Val::Array(
				x.iter()
					.map(|e| self.exec_expr(e, ctxt).unwrap()).collect()
			)),
			&Expr::Tuple(ref x) => Some(Val::Tuple(
				x.iter()
					.map(|e| self.exec_expr(e, ctxt).unwrap()).collect()
			)),
			&Expr::Object(ref x) => Some(Val::Object(
				x.iter()
					.map(|(key, e)|
						(key.to_string(), Box::new(self.exec_expr(e, ctxt).unwrap()))
					).collect()
			)),
			&Expr::PostOp(ref fun, PostOp::FunCall(ref args)) => {
				if let &Expr::Var(ref s) = &**fun {
					if s == "print" {
						for arg in args {
							let val = self.exec_expr(&arg, ctxt).unwrap();
							println!("{:?}", val);
						}
						return None;
					}
				}

				if let Some(Val::Fun { signature, body }) = self.exec_expr(&**fun, ctxt) {
					assert_eq!(args.len(), signature.len());

					for (c_item, expr) in signature.iter().zip(args.iter()) {
						let val = self.exec_expr(expr, ctxt).unwrap();
						let i = self.heap.alloc(val);
						self.local_mut().insert(c_item.name.to_string(), Some(i));
						// TODO typechecking
					}

					let mut ret = None;

					self.push_stack();
					for node in body.nodes.iter() {
						if let Some(CtrlFlow::Return(x)) = self.exec_ast_node(node, ctxt) {
							ret = x;
							break;
						}
					}
					self.pop_stack();
					return ret;
				} else { panic!("calling non-fun value"); }
			},
			&Expr::Op2(ref a, ref op, ref b) => {
				let a = self.exec_expr(a, ctxt).unwrap();
				let b = self.exec_expr(b, ctxt).unwrap();

				let a = if let Val::Num(x) = a { x } else { panic!("operation on non-num value"); };
				let b = if let Val::Num(x) = b { x } else { panic!("operation on non-num value"); };

				Some(match op {
					&Op2::LessEq => Val::Bool(a <= b),
					&Op2::Eq => Val::Bool(a == b),
					&Op2::GreaterEq => Val::Bool(a >= b),
					&Op2::NotEq => Val::Bool(a != b),

					&Op2::Less => Val::Bool(a < b),
					&Op2::Greater => Val::Bool(a > b),

					&Op2::Plus => Val::Num(a + b),
					&Op2::Minus => Val::Num(a - b),
					&Op2::Mul => Val::Num(a * b),
					&Op2::Div => Val::Num(a / b),
					&Op2::Mod => Val::Num(a % b),
				})
			},
			&Expr::Fun { ref signature, ref body } => Some(Val::Fun { signature: signature.clone(), body: body.clone() }),
			&Expr::Var(ref name) => {
				self.get_var(name)
					.and_then(|i| self.heap.get(i))
					.map(|x| x.clone())
			},
			_ => unimplemented!(),
		}
	}
}
