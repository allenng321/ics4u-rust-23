fn main() {

	let values = [40, 25, 15, 70, 60];
	let mut max = values[0];
	let mut min = values[0];

	for i in 0..values.len() {
		if values[i] > max {max = values[i];}
		if values[i] < min {min = values[i];}
	}

	println!("Maximum:  {}", max);
	println!("Minimum:  {}", min);
}