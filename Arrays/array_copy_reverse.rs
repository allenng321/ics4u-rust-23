fn main() {

	let values = [2, 8, 5, 10];
	let mut reverse_values = [0; 4];

    
	for i in 0..values.len() {
		reverse_values[values.len()-1-i] = values[i];
	}

	for i in 0..reverse_values.len() {
        print!("{} ", reverse_values[i]);
    }
	
}