mod build;
pub use self::build::build as build;

use std::collections::HashMap;

use op::{PreOp, Op2};
use ast::Ast;
use val::Val;
use cexpr::CExpr;
type ExprVal = Val<Vec<CExpr>, Ast>;

pub enum Expr {
	FunCall(Box<Expr>, Vec<Expr>),
	PreOp(PreOp, Box<Expr>),
	Op2(Box<Expr>, Op2, Box<Expr>),
	Val(ExprVal),
	Var(usize),
}

pub enum Command {
	Set(usize, Expr),
	Ifgo(Expr, usize),
	Push,
	Pop,
}

pub struct Bytecode {
	commands: Vec<Command>,
	vars: Vec<String>,
}
