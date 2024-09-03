// Create a new project
cargo new my_project
cd my_project

// Tree of the directory created
my_project
|__ Cargo.toml
|__ src
	|__ main.rs

// Building the project
cargo build // Creates a binary output

// Running the project
cargo run // Creates and runs the binary output


//File Configuration file for rust
// Cargo.toml

[package]
name = "my_project"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2021"

[dependencies]
