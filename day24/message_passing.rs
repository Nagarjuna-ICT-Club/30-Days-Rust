use std::sync::mpsc;
use std:thread;

fn main() {
	let (tx, rx) = mpsc::channel();

	thread::spawn(move || {
		tx.send("Hello from another thread!").unwrap();
	});

	let message = rx.recv().unwrap();
	println!("{}", message);
}