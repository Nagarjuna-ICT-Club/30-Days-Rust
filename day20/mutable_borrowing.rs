fn main() {
	let mut s = String::from("hello");

	//Mutable Borrow
	change(&mut s);

	//s is still valid and modified
	println!("{}", s);
}

fn change(s: &mut String) {
	s.push_str(", world");
}