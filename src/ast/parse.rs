use ast::{Ast, AstNode, ConstraintDef, LetExpression};

use nom::{IResult, space, alpha, alphanumeric, digit, multispace};
use std::str::from_utf8;

named!(parse_constraint<AstNode>,
    do_parse!(
        tag!("constraint") >>
        multispace >>
        name: map_res!(
            alpha,
            from_utf8
        ) >>
        opt!(multispace) >>
        tag!("{") >>
        opt!(multispace) >>
        tag!("}") >>
        opt!(multispace) >>
        (AstNode::ConstraintDef(ConstraintDef { name: String::from(name) }))
    )
);

named!(parse_let<AstNode>,
	do_parse!(
		tag!("let") >>
		multispace >>
		name: map_res!(
			alpha,
			from_utf8
		) >>
		opt!(multispace) >>
		tag!(";") >>
        opt!(multispace) >>
		(AstNode::LetExpression(LetExpression { name: String::from(name) }))
	)
);

named!(parse_ast_node<AstNode>,
	alt!(parse_constraint | parse_let)
);

named!(parse_ast<Ast>,
	do_parse!(
		nodes: many0!(parse_ast_node) >>
		(Ast { nodes })
	)
);

pub fn parse(s: String) -> Result<Ast, String> {
	let parsed = parse_ast(s.as_bytes());
	println!("{:?}", parsed);

	match parsed {
		IResult::Done(_, x) => return Ok(x),
		_ => panic!("TODO")
	}
}
