use ast::*;

use ast::parse::ignore::ignore0;
use ast::parse::expr::parse_expr;
use ast::parse::parse_ast;
use ast::parse::keyword::parse_keyword;

named!(pub parse_while<AstNode>,
	do_parse!(
		call!(parse_keyword, "while") >>
		ignore0 >>
		condition: parse_expr >>
		ignore0 >>
		char!('{') >>
		ignore0 >>
		body: parse_ast >>
		char!('}') >>
		ignore0 >>
		(AstNode::While(condition, body))
	)
);
