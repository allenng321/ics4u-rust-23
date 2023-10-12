fn main() {
    let number = 75;
    let swap: i32;

    swap = number%10*10 + number/10;
    println!("Swapped Digit: {}", swap);
}