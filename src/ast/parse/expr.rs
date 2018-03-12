use ast::*;
use super::ignore::ignore0;

use nom::digit;

use std::str::from_utf8;

named!(parse_expr_pos_int<f64>,
	do_parse!(
		d: digit >>
		ignore0 >>
		(from_utf8(d).unwrap().parse().unwrap())
	)
);

named!(parse_expr_pos_num<f64>,
	alt!( parse_expr_pos_int ) // TODO add floating point parser!
);

named!(parse_expr_num<Expr>,
	do_parse!(
		negative: opt!(char!('-')) >>
		ignore0 >>
		num: parse_expr_pos_num >>
		ignore0 >>
		({
			if negative.is_some() {
				Expr::Num(-num)
			} else {
				Expr::Num(num)
			}
		})
	)
);

named!(pub parse_expr<Expr>,
	do_parse!(
		x: parse_expr_num >>
		(x)
	)
);

named!(pub parse_expr_statement<AstNode>,
	do_parse!(
		expr: parse_expr >>
		tag!(";") >>
		ignore0 >>
		(AstNode::Expr(expr))
	)
);
