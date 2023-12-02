use std::ops::Index;

#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();
            10 * digits.first().unwrap() + digits.last().unwrap()
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
                .filter_map(|(i, c)| c.to_digit(10).map(|d| (i, d)))
                .collect();
            let mut words: Vec<(usize, u32)> = DIGITS.iter().enumerate()
                .flat_map(|(d, w)| [(s.find(w), d), (s.rfind(w), d)])
                .filter_map(|(p, d)| p.map(|x| (x, d as u32)))
                .collect();
            digits.append(&mut words);
            digits.sort();
            let (_, f) = digits.first().unwrap();
            let (_, l) = digits.last().unwrap();
            10 * f + l
        })
        .sum()
}
