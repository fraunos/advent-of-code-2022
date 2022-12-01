use std::{error::Error, fs, num::ParseIntError};

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "inputs/day1";
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim();
    let split = contents.split("\n\n");

    let mut sums = split
        .map(|calories_list| {
            calories_list
                .split('\n')
                .map(|x| x.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();
    sums.sort();
    println!("{:?}", sums.iter().rev().take(3).sum::<i32>());

    // for elf_calories_list in split {
    //     println!("{}\n", elf_calories_list);
    //     let sum = elf_calories_list
    //         .split('\n')
    //         .map(|x| x.parse::<i32>().unwrap())
    //         .sum::<i32>();
    //     println!("{}\n", sum);
    //     if sum > maxValue {
    //         maxValue = sum;
    //         maxValueIndex = currentIndex;
    //     }
    //     currentIndex += 1;
    // }
    // println!("{maxValueIndex} {maxValue}");

    Ok(())
}
