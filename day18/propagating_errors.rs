use std::fs;
use std::io;

fn read_number_from_file() -> Result<i32, io::Error> {
	let contents = fs::read_to_string("number.txt")?;
	let number = contents.trim().parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid number"))?;
	Ok(number)
}

fn main() {
	match read_number_from_file() {
		Ok(number) => print!("Number:{}", number);
		Err(e) => print!("Error reading file: {}", e);
	}
}