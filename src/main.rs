use std::io;

fn main(){

	println!("Enter a text:");
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("enter correct input");

	let first_char = match input.get(0){
				Some(val) => val,
				None(_) => _
	};
	
	println!("return 1 or 2? {}", check(first_char));

//	let first_char = input.remove(0);

}

//????
fn check(letter: char) -> i32{
	match letter{
		a => 2,
		e => 2,	
		i => 2,
		o => 2,
 		u => 2,
		_ => 1
	}
}