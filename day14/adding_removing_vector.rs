fn main() {
	let mut numbers = vec![1,2,3,4,5];
	numbers.push(6);
	println!("Numbers: {:?}", numbers);
	// numbers = [1,2,3,4,5,6]

	numbers.remove(2);
	println!("Numbers: {:?}", numbers);
	// numbers = [1,3,4,5,6]
}