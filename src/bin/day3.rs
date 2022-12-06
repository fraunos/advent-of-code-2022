#![feature(iter_array_chunks)]
use std::{collections::HashMap, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "inputs/day3";
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim().lines().map(String::from);
    let mut result1 = 0;
    let mut result2 = 0;

    for line in contents.clone() {
        let (first_part, second_part) =
            (&line[..line.len() / 2], &line[line.len() / 2..line.len()]);
        let shared_character = find_shared_character(first_part, second_part);

        // println!("{line}");
        // println!("{first_part}");
        // println!("{second_part}");
        // println!("{shared_character}");
        // println!("{}", shared_character as u32);
        // println!("{}", get_shared_character_value(shared_character).unwrap());
        result1 += get_shared_character_value(shared_character).unwrap();
        // println!();
    }

    for elven_group in contents.array_chunks::<3>() {
        let mut hashmap: HashMap<char, usize> = HashMap::new();
        // println!("{:?}", elven_group);
        let mut group = elven_group.map(|x| String::from(x).chars().collect::<Vec<char>>());
        group.iter_mut().for_each(|x| x.sort());
        group.iter_mut().for_each(|x| x.dedup());
        let test = group.iter().flatten().collect::<Vec<&char>>();

        for char in test {
            match hashmap.get(char) {
                Some(value) => hashmap.insert(*char, value + 1),
                None => hashmap.insert(*char, 1),
            };
        }

        let group_key_char = hashmap.iter().find(|(_, count)| **count == 3).unwrap().0;
        result2 += get_shared_character_value(*group_key_char).unwrap();
    }

    println!("Result 1: {result1}");
    println!("Result 2: {result2}");
    Ok(())
}

fn find_shared_character(first_text: &str, second_text: &str) -> char {
    let mut text = String::from(first_text);
    text.retain(|c| second_text.contains(c));
    text.chars().next().unwrap()
}

fn get_shared_character_value(shared_character: char) -> Option<u32> {
    if shared_character.is_lowercase() {
        return Some(shared_character as u32 - 'a' as u32 + 1);
    }
    if shared_character.is_uppercase() {
        return Some(shared_character as u32 - 'A' as u32 + 27);
    }
    None
}
