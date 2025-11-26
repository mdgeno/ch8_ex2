use std::io;

fn main(){

	println!("Enter a text:");
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("enter correct input");

	for word in input.split_whitespace(){

		match word.get(0..1){
				Some(val) => check(val, word.trim()),
				None => println!("no value")
		};

	}

}

fn check(letter: &str, word: &str){
	match letter.trim(){
		"a" => run_vow(word),
		"e" => run_vow(word),	
		"i" => run_vow(word),
		"o" => run_vow(word),
 		"u" => run_vow(word),
		 _ => run_cons(word)
	}
}

fn run_cons(word: &str){
	let mut text = String::from(word);
	let first_letter = text.remove(0);
	let output = format!("{text}-{first_letter}ay");
	print!("{output} ");
}

fn run_vow(word: &str){
	let output = format!("{word}-hay");
	print!("{output} ");
}