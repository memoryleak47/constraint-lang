use ast::*;
use super::super::ignore::ignore0;

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

named!(pub parse_expr_num<Expr>,
	do_parse!(
		negative: opt!(char!('-')) >>
		ignore0 >>
		num: parse_expr_pos_num >>
		ignore0 >>
		({
			if negative.is_some() {
				Expr::Val(Val::Num(-num))
			} else {
				Expr::Val(Val::Num(num))
			}
		})
	)
);
