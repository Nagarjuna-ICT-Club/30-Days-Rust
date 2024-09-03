fn main() {
	{
		let s = String::from("hello");
		// s is valid from this point forward

		println!("{}", s);
	}
	// this scope is now over, and s is no longer valid
}