use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::iter::Iterator;

#[derive(Debug)]
struct RaceData {
    races: Vec<[u32; 2]>,
}

impl RaceData {
    fn new(races: Vec<[u32; 2]>) -> RaceData {
        RaceData { races }
    }

    fn from_vec(times: Vec<u32>, distances: Vec<u32>) -> RaceData {
        let races = times
            .iter()
            .zip(distances.iter())
            .map(|(x, y)| [*x, *y])
            .collect();
        RaceData::new(races)
    }
}

fn get_count_of_race(t: u32, d: u32) -> u32 {
    (0..=t).filter(|time| (t - time) * time > d).count() as u32
}

fn main() -> Result<(), Error> {
    let path = "./input/day6_short.txt";

    let input = File::open(path)?;
    let mut lines = BufReader::new(input).lines();

    let times: Vec<_> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    let distances: Vec<_> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    let rd = RaceData::from_vec(times, distances);

    let result: Vec<_> = rd
        .races
        .iter()
        .map(|[t, d]| get_count_of_race(*t, *d))
        .collect();
    println!("{:?}", result.iter().product::<u32>());
    Ok(())
}
// Results are 1413720
