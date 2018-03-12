mod heap;
mod eval;

use self::heap::Heap;
use ctxt::Ctxt;
use ast::AstNode;
use ast::VarDef;

use std::collections::HashMap;

pub enum Val {
	Null,
	Bool(bool),
	String(String),
	Num(f64),
	Array(Vec<Val>),
	Tuple(Vec<Val>),
	Object(HashMap<String, Box<Val>>),
	Fun // TODO add members
}

// The mutable part of the Execution Environment
struct ExecState {
	heap: Heap<Val>,
	stack: Vec<HashMap<String, Option<usize>>>
}

pub fn exec(ctxt: Ctxt) {
	let mut state = ExecState::new();
	state.exec(ctxt);
}

impl ExecState {
	fn new() -> ExecState {
		ExecState {
			stack: vec![HashMap::new()],
			heap: Heap::new()
		}
	}

	fn global(&self) -> &HashMap<String, Option<usize>> {
		self.stack.first().unwrap()
	}

	fn global_mut(&mut self) -> &mut HashMap<String, Option<usize>> {
		self.stack.first_mut().unwrap()
	}

	fn local(&self) -> &HashMap<String, Option<usize>> {
		self.stack.last().unwrap()
	}

	fn local_mut(&mut self) -> &mut HashMap<String, Option<usize>> {
		self.stack.last_mut().unwrap()
	}

	fn find_var(&self, name: &str) -> Option<usize> {
		for (i, scope) in self.stack.iter().rev().enumerate() { 
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

	fn exec(&mut self, ctxt: Ctxt) {
		for node in ctxt.ast.nodes.iter() {
			self.exec_node(node, &ctxt);
		}
	}

	fn exec_node(&mut self, node: &AstNode, ctxt: &Ctxt) {
		match node {
			&AstNode::VarDef(VarDef { ref name, ref expr, ref prefix }) => {
				// TODO eval the expr, and set the value!
			},
			&AstNode::Expr(ref expr) => {
				self.eval(expr, ctxt);
			},
			_ => panic!("This should not happen")
		}
	}
}
