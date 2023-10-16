fn main() {

	let mut velocity = [12,-5, 13];

	println!("Velocity: {:?}", velocity);
    opposite(&mut velocity);
	println!("Opposite Velocity:  {:?}", velocity);
}

fn opposite(v: &mut [i32]) {
    for _i in 0..v.len() {
        v[_i] = v[_i]*-1;
    }
}