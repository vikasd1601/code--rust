use std::io;

fn is_palindrome(arr: &[char]) -> bool {
    let mut start = 0;
    let mut end = arr.len() - 1;

    while start < end {
        while start < end && !arr[start].is_ascii_alphanumeric() {
            start += 1;
        }

        while start < end && !arr[end].is_ascii_alphanumeric() {
            end -= 1;
        }
        if arr[start].to_ascii_lowercase() != arr[end].to_ascii_lowercase() {
            return false;
        }
        start += 1;
        end -= 1;
    }

   return  true 
}

fn main() {
    println!("Enter a string:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed");

    let arr: Vec<char> = input.chars().collect();

    if is_palindrome(&arr) {
        println!("palindrome.");
    } else {
        println!("not a palindrome.");
    }
}
