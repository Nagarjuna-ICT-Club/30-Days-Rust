// Defining a function
fn hello() {
	println!("Hello from the other side.");
}

fn get_pi() -> f64 {
	22.0 / 7.0 // Returning the value to main function
}

fn main() {
	// Calling the hello function
	hello();

	// Calling the get_pi function
	println!("PI value is {}", get_pi());
}