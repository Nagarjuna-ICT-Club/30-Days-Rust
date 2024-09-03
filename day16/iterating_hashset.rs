use std::collections::HashSet;

fn main() {
	let mut colors: HashSet<&str> = HashSet::new();

	colors.insert("Red");
	colors.insert("Green");
	colors.insert("Blue");

	// interating over the hashset
	for color in colors {
		// printing each value in the hashset
		print!("{}}", color);
	}
}