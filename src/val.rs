use std::collections::HashMap;

pub enum Val<S, T> {
	Fun {
		signature: S,
		body: T
	}, // fun(x, y) { return x+y; }
	String(String),
	Null, // the `null` expression
	Num(f64),
	Bool(bool),
	Array(Vec<Val<S, T>>),
	Tuple(Vec<Val<S, T>>),
	Object(HashMap<String, Box<Val<S, T>>>), // { f = 2 }
}
