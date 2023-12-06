use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::iter::Iterator;

#[derive(Debug)]
struct RaceData {
    races: Vec<[usize; 2]>,
}

impl RaceData {
    fn new(races: Vec<[usize; 2]>) -> RaceData {
        RaceData { races }
    }

    fn from_vec(times: Vec<usize>, distances: Vec<usize>) -> RaceData {
        let races = times
            .iter()
            .zip(distances.iter())
            .map(|(x, y)| [*x, *y])
            .collect();
        RaceData::new(races)
    }
}

fn get_count_of_race(t: usize, d: usize) -> usize {
    (0..=t).filter(|time| (t - time) * time > d).count()
}

fn main() -> Result<(), Error> {
    let path = "./input/day6.txt";

    let input = File::open(path)?;
    let mut lines = BufReader::new(input).lines();

    let times: Vec<_> = lines
        .next()
        .unwrap()
        .unwrap()
        .replace(" ", "")
        .split(':')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    let distances: Vec<_> = lines
        .next()
        .unwrap()
        .unwrap()
        .replace(" ", "")
        .split(':')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect();

    println!("{:?}", &times);
    println!("{:?}", &distances);
    let rd = RaceData::from_vec(times, distances);
    

    let result: Vec<_> = rd
        .races
        .iter()
        .map(|[t, d]| get_count_of_race(*t, *d))
        .collect();
    println!("{:?}", result.iter().product::<usize>());
    Ok(())

    // Results are 1413720
}
