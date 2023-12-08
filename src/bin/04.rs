advent_of_code::solution!(4);
use std::collections::{HashMap, HashSet};

use fancy_regex::{Match, Regex};
use lazy_static::lazy_static;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^Card\s+\d+: ([^|]+) \| ([^|]+)$").unwrap();
}

fn c_to_set(m: Option<Match>) -> HashSet<u32> {
    m.unwrap()
        .as_str()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<HashSet<u32>>()
}

fn get_winning_quantity(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.lines().flat_map(|line| {
        RE.captures_iter(line).map(|c| {
            let c = c.unwrap();
            let winning_numbers = c_to_set(c.get(1));
            let numbers = c_to_set(c.get(2));

            winning_numbers.intersection(&numbers).count() as u32
        })
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let res = get_winning_quantity(input)
        .map(|n| if n == 0 { 0 } else { 2u32.pow(n - 1) })
        .sum::<u32>();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut card_quantities: HashMap<usize, u32> = HashMap::new();
    for (idx, _) in input.lines().enumerate() {
        card_quantities.insert(idx, 1);
    }

    for (row, qty) in get_winning_quantity(input).enumerate() {
        for next_row in row + 1..row + 1 + qty as usize {
            *(card_quantities.get_mut(&(next_row)).unwrap()) +=
                *card_quantities.get(&(row)).unwrap();
        }
    }

    let res = card_quantities.iter().map(|x| x.1).sum::<u32>();
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
