fn divide(numerator: f64, denominator: f64) -> Option<f64> {
	if denominator == 0.0 {
		None
	} else {
		Some(numerator / denominator)
	}
}

fn main() {
	match divide(4.0, 2.0) {
		Some(result) => println!("Result: {}", result),
		None => println!("Cannot divide by zero"),
	}
}