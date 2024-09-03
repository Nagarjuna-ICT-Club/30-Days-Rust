fn main() {
	{
		let r;				//'r' does not have a value yet
		{
			let x = 5;		//'x' is created here
			r = &x;			// 'r' borrows 'x'
		}					// 'x' goes out of scope here
		println!("{}", r);	// ERROR: 'r' is now a dangling reference
	}
}