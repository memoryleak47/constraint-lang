use ast::*;
use super::ignore::ignore0;
use super::expr::parse_expr;
use ast::parse::keyword::parse_keyword;

named!(pub parse_ctrl_flow<AstNode>,
	alt_complete!(parse_return | parse_break | parse_continue)
);

named!(parse_return<AstNode>,
	do_parse!(
		call!(parse_keyword, "return") >>
		ignore0 >>
		expr: opt!( parse_expr ) >>
		char!(';') >>
		ignore0 >>
		(AstNode::CtrlFlow(CtrlFlow::Return(expr)))
	)
);

named!(parse_break<AstNode>,
	do_parse!(
		call!(parse_keyword, "break") >>
		ignore0 >>
		char!(';') >>
		ignore0 >>
		(AstNode::CtrlFlow(CtrlFlow::Break))
	)
);

named!(parse_continue<AstNode>,
	do_parse!(
		call!(parse_keyword, "continue") >>
		ignore0 >>
		char!(';') >>
		ignore0 >>
		(AstNode::CtrlFlow(CtrlFlow::Continue))
	)
);
