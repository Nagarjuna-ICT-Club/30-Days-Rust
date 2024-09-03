// const: An unchangeable value (the common case)
const GRAVITY_ACCELERATION: f64 = 9.81;

// static: A possibly mutable variable with 'static lifetime.'
static mut COUNTER:u32 = 0;

fn main() {
	// Utilize const in a calculation
	let time_to_fall_10_meters = (2.0 * 10.0 / GRAVITY_ACCELERATION).sqrt();
	
	println!("It takes {:.2} seconds", time_to_fall_10_meters);
}