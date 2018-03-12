use nom::{IResult, ErrorKind};

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

named!(pub parse_name<String>,
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


