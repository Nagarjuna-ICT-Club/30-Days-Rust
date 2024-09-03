fn main() {
	// Allocate an integer on the heap using the Box<T>
	let boxed_integer = Box::new(10);

	// Print the value stored in the Box<T>
	println!("Value in the box: {}", boxed_integer);

	// Access the value by dereferencing the Box<T>
	let value = *boxed_integer;
	println!("Dereferenced value: {}", value);
}