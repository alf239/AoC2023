use aoc_parse::{parser, prelude::*};

const CARDS: &str = "AKQJT98765432";
const N: usize = CARDS.len();

fn card_rank(card: char) -> usize {
    CARDS
        .char_indices()
        .find_map(|(i, c)| if c == card { Some(i) } else { None })
        .unwrap()
}

fn hand_rank(hand: &str) -> (u32, u64) {
    let mut counts: [usize; N] = [0; N];
    let mut value: u64 = 0;
    for c in hand.chars() {
        let card = N - card_rank(c) - 1;
        counts[card] += 1;
        value = value * N as u64 + card as u64;
    }
    counts.sort_unstable();
    let x1 = counts[counts.len() - 1];
    let x2 = counts[counts.len() - 2];
    let wide_rank = if x1 == 5 {
        7
    } else if x1 == 4 {
        6
    } else if x1 == 3 && x2 == 2 {
        5
    } else if x1 == 3 {
        4
    } else if x1 == 2 && x2 == 2 {
        3
    } else if x1 == 2 {
        2
    } else {
        1
    };
    (wide_rank, value)
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<(String, u64)> {
    let p = parser!(lines(
        hand:string(alnum+) " "+ stake:u64 => (hand, stake)
    ));
    p.parse(input).unwrap()
}

#[aoc(day7, part1)]
fn solve_part1(input: &Vec<(String, u64)>) -> u64 {
    let mut work: Vec<((u32, u64), u64)> = input
        .iter()
        .map(|(hand, stake)| (hand_rank(hand), *stake))
        .collect();
    work.sort_by_key(|(rank, _)| *rank);

    work.iter()
        .enumerate()
        .map(|(i, (_, stake))| stake * (1 + i as u64))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 6440);
    }
}
