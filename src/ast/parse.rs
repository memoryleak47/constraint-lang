use ast::{Ast, AstNode, ConstraintDef, LetExpression};

use nom::{IResult, alpha, multispace};
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

named!(parse_constraint<AstNode>,
    do_parse!(
        tag!("constraint") >>
        ignore1 >>
        name: map_res!(
            alpha,
            from_utf8
        ) >>
        ignore0 >>
        tag!("{") >>
        ignore0 >>
        tag!("}") >>
        ignore0 >>
        (AstNode::ConstraintDef(ConstraintDef { name: String::from(name) }))
    )
);

named!(parse_let<AstNode>,
	do_parse!(
		tag!("let") >>
		ignore1 >>
		name: map_res!(
			alpha,
			from_utf8
		) >>
		ignore0 >>
		tag!(";") >>
        ignore0 >>
		(AstNode::LetExpression(LetExpression { name: String::from(name) }))
	)
);

named!(parse_ast_node<AstNode>,
	alt!(parse_constraint | parse_let)
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
