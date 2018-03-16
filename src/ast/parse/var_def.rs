// VarDec / VarSet

use ast::*;

use super::ignore::*;
use super::expr::*;
use super::name::parse_name;
use ast::parse::keyword::parse_keyword;

named!(parse_var_set<AstNode>,
	do_parse!(
		name: parse_name >>
		ignore0 >>
		tag!("=") >>
		ignore0 >>
		expr: parse_expr >>
		tag!(";") >>
		ignore0 >>
		(AstNode::VarSet(VarSet {
			name: String::from(name),
			expr
		}))
	)
);

named!(parse_var_dec<AstNode>,
	do_parse!(
		prefix: alt!(call!(parse_keyword, "let") | call!(parse_keyword, "global")) >>
		ignore1 >>
		name: parse_name >>
		ignore0 >>
		tag!(";") >>
		ignore0 >>
		(AstNode::VarDec(VarDec {
			name: String::from(name),
			prefix: match prefix.as_str() {
					"let" => VarDecPrefix::Let,
					"global" => VarDecPrefix::Global,
					_ => panic!("This should not happen!")
			},
			cexpr: None // TODO parse dat cexpr
		}))
	)
);

// used for combinations of VarDec and VarSet, eg. `let x = 2;`
// This only consumes `let ` and creates the apropriate AstNode::VarDec, then afterwards the `x = 2;` will be parsed normally
named!(parse_var_dec_peeky<AstNode>,
	do_parse!(
		prefix: alt!(call!(parse_keyword, "let") | call!(parse_keyword, "global")) >>
		ignore1 >>
		name: peek!(
			do_parse!(
				name: parse_name >>
				ignore0 >>
				tag!("=") >>
				ignore0 >>
				(name)
			)
		) >>
		(AstNode::VarDec(VarDec {
			name: String::from(name),
			prefix: match prefix.as_str() {
					"let" => VarDecPrefix::Let,
					"global" => VarDecPrefix::Global,
					_ => panic!("This should not happen!")
			},
			cexpr: None // TODO parse dat cexpr
		}))
	)
);

named!(pub parse_var_def<AstNode>,
	alt!( parse_var_set | parse_var_dec | parse_var_dec_peeky )
);
