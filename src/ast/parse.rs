use ast::*;

use nom::{IResult, alpha, multispace, ErrorKind};
use std::str::from_utf8;

named!(parse_one_line_comment,
	do_parse!(
		tag!("#") >>
		text: take_until!("\n") >>
		tag!("\n") >>
		(text)
	)
);

named!(parse_multiline_comment,
	do_parse!(
		tag!("#{") >>
		text: take_until!("}#") >>
		tag!("}#") >>
		(text)
	)
);

named!(ignore1<Vec<&[u8]>>,
	many1!(alt!(
		multispace
		| parse_multiline_comment
		| parse_one_line_comment
	))
);

named!(ignore0<Vec<&[u8]>>,
	many0!(alt!(
		multispace
		| parse_multiline_comment
		| parse_one_line_comment
	))
);

fn letter(data: &[u8]) -> IResult<&[u8], char> {
	if let Some(&x) = data.get(0) {
		let c = x as char;
		if  (c >= 'a' && c <= 'z') ||
			(c >= 'A' && c <= 'Z') {
			return IResult::Done(&data[1..], c);
		}
	}

	return IResult::Error(ErrorKind::Custom(42));
}

fn digit(data: &[u8]) -> IResult<&[u8], char> {
	if let Some(&x) = data.get(0) {
		let c = x as char;
		if c.is_digit(10) {
			return IResult::Done(&data[1..], c);
		}
	}

	return IResult::Error(ErrorKind::Custom(42));
}

named!(init_name_letter<char>,
	alt!(letter | char!('_'))
);

named!(name_letter<char>,
	alt!(init_name_letter | digit)
);

named!(parse_name<String>,
	do_parse!(
		initial: init_name_letter >>
		rest: many1!(name_letter) >>
		({
			let mut x = String::new();
			x.push(initial);

			let rest_string: String = rest.into_iter().collect();
			x.push_str(&rest_string);
			x
		})
	)
);

named!(parse_c_expr_named<CExpr>,
	do_parse!(
		name: parse_name >>
		(CExpr::Var { name })
	)
);

named!(parse_c_expr_until_semicolon<CExpr>, // TODO add other CExpr types!
	do_parse!(
		v: parse_c_expr_named >>
		(v)
	)
);

named!(parse_constraint_def<AstNode>,
    do_parse!(
        tag!("constraint") >>
        ignore1 >>
        name: map_res!(
            alpha,
            from_utf8
        ) >>
        ignore0 >>
		tag!("=") >>
        ignore0 >>
		c_expr: parse_c_expr_until_semicolon >>
        tag!(";") >>
        ignore0 >>
        (AstNode::CDef(CDef {
			name: String::from(name),
			body: c_expr
		}))
    )
);

// TODO
named!(parse_expr_until_semicolon<Expr>,
	do_parse!(
		val: take_until!(";") >>
		(Expr { val: from_utf8(val).unwrap().to_string() })
	)
);

named!(parse_var_def<AstNode>,
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

named!(parse_expr_statement<AstNode>,
	do_parse!(
		expr: parse_expr_until_semicolon >>
		tag!(";") >>
		ignore0 >>
		(AstNode::Expr(expr))
	)
);

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
