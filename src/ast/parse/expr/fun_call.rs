use ast::*;

use super::super::ignore::ignore0;
use super::super::expr::parse_expr;

named!(pub parse_fun_call<PostOp>,
	do_parse!(
		tag!("(") >>
		ignore0 >>
		args: separated_list_complete!(
			do_parse!( tag!(",") >> ignore0 >> (())),
			parse_expr
		) >>
		( PostOp::FunCall(args) )
	)
);
