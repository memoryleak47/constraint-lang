use ctxt::Ctxt;

pub struct ExecState { // TODO add heap, variable state
	ctxt: Ctxt,
	ip: usize // instruction pointer
}

impl ExecState {
	pub fn new(ctxt: Ctxt) -> ExecState {
		ExecState {
			ctxt,
			ip: 0
		}
	}
}
