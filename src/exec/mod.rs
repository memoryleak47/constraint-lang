mod heap;

use self::heap::Heap;
use ctxt::Ctxt;
use ast::AstNode;

use std::collections::HashMap;

struct Object {
	map: HashMap<String, Box<Val>>
}

enum Val {
	Null,
	Bool(bool),
	String(String),
	Num(f64),
	Array(Vec<Val>),
	Tuple(Vec<Val>),
	Object(Object)
}

// The mutable part of the Execution Environment
struct ExecState {
	global: HashMap<String, usize>,
	heap: Heap<Val>
}

pub fn exec(ctxt: Ctxt) {
	let mut state = ExecState::new();
	state.exec(ctxt);
}

impl ExecState {
	fn new() -> ExecState {
		ExecState {
			global: HashMap::new(),
			heap: Heap::new()
		}
	}

	fn exec(&mut self, ctxt: Ctxt) {
		for node in ctxt.ast.nodes.iter() {
			self.exec_node(node);
		}
	}

	fn exec_node(&mut self, node: &AstNode) {
		match node {
			&AstNode::VarDef(_) => {
				unimplemented!();
			},
			&AstNode::Expr(_) => {
				unimplemented!();
			},
			_ => panic!("This should not happen")
		}
	}
}
