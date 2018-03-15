use ast::{Expr, PostOp, Op2};
use ctxt::Ctxt;
use super::{ExecState, Val};

use std::collections::HashMap;

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

				if let Some(Val::Fun { signature, body }) = self.eval(&**fun, ctxt) {
					self.stack.push(HashMap::new());

					assert_eq!(args.len(), signature.len());

					for (c_item, expr) in signature.iter().zip(args.iter()) {
						let val = self.eval(expr, ctxt).unwrap();
						let i = self.heap.alloc(val);
						self.local_mut().insert(c_item.name.to_string(), Some(i));
						// TODO typechecking
					}

					for node in body.nodes.iter() {
						self.exec_node(node, ctxt);
					}

					self.stack.pop();
				} else { panic!("calling non-fun value"); }

				return None;
			},
			&Expr::Op2(ref a, ref op, ref b) => {
				let a = self.eval(a, ctxt).unwrap();
				let b = self.eval(b, ctxt).unwrap();

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
			&Expr::If { ref condition, ref body } => {
				if let Val::Bool(true) = self.eval(&**condition, ctxt).unwrap() {
					for node in body.nodes.iter() {
						self.exec_node(node, ctxt);
					}
					return None; // maybe let it return something later
				} else { None }
			},
			_ => unimplemented!(),
		}
	}
}
