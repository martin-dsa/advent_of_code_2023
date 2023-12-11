use itertools::Itertools;

advent_of_code::solution!(6);

fn solve((time, dist): (u64, u64)) -> usize {
    (0..time)
        .filter(move |delay| {
            let speed = delay;
            let time_left = time - delay;
            let distance = speed * time_left;
            dist < distance
        })
        .count()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (times, distances) = input
        .split_once('\n')
        .map(|line| {
            (
                line.0
                    .split_whitespace()
                    .skip(1)
                    .map(|s| s.parse::<u64>().unwrap()),
                line.1
                    .split_whitespace()
                    .skip(1)
                    .map(|s| s.parse::<u64>().unwrap()),
            )
        })
        .unwrap();

    let res = times.zip(distances).map(solve).product::<usize>();

    Some(res as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = input
        .split_once('\n')
        .map(|line| {
            (
                line.0
                    .split_whitespace()
                    .skip(1)
                    .join("")
                    .parse::<u64>()
                    .unwrap(),
                line.1
                    .split_whitespace()
                    .skip(1)
                    .join("")
                    .parse::<u64>()
                    .unwrap(),
            )
        })
        .unwrap();

    let result = solve(data);

    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
