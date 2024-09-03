use std::collections::HashSet;

fn main() {
	// initializing a hashset
	let mut colors: HashSet<&str> = HashSet::new();

	// inserting values in the hashset
	colors.insert("Red");
	colors.insert("Green");
	colors.insert("Blue");

	println!("Color = {:?}", colors);
}