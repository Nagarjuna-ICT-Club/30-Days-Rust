fn main() {
	// Scalars
	let integer_num: i32 = 42;
	let float_num: f64 = 3.14;
	let is_rust_cool: bool = true;
	let fav_char: char = 'R';

	// Compounds
	let array_nums: [i32; 3] = [1, 2, 3];
	let tuple_data: (char, i32, bool) = ('A', 42, false);

	println!("Scalars: Int-{}", integer_num);
	println!("Compounds: Array-{:?}", array_nums);	
}