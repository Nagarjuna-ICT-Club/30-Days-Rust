use std::collections::HashSet;

fn main() {
	let mut colors: HashSet<&str> = HashSet::new();

	colors.insert("Red");
	colors.insert("Green");
	colors.insert("Blue");

	println!("Color = {:?}", colors);

	//Removing a element from the hashset
	colors.remove("Blue");
	print!("Color = {:?}", colors);
}