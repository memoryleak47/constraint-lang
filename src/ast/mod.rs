mod parse;

pub use self::parse::parse;

#[derive(Debug)]
pub struct Ast {
	pub nodes: Vec<AstNode>
}

#[derive(Debug)]
pub struct ConstraintDef {
	pub name: String
}

#[derive(Debug)]
pub struct LetExpression {
	pub name: String
}

#[derive(Debug)]
pub enum AstNode {
	LetExpression(LetExpression),
	ConstraintDef(ConstraintDef)
}
