use ast::*;

use ast::parse::name::name_letter;

use std::str::from_utf8;

named!(pub parse_expr_bool<Expr>,
	do_parse!(
		val: alt!(tag!("true") | tag!("false")) >>
		peek!(not!(name_letter)) >>
		({
			match from_utf8(val).unwrap() {
				"true" => Expr::Bool(true),
				"false" => Expr::Bool(false),
				_ => panic!("This should not happen")
			}
		})
	)
);
