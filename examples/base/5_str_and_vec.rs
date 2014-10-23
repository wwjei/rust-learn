
fn vector_fn(){
	
	let nums_array = [1i, 2i, 3i];
	// similar to array in C, with a fixed size

	let nums_array_all_zero_with_size_20 = [0i, ..20];
	let nums_array_all_one_with_size_20 = [1i, ..20];

	let nums_vec = vec![1i, 2i, 3i];
	// similar to vector in C++

	let mut nums_vec_mut = vec![1i];	
	nums_vec_mut.push(4i);

	// similar to &str, we can get a slice by as_slice() method
	let vec = vec![1i, 2i, 3i];
	let slice = vec.as_slice();

	for i in vec.iter(){
		println!("{} in vec", i);
	}

	for i in slice.iter(){
		println!("{} in slice", i);
	}	
	
}

fn main(){
	vector_fn();
}
