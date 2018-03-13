use ast::*;

use super::super::ignore::ignore0;
use super::super::name::parse_name;

named!(pub parse_expr_var<Expr>,
	do_parse!(
		name: parse_name >>
		ignore0 >>
		(Expr::Var(name))
	)
);
