struct ImportantExcerpt<'a> {
	part: &'a str,
}

fn main() {
	let novel = String::from("Because when I made Stella mine. I would do it so...");

	let first_sentence = novel.split('.').next().expect("Could not find a '.'");

	let i = ImportantExcerpt{
		part: first_sentence,
	};
}