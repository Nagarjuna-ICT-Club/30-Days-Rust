fn main() {

	let colors = vec!["blue", "red", "green"];

	// Using vector index
	println!("First color = {}", colors[0]);

	// Using get() method. Returns a Some("red")
	println!("First color = {:?}", colors.get(1));
}