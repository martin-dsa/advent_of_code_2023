use itertools::Itertools;
use std::{collections::HashMap, convert::Infallible, str::FromStr, thread};

advent_of_code::solution!(5);

#[derive(Debug)]
struct Map {
    source: u64,
    dest: u64,
    range: u64,
}

impl FromStr for Map {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut a = s.split_whitespace().map(|x| x.parse::<u64>().unwrap());
        Ok(Self {
            dest: a.next().unwrap(),
            source: a.next().unwrap(),
            range: a.next().unwrap(),
        })
    }
}

fn parse_seeds(input: &str) -> impl Iterator<Item = u64> + '_ {
    input
        .split(": ")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
}

fn parse_maps(input: &str) -> Vec<Map> {
    let mut lines = input.lines();
    let _title = lines.next();
    lines.map(|l| l.parse::<Map>().unwrap()).collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sections = input.split("\n\n");

    let mut seed_to_path: HashMap<u64, Vec<_>> = HashMap::new();

    let seeds = parse_seeds(sections.next().unwrap()).collect::<Vec<_>>();

    for seed in &seeds {
        seed_to_path.insert(*seed, vec![*seed]);
    }

    for section in sections {
        let maps = parse_maps(section);

        for seed in &seeds {
            let cur_loc = *seed_to_path.get(seed).unwrap().last().unwrap();

            let val = match maps
                .iter()
                .find(|x| x.source <= cur_loc && cur_loc <= x.source + x.range)
            {
                None => cur_loc,
                Some(m) => m.dest + cur_loc - m.source,
            };

            seed_to_path.get_mut(seed).unwrap().push(val);
        }
    }

    seed_to_path.iter().map(|e| *e.1.last().unwrap()).min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sections = input.split("\n\n");

    let seeds = parse_seeds(sections.next().unwrap());

    let mut handles = Vec::<std::thread::JoinHandle<u64>>::new();

    for t in seeds.tuples::<(u64, u64)>().map(|t| t.0..t.0 + t.1) {
        let maps: Vec<_> = sections.clone().map(parse_maps).collect::<Vec<_>>();

        let handle = thread::spawn(move || {
            let mut min_loc = u64::MAX;

            for seed in t {
                let mut cur_loc = seed;

                for map in &maps {
                    cur_loc = match map
                        .iter()
                        .find(|x| x.source <= cur_loc && cur_loc <= x.source + x.range)
                    {
                        None => cur_loc,
                        Some(m) => cur_loc + m.dest - m.source,
                    };
                }

                min_loc = cur_loc.min(min_loc);
            }
            min_loc
        });
        handles.push(handle);
    }

    handles.into_iter().map(|jh| jh.join().unwrap()).min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
