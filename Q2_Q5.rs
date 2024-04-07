use std::io;

//Ques 5

fn median(arr: &[i32]) -> f32 {
    let len = arr.len();
    //odd
    if len % 2 == 1 {
       return  arr[len / 2] as f32
    } else {  //even
        let r = arr[len / 2];
        let l = arr[len / 2 - 1];
        return (l as f32 + r as f32) / 2.0
    }
}

//Ques 5

fn first_occurrence(arr: &[i32], target: i32) -> i32 {
    let mut low = 0;
    let mut high = arr.len() as i32 - 1;
    let mut result = -1;

    while low <= high {
        let mid = low + (high - low) / 2;

        if arr[mid as usize] == target {
            result = mid;
            high = mid - 1; 
        } else if arr[mid as usize] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    return result
}

fn main() {
    println!("Enter the sorted array:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");

    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid"))
        .collect();

    println!("Enter the target value:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    
    let target: i32 = input.trim().parse().expect("Invalid");

    let index = first_occurrence(&arr, target);
    if index != -1 {
        println!("index {}",index);
    } else {
        println!("not found index");
    }
    
    println!("Median:{}" ,median(&arr) )
}
