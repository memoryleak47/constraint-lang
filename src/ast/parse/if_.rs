use ast::*;

use ast::parse::ignore::ignore0;
use ast::parse::expr::parse_expr;
use ast::parse::parse_ast;
use ast::parse::keyword::parse_keyword;

named!(pub parse_if<AstNode>,
	do_parse!(
		call!(parse_keyword, "if") >>
		ignore0 >>
		head_cond: parse_expr >>
		ignore0 >>
		char!('{') >>
		ignore0 >>
		head_body: parse_ast >>
		char!('}') >>
		ignore0 >>
		tail_cases: many0!(
			do_parse!(
				call!(parse_keyword, "else") >>
				ignore0 >>
				call!(parse_keyword, "if") >>
				ignore0 >>
				tail_cond: parse_expr >>
				ignore0 >>
				char!('{') >>
				ignore0 >>
				tail_body: parse_ast >>
				char!('}') >>
				ignore0 >>
				((tail_cond, tail_body))
			)
		) >>
		otherwise: opt!(
			complete!(do_parse!(
				call!(parse_keyword, "else") >>
				ignore0 >>
				char!('{') >>
				ignore0 >>
				otherwise_inner: parse_ast >>
				char!('}') >>
				ignore0 >>
				(otherwise_inner)
			))
		) >>
		({
			let mut cases = Vec::new();
			cases.push((head_cond, head_body));
			cases.extend(tail_cases);

			AstNode::If { cases, otherwise }
		})
	)
);
