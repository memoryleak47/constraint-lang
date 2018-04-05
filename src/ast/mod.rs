mod parse;

pub use self::parse::parse;
use ::op::{PreOp, Op2, PostOp};
use ::cexpr::{CExpr, CItem};

use std::collections::HashMap;

// C is short for Constraint

#[derive(Debug, Clone)]
pub struct Ast {
	pub nodes: Vec<AstNode>
}

#[derive(Debug, Clone)]
pub enum CtrlFlow<T: Clone> {
	Return(Option<T>),
	Break,
	Continue,
}

impl<T: Clone> CtrlFlow<T> {
	pub fn map<F, U: Clone>(&self, f: F) -> CtrlFlow<U>
		where F: FnOnce(T) -> U
		{
			match self {
				&CtrlFlow::Return(ref x) => CtrlFlow::Return(x.clone().map(f)),
				&CtrlFlow::Break => CtrlFlow::Break,
				&CtrlFlow::Continue => CtrlFlow::Continue,
			}
	}
}

#[derive(Debug, Clone)]
pub enum AstNode {
	VarDec(VarDec),
	VarSet(VarSet),
	CDef(CDef),
	Expr(Expr),
	CtrlFlow(CtrlFlow<Expr>),
	If {
		cases: Vec<(Expr, Ast)>,
		otherwise: Option<Ast>,
	},
	While(Expr, Ast),
}

// eg. `constraint X = {} | {};`
#[derive(Debug, Clone)]
pub struct CDef {
	pub name: String,
	pub body: CExpr
}

// eg. `1+foo() > "nice"`
#[derive(Debug, Clone)]
pub enum Expr {
	PreOp(PreOp, Box<Expr>),
	PostOp(Box<Expr>, PostOp<Vec<Expr>>),
	Op2(Box<Expr>, Op2, Box<Expr>),
	Fun {
		signature: Vec<CItem>,
		body: Ast
	}, // fun(x, y) { return x+y; }
	String(String),
	Var(String),
	Null, // the `null` expression
	Num(f64),
	Bool(bool),
	Array(Vec<Expr>),
	Tuple(Vec<Expr>),
	Object(HashMap<String, Box<Expr>>), // { f = 2 }
}

#[derive(Debug, Clone)]
pub enum VarDecPrefix { Let, Global }

// eg. `let x: Int;`
#[derive(Debug, Clone)]
pub struct VarDec {
	pub name: String,
	pub prefix: VarDecPrefix,
	pub cexpr: Option<CExpr>
}

// eg. `x = 2;`
#[derive(Debug, Clone)]
pub struct VarSet {
	pub name: String,
	pub expr: Expr
}
