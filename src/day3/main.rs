use std::fs::File;
use std::io::{BufRead, BufReader, Error};

const SYMBOLS: [char; 4] = ['+', '#', '*', '$'];
const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn main() -> Result<(), Error> {
    let path = "./input/day3.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    // Convert input into matrix like object
    let engine: Vec<_> = buffered
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();
    println!("{:?}", engine);

    let num_rows = engine.len();

    let mut numbers: Vec<u32> = Vec::new();
    for (row_idx, line) in engine.iter().enumerate() {
        let num_cols = line.len();
        let mut number: Option<u32> = None;
        let mut is_valid = false;
        for (col_idx, c) in line.iter().enumerate() {
            let update_valid = DIRECTIONS
                .into_iter()
                .map(|(x, y)| (row_idx as i32 + x, col_idx as i32 + y))
                .filter(|(x, y)| {
                    &0 <= x && &0 <= y && x < &(num_cols as i32) && y < &(num_rows as i32)
                })
                .map(|(x, y)| (usize::try_from(x).unwrap(), usize::try_from(y).unwrap()))
                .any(|(x, y)| engine[x][y] != '.' && !engine[x][y].is_digit(10));
            println!("{}, {}, {}", row_idx, col_idx, update_valid);

            match (c.is_digit(10), number) {
                (false, None) => {}
                (false, Some(value)) => {
                    if is_valid {
                        numbers.push(value);
                    };
                    number = None;
                    is_valid = false;
                }
                (true, None) => {
                    number = Some(c.to_digit(10).unwrap());
                    is_valid |= update_valid;
                }
                (true, Some(value)) => {
                    number = Some(value * 10 + c.to_digit(10).unwrap());
                    is_valid |= update_valid;
                }
            };
        }
        if let Some(value) = number {
            if is_valid {
                numbers.push(value);
            };
        }
        //is_valid = false;
    }
    println!("{:?}", &numbers);
    let result: u32 = numbers.iter().sum();
    println!("{:?}", &result);

    /*
        let result: u32 = buffered
        .lines()
        .map(|l| BallCount::from_line(&l.unwrap()))
        .filter(|bc| bc.red < 13 && bc.green < 14 && bc.blue < 15)
        .map(|bc| bc.id)
        .sum();


        let engine: u32 = buffered
        .lines()
        .map(|l| BallCount::from_line(&l.unwrap()))
        .map(|bc| bc.red * bc.green * bc.blue)
        .sum();

        println!("{:?}", engine);
    */
    Ok(())

    // Results are 2156 / 66909
}
