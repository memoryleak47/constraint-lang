use std::str::from_utf8;

named_args!(pub parse_keyword<'a>(x: &'a str)<String>,
	do_parse!(
		t: tag!(x) >>
		peek!(not!(::ast::parse::name::name_letter)) >>
		(from_utf8(t).unwrap().to_string())
	)
);

#[test]
fn test_parse_keyword_good() {

	named!(parse_fun<String>,
		call!(parse_keyword, "fun")
	);

	parse_fun("fun(".as_bytes()).unwrap();
}

#[test]
#[should_panic]
fn test_parse_keyword_bad() {

	named!(parse_fun<String>,
		call!(parse_keyword, "fun")
	);

	parse_fun("funx".as_bytes()).unwrap();
}
