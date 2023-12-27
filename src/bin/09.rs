advent_of_code::solution!(9);

fn get_interpolated_value(
    diff_fn: impl Fn(&[i64]) -> i64,
    interpolated_val_fn: impl Fn(&Vec<i64>) -> i64,
) -> impl Fn(Vec<i64>) -> i64 {
    move |seq: Vec<i64>| {
        let mut history = vec![seq];
        while !history.last().unwrap().iter().all(|x| *x == 0) {
            let next_seq = history
                .last()
                .unwrap()
                .windows(2)
                .map(&diff_fn)
                .collect::<Vec<_>>();
            history.push(next_seq);
        }

        history.iter().map(&interpolated_val_fn).sum()
    }
}

fn solve(input: &str, qwe: impl Fn(Vec<i64>) -> i64) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(qwe)
        .sum::<i64>()
}
pub fn part_one(input: &str) -> Option<i64> {
    let res = solve(
        input,
        get_interpolated_value(|x| (x[1] - x[0]), |s| *s.last().unwrap()),
    );
    Some(res)
}

pub fn part_two(input: &str) -> Option<i64> {
    let res = solve(
        input,
        get_interpolated_value(|x| (x[0] - x[1]), |s| *s.first().unwrap()),
    );
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
