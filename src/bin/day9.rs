#![feature(array_chunks)]
#![feature(once_cell)]

use std::{collections::HashSet, error::Error, fs, ops::Sub};

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn adjacent(self, other: Self) -> bool {
        let vector = self - other;
        let difference = f64::sqrt((vector.x.pow(2) + vector.y.pow(2)).into());
        difference < 2.0
        // product == 0 || product == 1
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "inputs/day9";
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut head = Position { x: 0, y: 0 };
    let mut last_head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };
    let mut tail_positions: HashSet<Position> = HashSet::new();
    tail_positions.insert(tail.clone());

    for line in binding.lines() {
        dbg!(line);
        let vector = match line.split_whitespace().collect::<Vec<&str>>()[..] {
            ["R", a] => ((1, 0), a),
            ["L", a] => ((-1, 0), a),
            ["U", a] => ((0, 1), a),
            ["D", a] => ((0, -1), a),
            _ => ((0, 0), ""),
        };
        let (dir, distance) = (vector.0, vector.1.parse::<i32>()?);
        for i in 0..distance {
            head.x += dir.0;
            head.y += dir.1;
            if !head.adjacent(tail) {
                tail = last_head;
            }
            tail_positions.insert(tail.clone());
            last_head = head;
        }
        dbg!(tail, head);
    }
    println!("{:?}\n", tail_positions.len());
    Ok(())
}
