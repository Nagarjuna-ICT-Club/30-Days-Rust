fn main() {
	let mut x = 0;
	loop { // indefinite loop
		x += 1;
		println!("x is {}", x);

		if x == 13 {
			break;  // break is required to exit out of loop
		}
	}
}