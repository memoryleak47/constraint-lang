use ast::*;

use ast::parse::ignore::ignore0;
use ast::parse::expr::parse_expr;
use ast::parse::parse_ast;

named!(pub parse_expr_if<Expr>,
	do_parse!(
		tag!("if") >>
		ignore0 >>
		cond: parse_expr >>
		ignore0 >>
		char!('{') >>
		body: parse_ast >>
		char!('}') >>
		ignore0 >>
		(Expr::If { condition: Box::new(cond), body })
	)
);
