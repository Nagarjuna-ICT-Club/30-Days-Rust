use std::collections::HashMap;

fn main() {
	let mut map = HashMap::new();
	map.insert("a", 1);
	map.insert("b", 2);
	map.insert("c", 3);
	println!("{:?}", map);
	
	for (key, value) in map.iter() {
		println!("Key: {}, Value: {}", key, value);
	}
}