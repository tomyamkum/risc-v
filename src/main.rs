use risc_v::gate::*;

fn main() {
	let x:bool = true;
	let y:bool = true;
	println!("{}", or(x,y));
}
