use ast::*;
use super::ignore::ignore0;
use super::expr::parse_expr;
use ast::parse::keyword::parse_keyword;

named!(pub parse_return<AstNode>,
	do_parse!(
		call!(parse_keyword, "return") >>
		ignore0 >>
		expr: parse_expr >>
		char!(';') >>
		ignore0 >>
		(AstNode::Return(expr))
	)
);

