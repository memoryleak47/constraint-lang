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
		tag!(")") >>
		ignore0 >>
		(PostOp::FunCall(args))
	)
);

#[test]
fn test1() {
	let (i, _) = parse_fun_call("(2,3)".as_bytes()).unwrap();
	assert!(i.is_empty());
}
