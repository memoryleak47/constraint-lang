mod parse;

pub use self::parse::parse;

use std::collections::HashMap;

// C is short for Constraint

#[derive(Debug, Clone)]
pub struct Ast {
	pub nodes: Vec<AstNode>
}

#[derive(Debug, Clone)]
pub enum AstNode {
	VarDec(VarDec),
	VarSet(VarSet),
	CDef(CDef),
	Expr(Expr),
	Return(Expr),
}

// eg. `constraint X = {} | {};`
#[derive(Debug, Clone)]
pub struct CDef {
	pub name: String,
	pub body: CExpr
}

// eg. `{ f: Float, g: Int }`
#[derive(Debug, Clone)]
pub struct CBlock {
	pub items: Vec<CItem>
}

#[derive(Debug, Clone)]
pub enum CExpr {
	And(Box<CExpr>, Box<CExpr>),
	Or(Box<CExpr>, Box<CExpr>),
	Var(String),
	CBlock(CBlock)
}

// eg. `f: function`
#[derive(Debug, Clone)]
pub struct CItem {
	pub name: String,
	pub c_expr: Option<CExpr>
}

trait Op {
	fn prio(&self) -> u8;
}

#[derive(Debug, Clone)]
pub enum PreOp {
	Minus, // -f
	Not, // !b
}

impl Op for PreOp {
	fn prio(&self) -> u8 {
		match self {
			&PreOp::Minus => 9,
			&PreOp::Not => 9,
		}
	}
}

#[derive(Debug, Clone)]
pub enum PostOp {
	FunCall(Vec<Expr>) // args
}

impl Op for PostOp {
	fn prio(&self) -> u8 {
		match self {
			&PostOp::FunCall(_) => 10
		}
	}
}

#[derive(Debug, Clone)]
pub enum Op2 {
	LessEq,
	Eq,
	GreaterEq,
	NotEq,

	Less,
	Greater,

	Plus, // a + b
	Minus, // a - b
	Mul, // a * b
	Div, // a / b
	Mod, // a % b
}

impl Op for Op2 {
	fn prio(&self) -> u8 {
		match self {
			&Op2::LessEq => 8,
			&Op2::Eq => 7,
			&Op2::GreaterEq => 8,
			&Op2::NotEq => 7,

			&Op2::Less => 8,
			&Op2::Greater => 8,

			&Op2::Plus => 5,
			&Op2::Minus => 5,
			&Op2::Mul => 6,
			&Op2::Div => 6,
			&Op2::Mod => 6,
		}
	}
}

// eg. `1+foo() > "nice"`
#[derive(Debug, Clone)]
pub enum Expr {
	PreOp(PreOp, Box<Expr>),
	PostOp(Box<Expr>, PostOp),
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

	If {
		condition: Box<Expr>,
		body: Ast
	}
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
