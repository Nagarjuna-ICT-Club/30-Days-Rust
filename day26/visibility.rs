mod outer {
	pub mod inner {
		pub fn public_function() {
			println!("This is public");
		}

		fn private_function() {
			println!("This is private");
		}
	}
}

fn main() {
	outer::inner::public_function(); // Accessible
	outer::inner::private_function(); // Not Accessible
}