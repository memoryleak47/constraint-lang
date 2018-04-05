mod parse;

pub use self::parse::parse;
use ::op::{PreOp, Op2, PostOp};
use ::cexpr::{CExpr, CItem};
use ::val::Val;

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
	Val(Val<Vec<CItem>, Ast>),
	Var(String),
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
