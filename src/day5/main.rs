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

    // println!("{:?}", &mappings);

    // println!("{:?}", &seeds);
    let mut results = Vec::new();
    for seed in &seeds {
        let mut value = seed.clone();
        for map in &mappings {
            // println!("Source {}", &value);
            value = map.get_value(value).clone();
            // println!("Dest {}", &value);
        }

        println!("Mapped seed {} to {}", &seed, &value);
        results.push(value);
    }

    println!("Number of mappings {:?}", mappings.len());
    println!("Min values {:?}", results.iter().min());
    /*
    for line in lines {
        println!("{:?}", line.unwrap())
    }*/

    /*
    let result: u32 = buffered
        .lines()
        .map(|l| BallCount::from_line(&l.unwrap()))
        .map(|bc| bc.red * bc.green * bc.blue)
        .sum();

    println!("{:?}", result); */
    Ok(())

    // Results are 2156 / 66909
}
