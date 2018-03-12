use ast::*;
use nom::alpha;
use super::ignore::{ignore1, ignore0};
use super::name::parse_name;

use std::str::from_utf8;

named!(parse_c_expr_named<CExpr>,
	do_parse!(
		name: parse_name >>
		(CExpr::Var { name })
	)
);

named!(pub parse_c_expr_until_semicolon<CExpr>, // TODO add other CExpr types!
	do_parse!(
		v: parse_c_expr_named >>
		(v)
	)
);

named!(pub parse_constraint_def<AstNode>,
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


