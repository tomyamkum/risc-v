pub fn nand(x: bool, y: bool) -> bool {
	!(x && y)
}

pub fn not(x: bool) -> bool {
	nand(x, x)
}

pub fn and(x: bool, y: bool) -> bool {
	not(nand(x, y))
}

pub fn or(x: bool, y: bool) -> bool {
	nand(not(x), not(y))
}

pub fn mux(x: bool, y: bool, s: bool) -> bool {
	or(and(x, not(s)), and(y, s))
}

