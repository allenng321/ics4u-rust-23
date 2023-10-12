fn main(){
    println!("GCD of 48 and 36: {}", gcd(48, 36));
}

fn gcd(m: u32, n: u32) -> u32 {
    // Return the correct greatest common divisor
    // between m and n
    if m%n == 0 {return n;}
    else {
        return gcd(n, m%n);
    }
}