pub trait Op {
	fn prio(&self) -> u8;
}

#[derive(Debug, Clone)]
pub enum Op2 {
	LessEq,
	Eq,
	GreaterEq,
	NotEq,

	Less,
	Greater,

	Plus, // a + b
	Minus, // a - b
	Mul, // a * b
	Div, // a / b
	Mod, // a % b
}

impl Op for Op2 {
	fn prio(&self) -> u8 {
		match self {
			&Op2::LessEq => 8,
			&Op2::Eq => 7,
			&Op2::GreaterEq => 8,
			&Op2::NotEq => 7,

			&Op2::Less => 8,
			&Op2::Greater => 8,

			&Op2::Plus => 5,
			&Op2::Minus => 5,
			&Op2::Mul => 6,
			&Op2::Div => 6,
			&Op2::Mod => 6,
		}
	}
}

#[derive(Debug, Clone)]
pub enum PreOp {
	Minus, // -f
	Not, // !b
}

impl Op for PreOp {
	fn prio(&self) -> u8 {
		match self {
			&PreOp::Minus => 9,
			&PreOp::Not => 9,
		}
	}
}

#[derive(Debug, Clone)]
pub enum PostOp<T> {
	FunCall(T) // args
}

impl<T> Op for PostOp<T> {
	fn prio(&self) -> u8 {
		match self {
			&PostOp::FunCall(_) => 10
		}
	}
}
