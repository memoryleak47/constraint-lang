use ast::parse::ignore::ignore0;
use ast::parse::constraint::parse_c_items;
use ast::*;
use ast::parse::parse_ast;

named!(pub parse_fun<Expr>,
	do_parse!(
		tag!("fun") >>
		ignore0 >>
		char!('(') >>
		ignore0 >>
		signature: parse_c_items >>
		char!(')') >>
		ignore0 >>
		char!('{') >>
		ignore0 >>
		body: parse_ast >>
		char!('}') >>
		ignore0 >>
		(Expr::Fun { signature, body })
	)
);

#[test]
fn test_fun() {
	let (i, o) = parse_fun("fun(x) { let y = 2; }".as_bytes()).unwrap();
	assert!(i.is_empty());
}

#[test]
fn test_empty_fun() {
	let (i, o) = parse_fun("fun() { }".as_bytes()).unwrap();
	assert!(i.is_empty());
}
