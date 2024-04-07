use std::collections::BinaryHeap;
use std::io;

fn kth_smallest_element(arr: &[i32], k: usize) -> Option<i32> {
    if arr.len() < k || k == 0 {
        return None; 
    }

    let mut max_heap = BinaryHeap::new();

    for &num in arr {
        if max_heap.len() < k {
            max_heap.push(num); 
        } else if num < *max_heap.peek().unwrap() {
            max_heap.pop();
            max_heap.push(num); 
        }
    }

    max_heap.peek().map(|x| *x) 
}

fn main() {
    println!("Enter the numbers");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    println!("Enter k:");
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let k: usize = input.trim().parse().expect("Invalid input");

    if let Some(kth_smallest) = kth_smallest_element(&arr, k) {
        println!(" {}", kth_smallest);
    } else {
        println!("Not possible");
    }
}
