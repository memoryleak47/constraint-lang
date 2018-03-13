use ast::*;

use super::super::ignore::ignore0;
use super::super::expr::{parse_expr_non_fun_call, parse_expr};

named!(pub parse_expr_fun_call<Expr>,
	do_parse!(
		fun: parse_expr_non_fun_call >>
		ignore0 >>
		tag!("(") >>
		ignore0 >>
		args: separated_list_complete!(
			do_parse!(tag!(",") >> ignore0 >> (())),
			parse_expr
		) >>
		tag!(")") >>
		ignore0 >>
		(Expr::FunCall { fun: Box::new(fun), args })
	)
);

#[test]
fn test_parse_expr_fun_call() {
	assert!(parse_expr_fun_call("2(2,3)".as_bytes()).unwrap().0.is_empty());
}
