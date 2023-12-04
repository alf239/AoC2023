use std::collections::HashMap;

use aoc_parse::{parser, prelude::*};

pub struct Card {
    nr: u32,
    won: Vec<u32>,
    got: Vec<u32>,
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Card> {
    let p = parser!(lines(
        "Card" " "+ nr:u32 ":" " "+ won:repeat_sep(u32, " "+) " |" " "+ got:repeat_sep(u32, " "+) => Card { nr, won, got }
    ));
    p.parse(input).unwrap()
}

fn score(card: &Card) -> usize {
    card.got.iter().filter(|&nr| card.won.contains(nr)).count()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Vec<Card>) -> usize {
    input
        .iter()
        .map(|card| {
            let count = score(card);
            if count == 0 {
                0
            } else {
                1 << (count - 1)
            }
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &Vec<Card>) -> usize {
    let winning: Vec<usize> = input.iter().map(|card| score(card)).collect();
    let mut dp: Vec<usize> = winning.iter().map(|_| 1).collect();
    winning.iter().enumerate().for_each(|(i, &won)| {
        let scale = dp[i];
        if won > 0 {
            (1..=won).for_each(|j| dp[i + j] += scale)
        }
    });
    dp.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#
            .trim();
        let parsed = input_generator(input);
        let answer = solve_part1(&parsed);
        assert_eq!(answer, 13);
        let answer2 = solve_part2(&parsed);
        assert_eq!(answer2, 30);
    }
}
