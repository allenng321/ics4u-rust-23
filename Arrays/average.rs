fn main() {

	let marks = [75, 82, 90, 95, 87, 80, 70, 92];
	let mut sum = 0;

	// Simple array output - You can comment this out
	// println!("{:?}", marks);

	for _i in 0..marks.len() {
		// Update the accumulator
		sum += marks[_i];
	}

	print!("Marks:  ");
	for _i in 0..marks.len() {
		print!("{} ", marks[_i]);
	}
	

	// Correct this calculation
	let average = (sum as f64) / (marks.len() as f64);
	println!("\nAverage:  {:0.1}", average);
}