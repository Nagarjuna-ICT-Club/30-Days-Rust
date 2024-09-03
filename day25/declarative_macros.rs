macro_rules! repeat {
	($val:expr, $times:expr) => {
		for _ in 0..$times {
			println!("{}", $val);
		}
	};
}

fn main() {
	repeat!("Hello",3);
}