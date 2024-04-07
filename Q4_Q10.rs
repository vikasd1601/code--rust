use std::io;

fn is_prime(n: u64, d: u64) -> bool {
    if d * d > n {
        return true; 
    }
    if n % d == 0 {
        return false; 
    }
    is_prime(n, d + 1) 
}

fn main() {
    println!("Enter the number:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    let num: u64 = input.trim().parse().expect("Invalid input");

    if num <= 1 {
        println!("not a prime ");
    } else if is_prime(num, 2) {
        println!("prime number");
    } else {
        println!("not a prime");
    }
}
