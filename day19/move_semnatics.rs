fn main() {
	let s1 = String::from("hello");
	let s2 = s1;
	//s1 is moved to s2 and is no longer valid

	println!("{}", s1); // this will cause a error
	println!("{}", s2); // this will print "hello"
}