fn main() {
	let s = String::from("hello");

	// Immutable borrow
	let len = calculate_length(&s);

	// s is still valid
	println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s: &String) -> usize {
	s.len()
}