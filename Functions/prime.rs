fn main(){
    println!("Is 15 prime? {}", is_prime(15));
}

fn is_prime(num: u32) -> bool {
    // Return the correct boolean value: true or false
    // depending upon whether the variable num is a prime number
    for i in 2..num/2 {
        if num % i == 0 {return false;}
    }
    return true;
}