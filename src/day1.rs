fn digit(c: &char) -> u32 {
    c.unwrap().to_digit(10).unwrap()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let digits: Vec<char> = s.chars().filter(|c| c.is_digit(10)).collect();
            10 * digit(digits.first().unwrap()) + digit(digits.last().unwrap())
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|s| { 1 })
        .sum()
}
