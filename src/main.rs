#[macro_use]
extern crate nom;

mod cli;
mod ast;
mod exec;

use cli::get_code;
use ast::parse;
use exec::exec;

fn main2() -> Result<(), String> {
	exec(
		get_code()
			.and_then(parse)?
	);
	Ok(())
}

fn main() {
	if let Err(x) = main2() {
		println!("Error: {}", x);
	}
}
