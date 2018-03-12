use ast::*;
use super::ignore::{ignore1, ignore0};
use super::name::parse_name;

use nom::IResult;

named!(parse_c_expr_named<CExpr>,
	do_parse!(
		name: parse_name >>
		ignore0 >>
		(CExpr::Var(name))
	)
);

named!(parse_c_expr_block<CExpr>,
	do_parse!(
		tag!("{") >>
        ignore0 >>
		items: separated_list!(
			do_parse!(tag!(",") >> ignore0 >> (())),
			parse_c_item
		) >>
		tag!("}") >>
        ignore0 >>
		(CExpr::CBlock(CBlock {
			items
		}))
	)
);

fn parse_c_expr_conjunction(data: &[u8]) -> IResult<&[u8], CExpr> { // TODO this requires some refactoring
	if let IResult::Done(d, (first, rest)) = parse_c_expr_conjunction_helper(data) {
		return match rest.into_iter()
			.fold(Some(first), |x, (del, expr)| {
				if let Some(x) = x {
					match del {
						'&' => Some(CExpr::And(Box::new(x), Box::new(expr))),
						'|' => Some(CExpr::Or(Box::new(x), Box::new(expr))),
						_ => None,
					}
				} else { None }
			}) {
				Some(x) => ::nom::IResult::Done(d, x),
				None => ::nom::IResult::Error(::nom::ErrorKind::Custom(42))
			};
	} else {
		return ::nom::IResult::Error(::nom::ErrorKind::Custom(42));
	}
}

named!(parse_c_expr_conjunction_helper<(CExpr, Vec<(char, CExpr)>)>,
	do_parse!(
		first: parse_c_expr_no_conjunction >>
		rest: many1!(
			do_parse!(
				del: alt!(char!('|') | char!('&')) >>
				ignore0 >>
				expr: parse_c_expr_no_conjunction >>
				ignore0 >>
				((del, expr))
			)
		) >>
        ignore0 >>
		(first, rest)
	)
);

named!(parse_c_expr_inner<CExpr>,
	do_parse!(
		tag!("(") >>
		ignore0 >>
		cexpr: parse_c_expr >>
		tag!(")") >>
		ignore0 >>
		(cexpr)
	)
);

named!(pub parse_c_expr_no_conjunction<CExpr>,
	alt!(parse_c_expr_inner | parse_c_expr_named | parse_c_expr_block)
);

named!(pub parse_c_expr<CExpr>,
	alt!(parse_c_expr_conjunction | parse_c_expr_no_conjunction)
);

named!(pub parse_constraint_def<AstNode>,
    do_parse!(
        tag!("constraint") >>
        ignore1 >>
        name: parse_name >>
        ignore0 >>
		tag!("=") >>
        ignore0 >>
		c_expr: parse_c_expr >>
        tag!(";") >>
        ignore0 >>
        (AstNode::CDef(CDef {
			name: String::from(name),
			body: c_expr
		}))
    )
);


// CItem parsing:

named!(parse_c_item<CItem>,
	alt!( parse_c_item_with_constraint | parse_c_item_only_name )
);

named!(parse_c_item_only_name<CItem>,
	do_parse!(
		name: parse_name >>
		ignore0 >>
		(CItem {
			name,
			c_expr: None
		})
	)
);

named!(parse_c_item_with_constraint<CItem>,
	do_parse!(
		name: parse_name >>
		ignore0 >>
		tag!(":") >>
		ignore0 >>
		c_expr: parse_c_expr >>
		ignore0 >>
		(CItem {
			name,
			c_expr: Some(c_expr)
		})
	)
);
