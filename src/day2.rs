use std::collections::HashMap;

use aoc_parse::{parser, prelude::*};

type Round = Vec<(String, usize)>;

type Game = (usize, Vec<Round>);

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    let p = parser!(lines(
        "Game " usize ": "
        repeat_sep(
            repeat_sep(
                cnt:usize " " clr:string(alpha*) => (clr, cnt),
                ", "),
            "; ")
    ));
    p.parse(input).unwrap()
}

fn possible(game: &Game, budget: &HashMap<&str, usize>) -> bool {
    game.1.iter().all(|round|
        round.iter().all(|(colour, req)|
            match budget.get(colour.as_str()) {
                Some(b) => b >= req,
                None => false
            }
        )
    )
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Game>) -> usize {
    let budget = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    input
        .iter()
        .filter(|g| possible(g, &budget))
        .map(|g| g.0)
        .sum()
}

fn power(game: &Game) -> usize {
    let mut req = HashMap::new();
    game.1.iter().for_each(|r| r.iter().for_each(|(colour, count)| {
        let c: usize = *count;
        let entry = req.entry(colour).or_insert(c);
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
