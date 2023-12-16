use aoc_parse::{parser, prelude::*};

type Task = Vec<Vec<char>>;

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Task {
    let p = parser!(lines(alnum+));
    p.parse(input).unwrap()
}
#[aoc(day17, part1)]
fn solve_part1(input: &Task) -> usize {
    1
}

#[aoc(day17, part2)]
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
