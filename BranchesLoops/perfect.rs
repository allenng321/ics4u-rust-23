fn main() {
    let number = 12;
    let mut sum = 0;

    for i in 1..number {

    // Is the remainder of (number / i) equal to zero?
        if number % i == 0 {
          sum = sum + i;
        }
        
    }

    if sum == number {
        println!("The number {} is a perfect number.", number);
    }
    else {
      println!("The number {} is NOT a perfect number.", number);
    }
}