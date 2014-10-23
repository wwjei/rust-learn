use std::io::stdin;

fn main(){
	println!("input something");
	
	let input = stdin().read_line().ok().expect("Failed to read line");

	println!("{}", input);

}
