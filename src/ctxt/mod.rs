// Context - The immutable part of the Execution Environment

mod build;

pub use self::build::build;

use ast::Ast;

pub struct CCtxt; // constraint context // TODO

pub struct Ctxt {
	pub ast: Ast,
	pub cctxt: CCtxt
}
