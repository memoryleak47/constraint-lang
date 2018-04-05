
#[derive(Debug, Clone)]
pub enum CExpr {
	And(Box<CExpr>, Box<CExpr>),
	Or(Box<CExpr>, Box<CExpr>),
	Var(String),
	CBlock(CBlock)
}

// eg. `{ f: Float, g: Int }`
#[derive(Debug, Clone)]
pub struct CBlock {
	pub items: Vec<CItem>
}

// eg. `f: function`
#[derive(Debug, Clone)]
pub struct CItem {
	pub name: String,
	pub c_expr: Option<CExpr>
}
