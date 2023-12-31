use aoc_parse::{parser, prelude::*};

const CARDS: &str = "AKQJT98765432";
const CARDS2: &str = "AKQT98765432J";
const N: usize = CARDS.len();

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Copy, Clone)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn rank_for(top_two: (usize, usize)) -> HandRank {
    match top_two {
        (5, _) => HandRank::FiveOfAKind,
        (4, _) => HandRank::FourOfAKind,
        (3, 2) => HandRank::FullHouse,
        (3, _) => HandRank::ThreeOfAKind,
        (2, 2) => HandRank::TwoPair,
        (2, _) => HandRank::OnePair,
        _ => HandRank::HighCard,
    }
}

fn card_rank(cards: &str, card: char) -> usize {
    cards
        .chars()
        .rev()
        .enumerate()
        .find_map(|(i, c)| if c == card { Some(i) } else { None })
        .unwrap()
}

fn card_counts(cards: &str, hand: &str) -> ([usize; 13], u64) {
    let mut counts: [usize; N] = [0; N];
    let mut value: u64 = 0;
    for c in hand.chars() {
        let card = card_rank(cards, c);
        counts[card] += 1;
        value = value * N as u64 + card as u64;
    }
    (counts, value)
}

fn top_two(mut counts: [usize; 13]) -> (usize, usize) {
    counts.sort_unstable();
    let x1 = counts[counts.len() - 1];
    let x2 = counts[counts.len() - 2];
    (x1, x2)
}

fn hand_rank(hand: &str) -> (HandRank, u64) {
    let (counts, value) = card_counts(CARDS, hand);
    (rank_for(top_two(counts)), value)
}

fn hand_rank2(hand: &str) -> (HandRank, u64) {
    let (mut counts, value) = card_counts(CARDS2, hand);
    let j = counts[0];
    counts[0] = 0;
    let (x1, x2) = top_two(counts);
    (rank_for((x1 + j, x2)), value)
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<(String, u64)> {
    let p = parser!(lines(
        hand:string(alnum+) " "+ stake:u64 => (hand, stake)
    ));
    p.parse(input).unwrap()
}

fn solve(input: &Vec<(String, u64)>, score: fn(&str) -> (HandRank, u64)) -> u64 {
    let mut work: Vec<((HandRank, u64), u64)> = input
        .iter()
        .map(|(hand, stake)| (score(hand), *stake))
        .collect();
    work.sort_by_key(|(rank, _)| *rank);

    work.iter()
        .enumerate()
        .map(|(i, (_, stake))| stake * (1 + i as u64))
        .sum()
}

#[aoc(day7, part1)]
fn solve_part1(input: &Vec<(String, u64)>) -> u64 {
    solve(input, hand_rank)
}

#[aoc(day7, part2)]
fn solve_part2(input: &Vec<(String, u64)>) -> u64 {
    solve(input, hand_rank2)
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
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 5905);
    }
}
