use std:: io;
fn Shortest_word(input: &str) -> Option<&str> {
    let mut min_length = usize::MAX;
    let mut min_start_index = 0;

    let mut si = 0; // Starting index of the current word
    let mut ei = 0; // Ending index of the current word

    for (i, c) in input.chars().enumerate() {
        if c == ' ' || c == '\0' { 
            let word_length = ei - si;
            if word_length < min_length {
                min_length = word_length;
                min_start_index = si;
            }
            si = i + 1; 
        } else {
            ei = i + 1; 
        }
    }


    let word_length = ei - si;
    if word_length < min_length {
        min_length = word_length;
        min_start_index = si;
    }

    if min_length != usize::MAX {
        Some(&input[min_start_index..(min_start_index + min_length)])
    } else {
        None
    }
}

fn main() {
   println!("Enter a string of words:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");

    let input_string = input.trim();
    
    if let Some(shortest_word) = Shortest_word(input_string) {
        println!("Shortest word: {}", shortest_word);
    } else {
        println!("No words found.");
    }
}
