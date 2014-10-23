

fn tuple_base(){
	// tuple
	let t1 = (1i, 2i, 3i);
	let t2: (int, int, int) = (1i, 2i, 3i);
	
	let (x, y, z) = (1i, 2i, 3i);
	println!("x,y,z is {},{},{}\n", x, y, z);

	let t1_eq_t2 = (t1 == t2);
	println!("t1 is {}\nt2 is {}", t1, t2);
	println!("t1==t2? {}", t1_eq_t2);

}

fn tuple_func(){
	let (x, y) = next_two(5i);
	println!("\nnext two of 5i: ({})", (x, y));
}

fn next_two(x: int) -> (int, int) { (x + 1i, x + 2i) }

fn point_(){
	let p1 = Point{x: 0i, y: 1i};
	let p2 = Point2{x: 1i, y: 2i};

	let p = Point{x: 0i, y: 1i};
	// println!("\nprint point: {}", p);
	// cannot be compiled
	println!("\npoint: x:{}, y:{}", p.x, p.y);

	//let eq_ = (p1 == p2);
	// error: binary operation `==` cannot be applied to type `Point`
}

struct Point {
	x: int,
	y: int
}

struct Point2 {
	x: int,
	y: int
}



fn tuple_struct(){
	let c1 = Color(1i, 2i, 3i);
}


// enum
enum Ordering{
	Less,
	Equal,
	Greater,
}

fn cmp(a: int, b: int) -> Ordering{
	if a < b { Less }
	else if a > b { Greater }
	else { Equal }
}


fn do_cmp(){
	
	println!("cmp 1, 2: {}", cmp_result_str(cmp(1i, 2i)));
	println!("cmp 1, 1: {}", cmp_result_str(cmp(1i, 1i)));
	println!("cmp -1, 100: {}", cmp_result_str(cmp(-1i, 100i)));
}

fn cmp_result_str(r: Ordering) -> String{
	match (r) {
		Less => "less".to_string(),
		Greater => "greater".to_string(),
		Equal => "equal".to_string(),
	}
}


struct Color(int, int, int);
struct Color2{
	red: int,
	blue: int,
	green: int
}

fn main(){
	tuple_base();
	
	tuple_func();
	
	point_();
	tuple_struct();
	
	do_cmp();
}

