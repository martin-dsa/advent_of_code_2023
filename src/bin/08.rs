use fancy_regex::Regex;
use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    convert::Infallible,
    str::{Chars, FromStr},
};

advent_of_code::solution!(8);
lazy_static! {
    static ref RE: Regex = Regex::new(r"(.*) = \((.*), (.*)\)").unwrap();
}

#[derive(Debug)]
struct Node {
    from: String,
    left: String,
    right: String,
}

impl FromStr for Node {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = RE.captures(s).unwrap().unwrap();
        let from = captures.get(1).unwrap().as_str().to_string();
        let left = captures.get(2).unwrap().as_str().to_string();
        let right = captures.get(3).unwrap().as_str().to_string();

        Ok(Self { from, left, right })
    }
}

const START: &str = "AAA";
const END: &str = "ZZZ";

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn solve<'a>(
    path_str: &'a str,
    mut path: Chars<'a>,
    network: &'a HashMap<String, (String, String)>,
    mut current_node: (&'a String, &'a (String, String)),
    end: &str,
) -> u64 {
    let mut steps = 0;

    while !current_node.0.ends_with(end) {
        let mut dir = path.next();

        if dir.is_none() {
            path = path_str.chars();
            dir = path.next();
        }

        let dir: char = dir.unwrap();

        let next_node = match dir {
            'L' => &current_node.1 .0,
            'R' => &current_node.1 .1,
            _ => panic!(),
        };

        current_node = (next_node, network.get(next_node).unwrap());

        steps += 1;
    }
    steps
}

pub fn part_one(input: &str) -> Option<u64> {
    let (path_str, network) = input.split_once("\n\n").unwrap();

    let path = path_str.chars();

    let network = network
        .lines()
        .map(|l| l.parse::<Node>().unwrap())
        .map(|n| (n.from, (n.left, n.right)))
        .collect::<HashMap<_, _>>();

    let start_node: (&String, &(String, String)) =
        (&START.to_string(), network.get(START).unwrap());

    let steps = solve(path_str, path, &network, start_node, END);
    Some(steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (path_str, network) = input.split_once("\n\n").unwrap();
    let path = path_str.chars();
    let network = network
        .lines()
        .map(|l| l.parse::<Node>().unwrap())
        .map(|n| (n.from, (n.left, n.right)))
        .collect::<HashMap<_, _>>();

    let current_nodes = network
        .iter()
        .filter(|x| x.0.ends_with('A'))
        .collect::<Vec<_>>();

    let steps = current_nodes
        .iter()
        .map(|n| solve(path_str, path.clone(), &network, *n, "Z"))
        .reduce(|a, b| lcm(a as usize, b as usize) as u64)
        .unwrap();

    Some(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
