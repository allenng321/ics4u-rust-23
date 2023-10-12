fn main(){
    println!("Distance between (1,2) and (7,10): {}", distance(1.0, 2.0, 7.0, 10.0));
}

fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    // Return the correct distance value
    return ((x1-x2).powf(2.0)+(y1-y2).powf(2.0)).sqrt();
}