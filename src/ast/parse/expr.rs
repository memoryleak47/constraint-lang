use ast::*;
use super::ignore::ignore0;

use nom::IResult;

pub fn parse_expr(data: &[u8]) -> IResult<&[u8], Expr> {
	unimplemented!()
}

named!(pub parse_expr_statement<AstNode>,
	do_parse!(
		expr: parse_expr >>
		tag!(";") >>
		ignore0 >>
		(AstNode::Expr(expr))
	)
);
