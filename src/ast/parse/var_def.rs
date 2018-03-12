use ast::*;
use nom::alpha;
use super::ignore::*;
use super::expr::*;

use std::str::from_utf8;

named!(pub parse_var_def<AstNode>,
	// This split is necessary as `let x;`, `let x = 2;` and `x = 2;` are accepted, but `x;` is not.
	alt!(
		do_parse!( // parse `[let|global] x [= *]`;
			prefix: alt!(tag!("let") | tag!("global")) >>
			ignore1 >>
			name: map_res!(
				alpha,
				from_utf8
			) >>
			ignore0 >>
			expr: opt!(
				do_parse!(
					tag!("=") >>
					ignore0 >>
					expr: parse_expr_until_semicolon >>
					(expr)
				)
			) >>
			tag!(";") >>
			ignore0 >>
			(AstNode::VarDef(VarDef {
				name: String::from(name),
				prefix: Some(match from_utf8(prefix).unwrap() {
						"let" => VarDefPrefix::Let,
						"global" => VarDefPrefix::Global,
						_ => panic!("This should not happen!")
				}),
				expr
			}))
		) |
		do_parse!( // parse `x = *;`
			name: map_res!(
				alpha,
				from_utf8
			) >>
			ignore0 >>
			expr: do_parse!(
					tag!("=") >>
					ignore0 >>
					expr: parse_expr_until_semicolon >>
					(expr)
			) >>
			tag!(";") >>
			ignore0 >>
			(AstNode::VarDef(VarDef {
				name: String::from(name),
				prefix: None,
				expr: Some(expr)
			}))
		)
	)
);
