use ast::*;

use super::super::ignore::ignore0;
use super::super::expr::{parse_expr_non_fun_call, parse_expr};

// This parses an expression, and checks whether it's a function which will be directly executed, in this case, the arguments will be parsed
named!(pub parse_expr_opt_fun_call<Expr>,
	do_parse!(
		fun: parse_expr_non_fun_call >>
		ignore0 >>
		calls: many0!( // f()() -> calls.len() == 2
			do_parse!(
				tag!("(") >>
				ignore0 >>
				args: separated_list_complete!(
					do_parse!(tag!(",") >> ignore0 >> (())),
					parse_expr
				) >>
				tag!(")") >>
				ignore0 >>
				(args)
			)
		) >>
		({
			let mut x = fun;
			for y in calls {
				x = Expr::FunCall { fun: Box::new(x), args: y };
			}

			x
		})
	)
);

#[test]
fn test_parse_expr_fun_call() {
	assert!(parse_expr_opt_fun_call("2(2,3)".as_bytes()).unwrap().0.is_empty());
}
