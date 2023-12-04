use std::cmp::min;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Debug)]
struct ScratchCard {
    id: String,
    count: usize,
    value: usize,
}

impl ScratchCard {
    fn new(id: String, count: usize, value: usize) -> ScratchCard {
        ScratchCard { id, count, value }
    }

    fn from_line(line: &str) -> ScratchCard {
        let mut parts = line.split(|c| c == ':' || c == '|');

        let id = parts.next().unwrap().trim().to_owned();

        let winners: HashSet<usize> = parts
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        let yours = parts
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .filter_map(|s| s.parse().ok())
            .collect();
        let value = winners.intersection(&yours).count();

        ScratchCard::new(id, 1, value)
    }
}

fn main() -> Result<(), Error> {
    let path = "./input/day4.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut scores: Vec<ScratchCard> = buffered
        .lines()
        .map(|l| ScratchCard::from_line(&l.unwrap()))
        .collect();

    for idx in 0..scores.len() {
        let start = idx + 1;
        let end = min(idx + scores[idx].value, scores.len() - 1);

        for update_idx in start..=end {
            scores[update_idx].count += scores[idx].count
        }
    }

    let result: usize = scores.iter()
    .map(|sc| sc.count)
    .sum();

    println!("{:?}", result);
    Ok(())

    // Results are
}
