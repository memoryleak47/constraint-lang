mod heap;
mod expr;

use self::heap::Heap;
use ctxt::Ctxt;
use ast::{Ast, CItem, AstNode, VarDecPrefix, VarDec, VarSet};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Val {
	Null,
	Bool(bool),
	String(String),
	Num(f64),
	Array(Vec<Val>),
	Tuple(Vec<Val>),
	Object(HashMap<String, Box<Val>>),
	Fun { signature: Vec<CItem>, body: Ast }
}

// The mutable part of the Execution Environment
struct ExecState {
	heap: Heap<Val>,
	stack: Vec<HashMap<String, Option<usize>>>
}

pub fn exec(ctxt: Ctxt) {
	let mut state = ExecState::new();
	state.exec_ast(&ctxt.ast, &ctxt);
}

impl ExecState {
	fn new() -> ExecState {
		ExecState {
			stack: vec![HashMap::new()],
			heap: Heap::new()
		}
	}

	fn global_mut(&mut self) -> &mut HashMap<String, Option<usize>> {
		self.stack.first_mut().unwrap()
	}

	fn local_mut(&mut self) -> &mut HashMap<String, Option<usize>> {
		self.stack.last_mut().unwrap()
	}

	fn push_stack(&mut self) {
		self.stack.push(HashMap::new());
	}

	fn pop_stack(&mut self) {
		self.stack.pop();
	}

	fn find_var(&self, name: &str) -> Option<usize> {
		for (i, scope) in self.stack.iter().enumerate().rev() { 
			if scope.get(name).is_some() { return Some(i); }
		}
		None
	}

	fn get_var(&self, name: &str) -> Option<usize> {
		self.find_var(name)
			.and_then(|i| self.stack[i][name])
	}

	fn set_var(&mut self, name: &str, pointer: usize) {
		if let Some(i) = self.find_var(name) {
			self.stack[i].insert(name.to_string(), Some(pointer));
		}
	}

	fn exec_ast(&mut self, ast: &Ast, ctxt: &Ctxt) -> Option<Val> {
		for node in ast.nodes.iter() {
			if let Some(v) = self.exec_ast_node(node, ctxt) {
				return Some(v);
			}
		}
		return None;
	}

	pub fn exec_ast_node(&mut self, node: &AstNode, ctxt: &Ctxt) -> Option<Val> {
		match node {
			&AstNode::VarDec(VarDec { ref name, ref prefix, .. }) => {
				(match prefix {
					&VarDecPrefix::Let => self.local_mut(),
					&VarDecPrefix::Global => self.global_mut(),
				}).insert(name.to_string(), None);
			},
			&AstNode::VarSet(VarSet { ref name, ref expr }) => {
				let val = self.exec_expr(expr, ctxt).unwrap();
				let i = self.heap.alloc(val);
				self.set_var(name, i);
			},
			&AstNode::Expr(ref expr) => {
				self.exec_expr(expr, ctxt);
			},
			&AstNode::If { ref cases, ref otherwise } => {
				for &(ref condition, ref body) in cases {
					if let Val::Bool(true) = self.exec_expr(condition, ctxt).unwrap() {
						self.push_stack();
						let res = self.exec_ast(body, ctxt);
						self.pop_stack();
						return res;
					}
				}

				if let &Some(ref x) = otherwise {
					self.push_stack();
					let res = self.exec_ast(&**x, ctxt);
					self.pop_stack();
					return res;
				} else { return None; }
			},
			&AstNode::Return(ref expr) => {
				return Some(self.exec_expr(expr, ctxt).unwrap());
			},
			_ => panic!("This should not happen")
		};

		return None;
	}
}
