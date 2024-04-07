use std::io;

fn max_subarray_sum(nums: &[i32]) -> i32 {
    let mut max_sum = std::i32::MIN;
    let mut current_sum = 0;

    for &num in nums {
        current_sum = current_sum + num;
        if num > current_sum {
            current_sum = num;
        }
        if current_sum > max_sum {
            max_sum = current_sum;
        }
    }

    max_sum
}

fn main() {
    println!("Enter the array");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid input"))
        .collect();

    let max_sum = max_subarray_sum(&nums);
    println!("Maximum subarray sum: {}", max_sum);
}
