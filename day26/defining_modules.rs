mod greetings {
	pub fn hello() {
		println!("Hello, world!");
	}

	fn goodbye() {
		println!("This is the end.");
	}
}

fn main() {
	greetings::hello(); // Hello, world!
	greetings::goodbye(); // This is the end.
}