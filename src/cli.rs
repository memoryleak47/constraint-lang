use std::env;
use std::fs::File;
use std::io::Read;

pub fn get_code() -> Result<String, String> {
	let filename = env::args().nth(1)
		.ok_or_else(|| String::from("Could not find file!"))?;
	let mut file = File::open(filename)
		.map_err(|_| String::from("Could not open file!"))?;

	let mut output = String::new();
	file.read_to_string(&mut output)
		.map_err(|_| String::from("Could not read from file!"))?;

	Ok(output)
}
