mod name;
mod ignore;
mod var_def;
mod expr;
mod constraint;

use self::ignore::ignore0;
use self::var_def::parse_var_def;
use self::expr::parse_expr_statement;
use self::constraint::parse_constraint_def;

use ast::*;

use nom::IResult;
use std::str::from_utf8;

named!(parse_ast_node<AstNode>,
	alt!(parse_constraint_def | parse_var_def | parse_expr_statement)
);

named!(parse_ast<Ast>,
	do_parse!(
		ignore0 >>
		nodes: many0!(parse_ast_node) >>
		(Ast { nodes })
	)
);

pub fn parse(s: String) -> Result<Ast, String> {
	let parsed = parse_ast(s.as_bytes());
	println!("{:?}", parsed);

	match parsed {
		IResult::Done(rest, x) => {
			if rest.is_empty() { return Ok(x); }
			return Err(format!("Couldn't parse {:?}", from_utf8(rest)));
		},
		_ => panic!("TODO")
	}
}
