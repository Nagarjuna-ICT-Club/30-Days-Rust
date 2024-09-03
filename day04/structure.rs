// Define a struct
struct Person {
	name: String,
	age: u32,
	is_programmer: bool,
}

fn main() {
	// Create an instance of the struct
	let coder = Person {
		name: String::from("JhonDoe"),
		age: 25,
		is_programmer: true,
	};

	// Access struct fields
	println!("Meet {}: Age {} - Progammer? {}", coder.name, coder.age, coder.is_programmer);
}