pub mod greetings {
	pub fn greet(name: &str) {
		println!("Hello, {}!", name);
	}
}

pub mod farewell {
	pub fn say_goodbye(name: &str) {
		println!("Goodbye, {}!", name);
	}
}