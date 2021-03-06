use ast::*;

use ast::parse::keyword::parse_keyword;

named!(pub parse_expr_bool<Expr>,
	do_parse!(
		val: alt!(call!(parse_keyword, "true") | call!(parse_keyword, "false")) >>
		({
			match val.as_str() {
				"true" => Expr::Bool(true),
				"false" => Expr::Bool(false),
				_ => panic!("This should not happen")
			}
		})
	)
);
