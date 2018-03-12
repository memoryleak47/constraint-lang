use ast::*;
use super::ignore::*;

use std::str::from_utf8;

// TODO
named!(pub parse_expr_until_semicolon<Expr>,
	do_parse!(
		val: take_until!(";") >>
		(Expr { val: from_utf8(val).unwrap().to_string() })
	)
);

named!(pub parse_expr_statement<AstNode>,
	do_parse!(
		expr: parse_expr_until_semicolon >>
		tag!(";") >>
		ignore0 >>
		(AstNode::Expr(expr))
	)
);


