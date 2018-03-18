use ast::*;

use ast::parse::ignore::ignore0;
use ast::parse::expr::parse_expr;
use ast::parse::parse_ast;
use ast::parse::keyword::parse_keyword;

named!(pub parse_if<AstNode>,
	do_parse!(
		call!(parse_keyword, "if") >>
		ignore0 >>
		cond: parse_expr >>
		ignore0 >>
		char!('{') >>
		body: parse_ast >>
		char!('}') >>
		ignore0 >>
		(AstNode::If(cond, Box::new(body)))
	)
);
