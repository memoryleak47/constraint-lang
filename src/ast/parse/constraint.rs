use ast::*;
use super::ignore::{ignore1, ignore0};
use super::name::parse_name;

named!(parse_c_expr_named<CExpr>,
	do_parse!(
		name: parse_name >>
		(CExpr::Var { name })
	)
);

named!(parse_c_expr_block<CExpr>,
	do_parse!(
		tag!("{") >>
        ignore0 >>
		items: separated_list!(
			tag!(","),
			parse_c_item
		) >>
        ignore0 >>
		tag!("}") >>
		(CExpr::CBlock(CBlock {
			items
		}))
	)
);

named!(pub parse_c_expr<CExpr>, // TODO add And / Or
	alt!(parse_c_expr_named | parse_c_expr_block)
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
	alt!( parse_c_item_only_name | parse_c_item_with_constraint )
);

named!(parse_c_item_only_name<CItem>,
	do_parse!(
		name: parse_name >>
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
		(CItem {
			name,
			c_expr: Some(c_expr)
		})
	)
);
