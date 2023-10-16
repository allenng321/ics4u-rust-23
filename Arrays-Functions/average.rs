fn main() {

	let marks = [75, 82, 90, 95, 87, 80, 70, 92];

	println!("{:?}", marks);
	println!("Average:  {:0.1}", average(marks));
}

fn average(m: [i32;8]) -> f64 {
    let mut sum = 0.0;
	for _i in 0..(m.len()) {
		sum += (m[_i] as f64);
	}
	sum = sum / (m.len() as f64);
	return sum;
}