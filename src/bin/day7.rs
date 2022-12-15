// i've been thinking about some recursive solution, but got stuck
// solution looked up from https://github.com/fogleman/AdventOfCode2022/blob/main/07.py
#![feature(iter_array_chunks)]
use std::{collections::HashMap, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "inputs/day7";
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut cwd: Vec<&str> = vec![];
    let mut files: Vec<(Vec<&str>, i32)> = vec![];
    let mut dir_sizes_map: HashMap<String, i32> = HashMap::new();
    const MAX_DIR_SIZE: i32 = 100000;

    // Part 1
    for input_output in binding.split('$').map(|x| x.lines()) {
        let mut command_output_line = input_output
            .clone()
            .map(|x| x.trim().split_whitespace().collect::<Vec<&str>>());
        let command = match command_output_line.next() {
            Some(command) => command,
            None => vec![],
        };

        println!("{}> {}", cwd.join("/"), command.join(" "));
        match command[..] {
            [] => println!("Empty"),
            ["cd", ".."] => {
                cwd.pop();
                ()
            }
            ["cd", "/"] => cwd.push(""),
            ["cd", dir] => cwd.push(dir),
            ["ls"] => {
                for item in command_output_line {
                    println!("{:?}", item);
                    if let Some(size) = item[0].parse::<i32>().ok() {
                        let mut dir_path = cwd.clone();
                        dir_path.push(item[1]);
                        let clone = dir_path.clone();
                        files.push((clone.clone(), size));
                        for i in 1..dir_path.len() {
                            let partial_path =
                                clone.iter().take(i).map(|x| *x).collect::<Vec<&str>>();
                            println!("{:?}", partial_path);
                            println!("{:?}", dir_sizes_map.entry(partial_path.join("/")));
                            *dir_sizes_map.entry(partial_path.join("/")).or_insert(0) += size;
                            println!("{:?}", dir_sizes_map.entry(partial_path.join("/")));
                        }
                    }
                }
            }
            _ => todo!("Not implemented"),
        }
        println!();
    }

    println!("{:#?}", dir_sizes_map);
    println!(
        "Part1 {:?}",
        dir_sizes_map
            .iter()
            .filter(|x| x.1 <= &MAX_DIR_SIZE)
            .map(|x| x.1)
            .sum::<i32>()
    );
    let mut dir_sizes = dir_sizes_map.iter().map(|x| *x.1).collect::<Vec<i32>>();
    let root_size = dir_sizes_map.get("").unwrap();
    dir_sizes.sort();
    let max_fs_size: i32 = 70000000;
    println!(
        "Part2 {:?}",
        dir_sizes
            .iter()
            .find(|x| **x > 30000000 - (max_fs_size - root_size))
            .unwrap()
    );
    Ok(())
}
