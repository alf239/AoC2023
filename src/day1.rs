use std::ops::Index;

fn digit(c: &char) -> u32 {
    c.to_digit(10).unwrap()
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

const DIGITS: &'static [&'static str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
];

#[aoc(day1, part2)]
pub fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let mut digits: Vec<(usize, u32)> = s.chars().enumerate()
                .filter(|(_, c)| c.is_digit(10))
                .map(|(i, c)| (i, c.to_digit(10).unwrap()))
                .collect();
            let mut words: Vec<(usize, u32)> = DIGITS.iter().enumerate().flat_map(|(d, w)| {
                let l = s.find(w);
                let r = s.rfind(w);
                let digit = d as u32;
                [(l, digit), (r, digit)]
            }).filter_map(|(p, d)| p.map(|x| (x, d))).collect();
            digits.append(&mut words);
            digits.sort();
            let (_, f) = digits.first().unwrap();
            let (_, l) = digits.last().unwrap();
            10 * f + l
        })
        .sum()
}
