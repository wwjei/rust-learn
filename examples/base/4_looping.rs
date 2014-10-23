fn main(){
	let mut x = 1u;
	let mut done = false;

	println!("while:")
	while !done {
		x += 1;
		println!("x increased to {}", x);
		if x > 5 { done = true; }
	}

	println!("\n");

	println!("loop:");
	loop{
		x += 1;
		println!("x increased to {}", x);
		if x > 10 { break; }
	}

	
	println!("for");
	for x in range(0i, 10i){
		if x % 2 == 0 { continue; }
		println!("{:d}", x);
	}

}
