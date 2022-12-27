// okaaay this taught me a bit, the code was getting ugly really fast
// and I had to refactor it over and over
// had to write some visualising code to understand and debug
#![feature(array_chunks)]
#![feature(once_cell)]

use std::sync::LazyLock;
use std::{env, io};
use std::{error::Error, fs};

enum Axis {
    Horizontal,
    Vertical,
}
#[derive(Debug, Clone, Copy)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}
impl Direction {
    fn axis(&self) -> Axis {
        match *self {
            Direction::Top => Axis::Vertical,
            Direction::Bottom => Axis::Vertical,
            Direction::Left => Axis::Horizontal,
            Direction::Right => Axis::Horizontal,
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct Visibility {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

#[derive(Debug, Clone, Copy)]
enum ProcessingColorCode {
    Red = 31,
    Blue = 34,
    None = 0,
}

#[derive(Debug, Clone, Copy)]
struct Tree {
    height: isize,
    visible: Visibility,
    processing: ProcessingColorCode,
    scenic_score: i32,
}

static DEBUG: LazyLock<bool> = LazyLock::new(|| match env::args().nth(1) {
    Some(a) => "--debug" == a,
    _ => false,
});
fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "inputs/day8";
    let binding = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut forest: Vec<Vec<Tree>> = binding
        .lines()
        .map(|line| {
            line.chars()
                .map(|tree_height| Tree {
                    height: tree_height.to_digit(10).unwrap() as isize,
                    visible: Visibility {
                        top: false,
                        bottom: false,
                        left: false,
                        right: false,
                    },
                    processing: ProcessingColorCode::None,
                    scenic_score: 1,
                })
                .collect::<Vec<Tree>>()
        })
        .collect::<Vec<Vec<Tree>>>();
    let forest_len_range = 0..forest.len();
    // scan_forest_side(
    //     &mut forest,
    //     Direction::Bottom,
    //     &forest_len_range.clone().collect::<Vec<usize>>(),
    //     &forest_len_range.clone().rev().collect::<Vec<usize>>(),
    // );
    // scan_forest_side(
    //     &mut forest,
    //     Direction::Top,
    //     &forest_len_range.clone().collect::<Vec<usize>>(),
    //     &forest_len_range.clone().collect::<Vec<usize>>(),
    // );
    // scan_forest_side(
    //     &mut forest,
    //     Direction::Right,
    //     &forest_len_range.clone().collect::<Vec<usize>>(),
    //     &forest_len_range.clone().rev().collect::<Vec<usize>>(),
    // );
    // scan_forest_side(
    //     &mut forest,
    //     Direction::Left,
    //     &forest_len_range.clone().collect::<Vec<usize>>(),
    //     &forest_len_range.clone().collect::<Vec<usize>>(),
    // );

    scan_forest_tree_views(&mut forest);

    print_forest(&forest.clone());

    println!(
        "{:?}",
        forest
            .iter()
            .flatten()
            .map(
                |x| if x.visible.top | x.visible.bottom | x.visible.left | x.visible.right {
                    1
                } else {
                    0
                }
            )
            .sum::<i32>()
    );
    println!(
        "{:?}",
        forest.iter().flatten().map(|x| x.scenic_score).max()
    );

    Ok(())
}
fn print_forest(forest: &Vec<Vec<Tree>>) {
    let test = forest.iter().map(|treeline| {
        treeline
            .iter()
            // .map(|tree| {
            //     if tree.processing {
            //         "."
            //     } else if tree.visible.top
            //         | tree.visible.bottom
            //         | tree.visible.left
            //         | tree.visible.right
            //     {
            //         "1"
            //     } else {
            //         "0"
            //     }
            // })
            .map(|tree| {
                format!(
                    "\u{1b}[{}m{}\u{001b}[0m",
                    tree.processing as usize, tree.height
                )
            })
            .collect::<Vec<String>>()
            .join::<&str>("")
    });

    println!("{}\n", test.collect::<Vec<String>>().join("\n"));
}

fn scan_forest_tree_views(forest: &mut Vec<Vec<Tree>>) {
    for x in 0..forest.len() {
        for y in 0..forest.len() {
            forest[x][y].processing = ProcessingColorCode::Red;
            println!("START {:?}", forest[x][y]);
            println!("TREE POS {x}, {y}");
            for (dx, dy) in [(1, 0), (0, 1), (0, -1), (-1, 0)] {
                let mut counter = 1;
                loop {
                    let (xx, yy) = (x as i32 + dx * counter, y as i32 + dy * counter);
                    println!("\nVIEW POS {xx}, {yy}, counter: {counter}");
                    if *DEBUG {
                        io::stdin().read_line(&mut String::new());
                    }
                    if x == 0 || y == 0 || x == forest.len() || y == forest.len() {
                        println!("Instant border");
                        forest[x][y].scenic_score = 0;
                        println!("Current total: {}", forest[x][y].scenic_score);
                        break;
                    }
                    if xx <= 0
                        || xx >= forest.len() as i32 - 1
                        || yy <= 0
                        || yy >= forest.len() as i32 - 1
                    {
                        forest[x][y].scenic_score *= counter;

                        println!("Border! View score: {counter}");
                        println!("Current total: {}", forest[x][y].scenic_score);
                        break;
                    }
                    forest[xx as usize][yy as usize].processing = ProcessingColorCode::Blue;
                    if *DEBUG {
                        print_forest(&forest.clone());
                    }
                    if forest[xx as usize][yy as usize].height >= forest[x][y].height {
                        forest[x][y].scenic_score *= counter;
                        forest[xx as usize][yy as usize].processing = ProcessingColorCode::None;
                        println!("A tree! View score: {counter}");
                        println!("Current total: {}", forest[x][y].scenic_score);
                        break;
                    }
                    counter += 1;
                    forest[xx as usize][yy as usize].processing = ProcessingColorCode::None;
                }
            }
            println!("FINISH {:?}", forest[x][y]);
            forest[x][y].processing = ProcessingColorCode::None;
            println!();
        }
    }
}

fn scan_forest_side(
    forest: &mut Vec<Vec<Tree>>,
    dir: Direction,
    x_range: &Vec<usize>,
    y_range: &Vec<usize>,
) {
    let mut answer = String::new();
    println!("{:?}", dir);
    // println!("{:?}", x_range);

    for x in x_range {
        let mut top_tree_height = -1;
        for y in y_range {
            let current_tree = match dir.axis() {
                Axis::Horizontal => &mut forest[*x][*y],
                Axis::Vertical => &mut forest[*y][*x],
            };
            // println!(
            //     "tree height: {:?}, top height: {:?}",
            //     current_tree.height, top_tree_height
            // );
            // println!("{x} {y}");
            current_tree.processing = ProcessingColorCode::Blue;
            if current_tree.height > top_tree_height || top_tree_height == -1 {
                // println!("Visible!");
                match dir {
                    Direction::Top => current_tree.visible.top = true,
                    Direction::Bottom => current_tree.visible.bottom = true,
                    Direction::Left => current_tree.visible.left = true,
                    Direction::Right => current_tree.visible.right = true,
                }
                top_tree_height = current_tree.height;
            };

            if *DEBUG {
                print_forest(&forest.clone());
            }
            match dir.axis() {
                Axis::Horizontal => forest[*x][*y].processing = ProcessingColorCode::None,
                Axis::Vertical => forest[*y][*x].processing = ProcessingColorCode::None,
            };
            if *DEBUG {
                // io::stdin().read_line(&mut answer);
            }
        }
    }
}
