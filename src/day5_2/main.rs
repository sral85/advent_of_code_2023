use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

#[derive(Clone, Debug)]
struct RangeMap {
    ranges: Vec<[u64; 3]>,
}

impl RangeMap {
    fn new() -> RangeMap {
        RangeMap { ranges: Vec::new() }
    }

    fn insert_range(&mut self, dest: u64, source: u64, gap: u64) {
        self.ranges.push([dest, source, gap]);
    }

    fn get_value(&self, value: u64) -> u64 {
        for range in &self.ranges {
            if range[1] <= value && value < range[1] + range[2] {
                // println!("{}, {}, {}, {}", value, range[0], range[1], range[2]);
                return (value - range[1]) + range[0];
            }
        }
        return value;
    }
}

fn main() -> Result<(), Error> {
    let path = "./input/day5.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut lines = buffered.lines();

    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .filter_map(|s| s.parse::<u64>().ok())
        .collect();

    let mut mappings: Vec<RangeMap> = Vec::new();

    let mut range_map: RangeMap = RangeMap::new();
    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        if line.contains("map") {
            mappings.push(range_map.clone());
            println!("Create new map: {:?}", &line);
            range_map = RangeMap::new();
            continue;
        }
        let values: Vec<u64> = line
            .split(' ')
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();

        range_map.insert_range(values[0], values[1], values[2]);
    }
    mappings.push(range_map.clone());

    let mut minimum = u64::MAX;
    for chunk in seeds.chunks(2) {
        println!("New chunk {:?}", &chunk);
        for seed in chunk[0]..(chunk[0] + chunk[1]) {
            let mut value = seed.clone();
            for map in &mappings {
                value = map.get_value(value).clone();
            }
            minimum = min(minimum, value);
        }
    }
    println!("Minimum is {}", minimum);
    // Result is 69323688
    Ok(())
}
