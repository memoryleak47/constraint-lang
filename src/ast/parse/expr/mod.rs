mod num;
mod fun_call;

use self::num::parse_expr_num;
use self::fun_call::parse_expr_fun_call;

use ast::*;
use super::ignore::ignore0;

named!(pub parse_expr<Expr>,
	alt!( parse_expr_num | parse_expr_fun_call )
);

named!(pub parse_expr_non_fun_call<Expr>,
	alt!( parse_expr_num )
);

named!(pub parse_expr_statement<AstNode>,
	do_parse!(
		expr: parse_expr >>
		tag!(";") >>
		ignore0 >>
		(AstNode::Expr(expr))
	)
);
