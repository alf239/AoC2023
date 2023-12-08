use aoc_parse::{parser, prelude::*};

pub struct Task {
    input: String,
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Task {
    // let p = parser!(

    // );
    // p.parse(input).unwrap()
    Task {
        input: input.to_string(),
    }
}

#[aoc(day9, part1)]
fn solve_part1(input: &Task) -> usize {
    1
}

#[aoc(day9, part2)]
fn solve_part2(input: &Task) -> usize {
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
