#![feature(iter_array_chunks)]
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "inputs/day7";
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut cwd: Vec<&str> = vec![];
    let mut total_size: i64 = 0;
    const MAX_DIR_SIZE: i32 = 100000;

    // Part 1
    for input_output in binding.split('$').map(|x| x.lines()) {
        let mut split = input_output
            .clone()
            .map(|x| x.trim().split_whitespace().collect::<Vec<&str>>());
        let command = match split.next() {
            Some(command) => command,
            None => vec![],
        };
        println!("{:?}", command);
        println!("{:?}", cwd);
        match command[..] {
            [] => println!("Empty"),
            ["cd", ".."] => {
                cwd.pop();
                ()
            }
            ["cd", dir] => cwd.push(dir),
            ["ls"] => {
                for item in split.clone() {
                    println!("{:?}", item);
                }
                let sum: i32 = split.filter_map(|x| x[0].parse::<i32>().ok()).sum();
                if sum >= MAX_DIR_SIZE {
                    total_size += sum as i64;
                };
                println!("{sum}");
                println!();
                // for item in split {
                //     println!("{:?}", item);
                // }
            }
            _ => todo!("Not implemented"),
        }
    }
    println!("{}", total_size);
    Ok(())
}
