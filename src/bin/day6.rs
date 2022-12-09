#![feature(iter_array_chunks)]
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "inputs/day6";
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("{}", binding);
    let chars = binding.chars().collect::<Vec<char>>();
    // Part 1
    for i in 0..=chars.len() {
        let mut marker = chars.iter().skip(i).take(4).collect::<Vec<&char>>();
        marker.sort();
        marker.dedup();
        if marker.len() == 4 {
            println!("{:?}", marker);
            println!("Found {}", i + 4);
            break;
        }
    }
    // Part 2
    for i in 0..=chars.len() {
        let mut marker = chars.iter().skip(i).take(14).collect::<Vec<&char>>();
        marker.sort();
        marker.dedup();
        if marker.len() == 14 {
            println!("{:?}", marker);
            println!("Found {}", i + 14);
            break;
        }
    }
    Ok(())
}
