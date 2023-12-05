use core::panic;

use fancy_regex::Regex;

advent_of_code::solution!(1);

fn parse(s: &str) -> u32 {
    match s.parse::<u32>() {
        Ok(v) => v,
        Err(_) => match s {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => panic!(),
        },
    }
}

fn solve(input: &str, r: Regex) -> u32 {
    input
        .lines()
        .map(|line: &str| {
            let captures = r
                .captures_iter(line)
                .map(|c| c.unwrap().get(1).unwrap().as_str())
                .collect::<Vec<_>>();

            let d1 = parse(captures.first().unwrap());
            let d2 = parse(captures.last().unwrap());

            d1 * 10 + d2
        })
        .sum::<u32>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let r = Regex::new(r"(\d)").unwrap();
    Some(solve(input, r))
}

pub fn part_two(input: &str) -> Option<u32> {
    let r = Regex::new(r"(?=(\d|one|two|three|four|five|six|seven|eight|nine))").unwrap();
    Some(solve(input, r))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }
}
