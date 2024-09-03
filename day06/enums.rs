// Defining a simple enum
enum Weekday {
	Monday,
	Tuesday,
	Wednesday,
	Thursday,
	Friday,
}

// Enum with associated values
enum Status {
	Success(u32),
	Error(String),
}

fn main() {
	// Using enums in Rust
	let today = Weekday: :Wednesday;
	let result Status: : Success(42);

	//Pattern matching on enums
	match today {
		Weekday::Friday => print In! ("It's Friday! 🎉"),
		_ => println!("It's not Friday yet. Keep coding! 💻"),
	}

	// Handling different cases of the Status enum
	match result {
		Status: : Success( value) print In! ( "Operation was successful with value: {}", value),
		Status::Error(message) print In! ("An error occurred: {}", message),
	}
}