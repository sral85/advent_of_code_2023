use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn get_calibration_value(s: &str) -> u32 {
    // Used for part 1
    let digits: Vec<u32> = s.chars().filter_map(|x| x.to_digit(10)).collect();
    digits.first().unwrap_or(&0) * 10 + digits.last().unwrap_or(&0)
}

fn get_number(s: &str) -> Option<u32> {
    if s.is_empty() {
        return None;
    }

    if let Some(value) = s.chars().nth(0).unwrap().to_digit(10) {
        return Some(value);
    }

    match s {
        s if s.starts_with("one") => Some(1),
        s if s.starts_with("two") => Some(2),
        s if s.starts_with("three") => Some(3),
        s if s.starts_with("four") => Some(4),
        s if s.starts_with("five") => Some(5),
        s if s.starts_with("six") => Some(6),
        s if s.starts_with("seven") => Some(7),
        s if s.starts_with("eight") => Some(8),
        s if s.starts_with("nine") => Some(9),
        _ => None,
    }
}

fn get_calibration_value2(s: &str) -> u32 {
    // Used for part 2
    let digits: Vec<u32> = (0..s.len())
        .map(|x| &s[x..])
        .filter_map(|x| get_number(x))
        .collect();

    digits.first().unwrap_or(&0) * 10 + digits.last().unwrap_or(&0)
}

fn main() -> Result<(), Error> {
    let path = "./input/day1.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let result: u32 = buffered
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| get_calibration_value(&x))
        .sum();
    println!("{:?}", result);

    Ok(())

    // Results are  54824
}
