mod build;

pub use self::build::build;

use ast::Ast;

struct CCtxt; // constraint context // TODO 

pub struct Ctxt {
	ast: Ast,
	cctxt: CCtxt
}
