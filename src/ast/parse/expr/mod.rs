mod num;
mod bool;
mod var;
mod fun;
mod fun_call;

use self::num::parse_expr_num;
use self::bool::parse_expr_bool;
use self::var::parse_expr_var;
use self::fun::parse_expr_fun;
use self::fun_call::parse_fun_call;

use ast::*;
use super::ignore::ignore0;

use std::str::from_utf8;

named!(parse_pre_op<PreOp>,
	do_parse!(
		op: alt_complete!(char!('-')
						| char!('!')
		) >>
		ignore0 >>
		(match op {
			'-' => PreOp::Minus,
			'!' => PreOp::Not,
			_ => panic!("This should not happen!")
		})
	)
);

named!(parse_post_op<PostOp>,
	do_parse!(
		x: parse_fun_call >>
		(x)
	)
);

named!(pub parse_main_expr<Expr>,
	alt_complete!(parse_expr_num | parse_expr_bool | parse_expr_inner | parse_expr_fun | parse_expr_var ) // parse_expr_var has to be behind parse_fun as it will thinks `fun` is a var
);

named!(parse_expr_inner<Expr>,
	do_parse!(
		tag!("(") >>
		ignore0 >>
		expr: parse_expr >>
		tag!(")") >>
		ignore0 >>
		(expr)
	)
);

named!(parse_op2<Op2>,
	do_parse!(
		op: alt_complete!(tag!("<=")
						| tag!("==")
						| tag!(">=")
						| tag!("!=")

						| tag!("<")
						| tag!(">")

						| tag!("+")
						| tag!("-")
						| tag!("*")
						| tag!("/")
						| tag!("%")
		) >>
		ignore0 >>
		(match from_utf8(op).unwrap() {
			"<=" => Op2::LessEq,
			"==" => Op2::Eq,
			">=" => Op2::GreaterEq,
			"!=" => Op2::NotEq,

			"<" => Op2::Less,
			">" => Op2::Greater,

			"+" => Op2::Plus,
			"-" => Op2::Minus,
			"*" => Op2::Mul,
			"/" => Op2::Div,
			"%" => Op2::Mod,
			_ => panic!("This should not happen!")
		})
	)
);

type InitType = (Option<PreOp>, Expr, Option<PostOp>);
type TailElem = (Op2, Option<PreOp>, Expr, Option<PostOp>);

fn assemble_expr(init: InitType, mut tail: Vec<TailElem>) -> Expr {
	const MAX_PRIO: u8 = 100;

	fn prio<T: Op>(x: Option<&T>) -> u8 {
		match x {
			Some(ref op) => op.prio(),
			None => MAX_PRIO,
		}
	}

	use std::cmp::min;

	let mut lowest_prio = MAX_PRIO;
	// find prio()-lowest Op, which will be the outest operator then
	// first PreOp?
	lowest_prio = min(lowest_prio, prio(init.0.as_ref()));

	// some Op2?
	for &(ref op2, _, _, _) in tail.iter() {
		lowest_prio = min(lowest_prio, prio(Some(op2)));
	}

	// last PostOp?
	if let Some(&(_, _, _, ref post_op)) = tail.last() {
		lowest_prio = min(lowest_prio, prio(post_op.as_ref()));
	} else {
		lowest_prio = min(lowest_prio, prio(init.2.as_ref()));
	}

	// there are no more operators!
	if lowest_prio == MAX_PRIO { return init.1; }

	// recursive calls:
	// first PreOp?
	if prio(init.0.as_ref()) == lowest_prio {
		return Expr::PreOp(init.0.unwrap(), Box::new(assemble_expr((None, init.1, init.2), tail)));
	}

	// some Op2?
	if let Some(i) = tail.iter()
		.position(|&(ref op2, _, _, _)| prio(Some(op2)) == lowest_prio) {

		let mut begin = tail;
		let end = begin.split_off(i+1);

		let (op2, pre_op, main_expr, post_op) = begin.pop().unwrap();

		return Expr::Op2(Box::new(assemble_expr(init, begin)), op2, Box::new(assemble_expr((pre_op, main_expr, post_op), end)));
	}

	// last PostOp?
	if let Some((op2, pre_op, main_expr, post_op)) = tail.pop() {
		assert!(lowest_prio == prio(post_op.as_ref()));

		tail.push((op2, pre_op, main_expr, None));
		return Expr::PostOp(Box::new(assemble_expr(init, tail)), post_op.unwrap())
	} else {
		assert!(lowest_prio == prio(init.2.as_ref()));

		return Expr::PostOp(Box::new(assemble_expr((init.0, init.1, None), tail)), init.2.unwrap());
	}
}

named!(pub parse_expr<Expr>,
	do_parse!(
		pre_op: opt!(parse_pre_op) >>
		main_expr: parse_main_expr >>
		post_op: opt!(parse_post_op) >>
		tail: many0!(
			do_parse!(
				op2: parse_op2 >>
				pre_op: opt!(parse_pre_op) >>
				main_expr: parse_main_expr >>
				post_op: opt!(parse_post_op) >>
				((op2, pre_op, main_expr, post_op))
			)
		) >>
		(assemble_expr((pre_op, main_expr, post_op), tail))
	)
);

named!(pub parse_expr_statement<AstNode>,
	do_parse!(
		expr: parse_expr >>
		tag!(";") >>
		ignore0 >>
		(AstNode::Expr(expr))
	)
);

#[test]
fn test_fun_call_expr_statement() {
	assert!(parse_expr_statement("2(2,3);".as_bytes()).unwrap().0.is_empty());
}

#[test]
fn test_fun_call_expr() {
	let (i, o)  = parse_expr("2(2,3)".as_bytes()).unwrap();
	assert!(i.is_empty());

	println!("{:?}", o);
	assert!(if let Expr::PostOp(_, PostOp::FunCall(_)) = o { true } else { false } );
}
