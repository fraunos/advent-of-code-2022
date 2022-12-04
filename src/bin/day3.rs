use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "inputs/day3";
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim().split('\n');
    let mut result = 0;

    for line in contents {
        let (first_part, second_part) =
            (&line[..line.len() / 2], &line[line.len() / 2..line.len()]);
        let shared_character = find_shared_character(first_part, second_part);

        println!("{line}");
        println!("{first_part}");
        println!("{second_part}");
        println!("{shared_character}");
        println!("{}", shared_character as u32);
        println!("{}", get_shared_character_value(shared_character).unwrap());
        result += get_shared_character_value(shared_character).unwrap();
        println!();
    }
    println!("{result}");
    Ok(())
}

fn find_shared_character<'a>(first_text: &'a str, second_text: &'a str) -> char {
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
