use std::io;

fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let first_string = &strings[0]; 

    for (i, c) in first_string.chars().enumerate() {
        for string in strings.iter().skip(1) {
            if let Some(ch) = string.chars().nth(i) {
                if ch != c {
                    return first_string[..i].to_string(); 
                }
            } else {
                return first_string[..i].to_string(); 
            }
        }
    }

    first_string.to_string() 
}

fn main() {
    println!("Enter strings :");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");

    let strings: Vec<String> = input.trim().split_whitespace().map(String::from).collect();
    
    let common_prefix = longest_common_prefix(&strings);
    println!("LCP: {}", common_prefix);
}
