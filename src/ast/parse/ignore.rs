use nom::multispace;

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

named!(pub ignore1<Vec<&[u8]>>,
	many1!(alt!(
		multispace
		| parse_multiline_comment
		| parse_one_line_comment
	))
);

named!(pub ignore0<Vec<&[u8]>>,
	many0!(alt!(
		multispace
		| parse_multiline_comment
		| parse_one_line_comment
	))
);
