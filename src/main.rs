#[macro_use]
extern crate nom;

mod cli;
mod ast;
mod ctxt;
mod exec;

use cli::get_code;
use exec::exec;

fn main2() -> Result<(), String> {
	exec(
		get_code()
			.and_then(ast::parse)
			.and_then(ctxt::build)?
	);
	Ok(())
}

fn main() {
	if let Err(x) = main2() {
		println!("Error: {}", x);
	}
}
