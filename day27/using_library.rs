/* Cargo.toml
[dependencies]
serde = "1.0"
serde_json = "1.0"
*/

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Person {
	name: String,
	age: u8,
}

fn main() {
	let person = Person {
		name: "Alice".to_string(),
		age: 30,
	};

	let json = serde_json::to_string(&person).unwrap();
	println!("Serialized: {}", json);

	let deserialized: Person = serde_json::from_str(&json).unwrap();
	println!("Deserialized:  {} is {} years old", deserialized.name, deserialized.age);
}