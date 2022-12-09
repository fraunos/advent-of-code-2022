#![feature(iter_array_chunks)]
use regex::Regex;
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "inputs/day5";
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let [initial_state, move_instructions] = match binding.split("\n\n").collect::<Vec<_>>()[..] {
        [a, b] => [a, b],
        _ => ["", ""],
    };

    let mut columns: Vec<Vec<char>> = vec![];
    columns.push(vec![]);

    for line in initial_state.lines().rev().skip(1) {
        println!("{:?}", line);
        let chars = line.chars();
        let cargo = chars
            .enumerate()
            .filter(|(index, _)| (index) % 4 == 1)
            .map(|(_, letter)| letter);

        for (index, letter) in cargo.enumerate() {
            match columns.get_mut(index) {
                Some(column) => {
                    if letter.is_alphabetic() {
                        column.push(letter)
                    }
                }
                None => columns.push(vec![letter]),
            }
        }
    }

    // Part 1
    let mut columns_part1 = columns.clone();
    println!("{:?}", columns_part1);
    let re: Regex = Regex::new(r"\d+").unwrap();
    for line in move_instructions.lines() {
        println!("{line}");
        let [times, from, to] = match re
            .find_iter(line)
            .map(|mat| mat.as_str().parse::<usize>().unwrap())
            .collect::<Vec<usize>>()[..]
        {
            [a, b, c] => [a, b - 1, c - 1],
            _ => [0, 0, 0],
        };

        for _ in 1..=times {
            let popped = columns_part1[from].pop().unwrap();
            columns_part1[to].push(popped);
        }
        println!();
    }

    println!(
        "{:?}",
        columns_part1
            .iter()
            .map(|x| x.last().unwrap())
            .collect::<String>()
    );

    // Part 2
    let mut columns_part2 = columns.clone();
    println!("{:?}", columns_part2);
    let re: Regex = Regex::new(r"\d+").unwrap();
    for line in move_instructions.lines() {
        println!("{line}");
        let [times, from, to] = match re
            .find_iter(line)
            .map(|mat| mat.as_str().parse::<usize>().unwrap())
            .collect::<Vec<usize>>()[..]
        {
            [a, b, c] => [a, b - 1, c - 1],
            _ => [0, 0, 0],
        };

        println!("{:?}", columns_part2);
        let column_from = columns_part2.get(from).unwrap().clone();
        let column_to = columns_part2.get(to).unwrap().clone();
        let split = column_from.split_at(column_from.len() - times);
        println!("{:?}", split);

        columns_part2[from] = split.clone().0.to_vec();
        columns_part2[to] = [column_to, split.1.to_vec()].concat();
        println!("{:?}", columns_part2);
        println!();
    }

    println!(
        "{:?}",
        columns_part2
            .iter()
            .map(|x| x.last().unwrap())
            .collect::<String>()
    );

    Ok(())
}
