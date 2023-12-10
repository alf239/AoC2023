use aoc_parse::{parser, prelude::*};

type Task = String;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Task {
    // let p = parser!(seqs:lines(repeat_sep(i32, " ")) => Task { seqs });
    // p.parse(input).unwrap()
    input.to_string()
}

#[aoc(day10, part1)]
fn solve_part1(input: &Task) -> i32 {
    1
}

#[aoc(day10, part2)]
fn solve_part2(input: &Task) -> i32 {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#""#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 1);
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 2);
    }
}
