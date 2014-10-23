fn add_one(x: int) -> int {
	x + 1
}

// or { return x+1; }

fn sum(x:int, y:int) -> int{
	return x + y;
}

fn main(){
	let a = 1i;
	let b = 2i;
	println!("sum of {} and {} is: {}", a, b, sum(a, b));
	println!("{} + 1 = {}", a, add_one(a));
}
