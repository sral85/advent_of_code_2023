use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Debug)]
struct BallCount {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl BallCount {
    fn new(id: u32, red: u32, green: u32, blue: u32) -> BallCount {
        BallCount {
            id: id,
            red: red,
            green: green,
            blue: blue,
        }
    }

    fn from_line(line: &str) -> BallCount {
        let mut bc = BallCount::new(0, 0, 0, 0);
        let mut parts = line.split(|c| c == ':' || c == ',' || c == ';');

        let game = parts.next().unwrap();
        bc.id = game[5..].parse().unwrap();

        for part in parts {
            let count_data: Vec<_> = part.trim().split(' ').collect();
            let count: u32 = count_data.first().unwrap().parse().unwrap();
            let color = count_data.last().unwrap().to_owned();

            match color {
                "red" => {
                    if bc.red < count {
                        bc.red = count
                    }
                }
                "green" => {
                    if bc.green < count {
                        bc.green = count
                    }
                }
                "blue" => {
                    if bc.blue < count {
                        bc.blue = count
                    }
                }
                _ => {}
            }
        }
        bc
    }
}

fn main() -> Result<(), Error> {
    let path = "./input/day2.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    /*
        let result: u32 = buffered
        .lines()
        .map(|l| BallCount::from_line(&l.unwrap()))
        .filter(|bc| bc.red < 13 && bc.green < 14 && bc.blue < 15)
        .map(|bc| bc.id)
        .sum();
    */

    let result: u32 = buffered
        .lines()
        .map(|l| BallCount::from_line(&l.unwrap()))
        .map(|bc| bc.red * bc.green * bc.blue)
        .sum();

    println!("{:?}", result);
    Ok(())

    // Results are 2156 / 66909
}
