use std::thread;

fn main() {
	let handle = thread::spawn(|| {
		// Code executed in a new thread
		println!("Hello from a thread!");
	});

	// Wait for the thread to finish
	handle.join().unwrap();
	println!("Hello from the main thread!");
}