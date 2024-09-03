fn main() {
	let some_value = Some(10);
	let no_value: Option<i32> = None;

	match some_value {
		Some(v) => print!("Found a value: {}", v),
		None => print!("No value found"),
	}

	match no_value {
		Some(v) => print!("Found a value: {}", v),
		None => print!("No value found"),
	}
}