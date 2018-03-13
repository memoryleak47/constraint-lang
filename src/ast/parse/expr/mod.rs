mod num;
mod fun_call;

use self::num::parse_expr_num;

use ast::*;
use super::ignore::ignore0;

named!(pub parse_expr_non_fun_call<Expr>,
	alt_complete!( parse_expr_num )
);

pub use self::fun_call::parse_expr_opt_fun_call as parse_expr;

named!(pub parse_expr_statement<AstNode>,
	do_parse!(
		expr: parse_expr >>
		tag!(";") >>
		ignore0 >>
		(AstNode::Expr(expr))
	)
);

#[test]
fn test_fun_call_expr_statement() {
	assert!(parse_expr_statement("2(2,3);".as_bytes()).unwrap().0.is_empty());
}

#[test]
fn test_fun_call_expr() {
	use nom::IResult;

	let x = parse_expr("2(2,3)".as_bytes());
	println!("{:?}", x);
	if let IResult::Done(_, Expr::FunCall { .. }) = x {
	} else { assert!(false); }
}
