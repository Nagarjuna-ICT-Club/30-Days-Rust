fn main() {
	// Converting a string literal(&str) into object
	let mut literal = "Hello Rustaceans".to_string();
	println!("Object: {}", literal);

	// Appending new string slice
	literal.push_str(". Hope you had a good day!!");
	println!("Appended string: {}", literal);
}}