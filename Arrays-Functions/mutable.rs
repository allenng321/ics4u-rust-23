fn main(){
    let mut my_array = [2, 4, 6, 8];
    println!("{:?}", my_array);
    my_array = double(my_array);
    println!("{:?}", my_array);
}

fn double(mut a: [i32;4]) -> [i32;4] {
    for i in 0..a.len(){
        a[i] = 2*a[i];
    }
    return a;
}