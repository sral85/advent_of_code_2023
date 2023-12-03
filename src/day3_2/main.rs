use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn collect_number(idx: usize, v: &Vec<char>) -> Option<u32> {
    if idx >= v.len() {
        return None;
    }

    if !v[idx].is_digit(10) {
        return None;
    };

    // Collect left bound
    let mut left_idx = idx;
    while 0 < left_idx && v[left_idx - 1].is_digit(10) {
        left_idx -= 1;
    }

    // Collect right bound
    let mut right_idx = idx;
    while right_idx + 1 < v.len() && v[right_idx+1].is_digit(10) {
        right_idx += 1;
    }
    Some(
        v[left_idx..=right_idx]
            .iter()
            .fold(0, |x, c| 10 * x + c.to_digit(10).unwrap()),
    )
}

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

    let num_rows = engine.len();
    let num_cols = engine[0].len();
    let mut map = HashMap::new(); // to each position (=row/col) each adjacent number is stored

    // Iterate over rows and columns and find all numbers adjacent to *
    for (row_idx, line) in engine.iter().enumerate() {
        for (col_idx, &c) in line.iter().enumerate() {
            if c != '*' {
                continue;
            }
            let iter = DIRECTIONS
                .iter()
                .map(|(x, y)| (row_idx as isize + x, col_idx as isize + y))
                .filter(|(x, y)| {
                    0 <= *x && 0 <= *y && *x < (num_rows as isize) && *y < (num_cols as isize)
                })
                .map(|(x, y)| (usize::try_from(x).unwrap(), usize::try_from(y).unwrap()))
                .filter(|(x, y)| engine[*x][*y].is_digit(10));
            for (x, y) in iter {
                if let Some(number) = collect_number(y, &engine[x]) {
                    map.entry((row_idx, col_idx)).or_insert(HashSet::new()).insert(number);
                }
            }
        }
    }

    let result: u32 = map.into_iter()
        .filter(|(_key, val)| val.len() == 2)
        .map(|(_key, val)| val.iter().product::<u32>())
        .sum();

    println!("{:?}", result);
    
    Ok(())

}
