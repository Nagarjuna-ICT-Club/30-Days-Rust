//Creating the macro

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn hello_world_macro(_input: TokenStream) -> TokenStream {
	let generated_code = quote::quote! {
		fn hello_world() {
			println!("Hello, world!");
		}
	};

	generated_code.into()
}


// Using the created macro

use simple_macro::hello_world_macro;

hello_world_macro!();

fn main() {
	hello_world();
}