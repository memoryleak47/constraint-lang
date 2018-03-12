use ctxt::Ctxt;

struct ExecState { // TODO add heap, variable state
	ctxt: Ctxt,
	ip: usize // instruction pointer
}

impl ExecState {
	fn new(ctxt: Ctxt) -> ExecState {
		ExecState {
			ctxt,
			ip: 0
		}
	}

	fn run(&mut self) {
		unimplemented!()
	}
}

pub fn exec(ctxt: Ctxt) {
	ExecState::new(ctxt).run();
}
