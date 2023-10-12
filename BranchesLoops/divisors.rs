fn main() {
    let number = 12;

    print!("Divisors of {}: ", number);

    for i in 1..(12+1) {

        // Is the remainder of (number / i) equal to zero?
        if number % i == 0 {
            print!("{} ",i);
        }
    }
}