use ast::{Ast, AstNode};

use super::{Ctxt, CCtxt};

pub fn build(mut ast: Ast) -> Result<Ctxt, String> {
	ast.nodes = ast.nodes.into_iter()
		.filter(|x| if let &AstNode::CDef{..} = x { false } else { true } )
		.collect();

	return Ok(Ctxt {
		ast,
		cctxt: CCtxt
	});
}
