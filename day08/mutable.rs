fn main() {
	// Defining a mutable variable
	// Adding prefix "mut" in variable makes it mutable.

	let mut changable: i32 = 270703;
	println!("My value right now is {changable}");

	// This variable value changes
	changable = 270801;
	println!("Now my value changed into {changable}");
	return 0;
}