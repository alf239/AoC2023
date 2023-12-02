use std::collections::HashMap;

use nom::{IResult, Parser};
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, digit1, space1};
use nom::multi::separated_list0;
use nom::sequence::{preceded, separated_pair};

type Round = Vec<(String, usize)>;

type Game = (usize, Vec<Round>);

fn parser(s: &str) -> IResult<&str, Game> {
    let title = preceded(tag("Game "), digit1);
    let ball_spec = separated_pair(digit1, space1, alphanumeric1);
    let round = separated_list0(tag(", "), ball_spec);
    let rounds = separated_list0(tag("; "), round);
    let mut full_spec = separated_pair(title, tag(": "), rounds);
    let (input, (nr, rs)) = full_spec.parse(s)?;

    let game_nr = nr.parse().unwrap();
    let game_def: Vec<Round> = rs.iter().map(
        |r| {
            let balls: Vec<(String, usize)> = r.iter().map(
                |(count, colour)| (colour.to_string(), count.parse().unwrap())
            ).collect();
            balls
        }).collect();
    Ok((input, (game_nr, game_def)))
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|l| parser(l).unwrap().1)
        .collect()
}

fn possible(game: &Game, budget: &HashMap<String, usize>) -> bool {
    game.1.iter().all(|round|
        round.iter().all(|(colour, req)|
            match budget.get(colour) {
                Some(b) => b >= req,
                None => false
            }
        )
    )
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Game>) -> usize {
    let budget: HashMap<String, usize> = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14)]);
    input
        .iter()
        .filter(|g| possible(g, &budget))
        .map(|g| g.0)
        .sum()
}

fn power(game: &Game) -> usize {
    let mut req: HashMap<&str, usize> = HashMap::new();
    game.1.iter().for_each(|r| r.iter().for_each(|(colour, count)| {
        let c: usize = *count;
        let mut entry = req.entry(colour).or_insert(c);
        if *entry < c { *entry = c; }
    }));
    req.values().product()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<Game>) -> usize {
    input
        .iter()
        .map(power)
        .sum()
}
