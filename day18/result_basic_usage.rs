fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
	if denominator == 0.0 {
		Err(String::from("Division by zero"))
	}else {
		Ok(numerator/ denominator)
	}
}


fn main() {
	match divide(4.0, 2.0) {
		Ok(result) => println!("Result: {}", result),
		Err(e) => println!("Error: {}", e),
	}

	match divide(4.0, 0.0) {
		Ok(result) => println("Result: {}", result),
		Err(e) => println("Error: {}", e),
	}
}