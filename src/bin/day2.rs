use std::{collections::HashMap, error::Error, fs};

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
enum HandGesturesEnum {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
enum GameResultEnum {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

#[derive(Debug)]
struct HandGesture {
    is: HandGesturesEnum,
    wins_with: HandGesturesEnum,
    loses_with: HandGesturesEnum,
}

// struct HandGestureGame<'a> {
//     allowed_gestures: &'a [HandGesture],
// }

// impl<'a> HandGestureGame<'a> {
//     // fn parse_
//     // fn
// }

// This ended up pretty verbose...

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "inputs/day2";
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = binding.trim().split('\n');
    let mut result: i32 = 0;

    let mut scoring = HashMap::new();
    scoring.insert(
        HandGesturesEnum::Rock,
        HandGesture {
            is: HandGesturesEnum::Rock,
            wins_with: HandGesturesEnum::Scissors,
            loses_with: HandGesturesEnum::Paper,
        },
    );
    scoring.insert(
        HandGesturesEnum::Paper,
        HandGesture {
            is: HandGesturesEnum::Paper,
            wins_with: HandGesturesEnum::Rock,
            loses_with: HandGesturesEnum::Scissors,
        },
    );
    scoring.insert(
        HandGesturesEnum::Scissors,
        HandGesture {
            is: HandGesturesEnum::Scissors,
            wins_with: HandGesturesEnum::Paper,
            loses_with: HandGesturesEnum::Rock,
        },
    );

    for line in contents {
        let mut split = line.split(' ');
        let opponent_move = match split.next() {
            Some("A") => Some(HandGesturesEnum::Rock),
            Some("B") => Some(HandGesturesEnum::Paper),
            Some("C") => Some(HandGesturesEnum::Scissors),
            Some(&_) => None,
            None => None,
        }
        .unwrap();

        let desired_outcome = match split.next() {
            Some("X") => Some(GameResultEnum::Lose),
            Some("Y") => Some(GameResultEnum::Draw),
            Some("Z") => Some(GameResultEnum::Win),
            Some(&_) => None,
            None => None,
        }
        .unwrap();

        println!("{line}");
        println!(
            "Opponent move: {:?},  Desired outcome: {:?}",
            opponent_move, desired_outcome
        );

        let our_move = match desired_outcome {
            GameResultEnum::Win => scoring.get(&opponent_move).unwrap().loses_with,
            GameResultEnum::Draw => scoring.get(&opponent_move).unwrap().is,
            GameResultEnum::Lose => scoring.get(&opponent_move).unwrap().wins_with,
        };

        println!("Our move: {:?}", our_move);

        let round_result = desired_outcome as i32 + our_move as i32;
        println!("Round result: {round_result}\n");

        result += round_result;

        // result += match line {
        //     "A X" => 1 + 3,
        //     "B Y" => 2 + 3,
        //     "C Z" => 3 + 3,
        //     "C X" => 1 + 6,
        //     "A Y" => 2 + 6,
        //     "B Z" => 3 + 6,
        //     "B X" => 1,
        //     "C Y" => 2,
        //     "A Z" => 3,
        //     &_ => 0,
        // }
    }
    println!("{result}");

    Ok(())
}
