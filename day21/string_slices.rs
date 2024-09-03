fn main() {
	let s = String::from("hello world");

	let hello = &s[0..3]; // Slice from index 0 to 3
	let world = &s[6..10]; // Slice from index 6 to 10

	println!("{} {}", hello, world);
}