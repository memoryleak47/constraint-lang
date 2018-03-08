mod parse;

pub use self::parse::parse;

// C is short for Constraint

#[derive(Debug)]
pub struct Ast {
	pub nodes: Vec<AstNode>
}

#[derive(Debug)]
pub enum AstNode {
	VarDef(VarDef),
	CDef(CDef)
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

/*
#[derive(Debug)]
pub enum CExpr {
	And(Box<CExpr>, Box<CExpr>),
	Or(Box<CExpr>, Box<CExpr>),
	Var { name: String },
	CBlock(CBlock)
}
*/

// eg. `{} | {} & Null`
#[derive(Debug)]
pub struct CExpr { // TODO remove this
	pub val: String
}

// eg. `f: function`
#[derive(Debug)]
pub struct CItem {
	pub name: String,
	pub c_expr: Option<CExpr>
}

// eg. `1+foo() > "nice"`
#[derive(Debug)]
pub struct Expr {
	pub val: String // TODO make this more precise
}

#[derive(Debug)]
pub enum VarDefPrefix { Let, Global }

// eg. `let x = 2;`
#[derive(Debug)]
pub struct VarDef {
	pub name: String,
	pub expr: Option<Expr>,
	pub prefix: Option<VarDefPrefix>
}
