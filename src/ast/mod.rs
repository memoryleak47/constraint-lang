mod parse;

pub use self::parse::parse;

use std::collections::HashMap;

// C is short for Constraint

#[derive(Debug)]
pub struct Ast {
	pub nodes: Vec<AstNode>
}

#[derive(Debug)]
pub enum AstNode {
	VarDec(VarDec),
	VarSet(VarSet),
	CDef(CDef),
	Expr(Expr)
}

// eg. `constraint X = {} | {};`
#[derive(Debug)]
pub struct CDef {
	pub name: String,
	pub body: CExpr
}

// eg. `{ f: Float, g: Int }`
#[derive(Debug)]
pub struct CBlock {
	pub items: Vec<CItem>
}

#[derive(Debug)]
pub enum CExpr {
	And(Box<CExpr>, Box<CExpr>),
	Or(Box<CExpr>, Box<CExpr>),
	Var(String),
	CBlock(CBlock)
}

// eg. `f: function`
#[derive(Debug)]
pub struct CItem {
	pub name: String,
	pub c_expr: Option<CExpr>
}

#[derive(Debug)]
pub enum Op1 {
	Minus, // -f
	Len // #array
}

#[derive(Debug)]
pub enum Op2 {
	Plus, // a + b
	Minus, // a - b
	Mul, // a * b
	Div, // a / b
	Mod, // a % b
}

// eg. `1+foo() > "nice"`
#[derive(Debug)]
pub enum Expr {
	Op1(Op1, Box<Expr>),
	Op2(Box<Expr>, Op2, Box<Expr>),
	FunCall { // f(x, y)
		fun: Box<Expr>,
		args: Vec<Expr>
	},
	Fun {
		signature: Vec<(String, Option<CExpr>)>,
		body: Ast
	}, // fun(x, y) { return x+y; }
	String(String),
	Var(String),
	Null, // the `null` expression
	Num(f64),
	Bool(bool),
	Array(Vec<Expr>),
	Tuple(Vec<Expr>),
	Object(HashMap<String, Box<Expr>>) // { f = 2 }
}

#[derive(Debug)]
pub enum VarDecPrefix { Let, Global }

// eg. `let x: Int;`
#[derive(Debug)]
pub struct VarDec {
	pub name: String,
	pub prefix: VarDecPrefix,
	pub cexpr: Option<CExpr>
}

// eg. `x = 2;`
#[derive(Debug)]
pub struct VarSet {
	pub name: String,
	pub expr: Expr
}
