use std::f64::consts;

fn main() {
    let radius = 5.0;

    let area = consts::PI * f64::powf(radius, 2.0);

    println!("Area of the Circle: {}", area);
}