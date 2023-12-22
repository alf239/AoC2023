use aoc_parse::{parser, prelude::*};

type Task = Vec<String>;

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Task {
    let p = parser!(lines(string(alpha+)));
    p.parse(input).unwrap()
}

#[aoc(day22, part1)]
fn solve_part1(input: &Task) -> u64 {
    1
}

#[aoc(day22, part2)]
fn solve_part2(input: &Task) -> u64 {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#""#.trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 1);
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 2);
    }
}
