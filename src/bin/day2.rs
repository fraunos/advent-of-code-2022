use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "inputs/day2";
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim().split('\n');
    let mut result: i32 = 0;

    for line in contents {
        println!("{line}");
        result += match line {
            "A X" => 1 + 3,
            "B Y" => 2 + 3,
            "C Z" => 3 + 3,
            "C X" => 1 + 6,
            "A Y" => 2 + 6,
            "B Z" => 3 + 6,
            "B X" => 1,
            "C Y" => 2,
            "A Z" => 3,
            &_ => 0,
        }
    }
    println!("{result}");

    Ok(())
}
