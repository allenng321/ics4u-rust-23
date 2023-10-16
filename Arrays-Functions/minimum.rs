fn main() {

	let marks = [75, 82, 90, 95, 87, 80, 70, 92];

	println!("{:?}", marks);
	println!("Minimum:  {}", minimum(marks));
}

fn minimum(m: [i32;8]) -> i32 {
    let mut min = i32::MAX;
    for _i in 0..(m.len()) {
        if min > m[_i] { min = m[_i];}
    }
    return min;
}
