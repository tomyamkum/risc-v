use crate::gate::*;

pub fn one_bit_alu(x: bool, y: bool, flag: bool) -> bool {
	mux(and(x, y), or(x, y), flag)
}


