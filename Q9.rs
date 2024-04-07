use std::io;

fn reverse_string(input: &mut String) {
    let mut start = 0;
    let mut end = input.len() - 1;
    let input_chars = unsafe { input.as_mut_vec() };

    while start < end {
        input_chars.swap(start, end);
        start += 1;
        end -= 1;
    }
}

fn main() {
    println!("Enter a string:");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read line");

    reverse_string(&mut input_string);
    println!("Reversed string: {}", input_string);
}
