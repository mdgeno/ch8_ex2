fn main(){

	let str_ex = String::from("example");

	let first_char = str_ex.get(str_ex.len()-1..);

	match first_char{
		Some(val) => println!("last char {val}"),
		None => println!("char does not exist")
	}

}