use std::collections::HashMap;

use aoc_parse::{parser, prelude::*};

pub struct Round {
    balls: Vec<(String, usize)>,
}

pub struct Game {
    nr: usize,
    rounds: Vec<Round>,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    let p = parser!(lines(
        "Game " nr:usize ": "
        rounds:repeat_sep(
            balls:repeat_sep(
                cnt:usize " " clr:string(alpha*) => (clr, cnt),
                ", ") => Round { balls},
            "; ") => Game { nr, rounds }
    ));
    p.parse(input).unwrap()
}

fn possible(game: &Game, budget: &HashMap<&str, usize>) -> bool {
    game.rounds.iter().all(|round| {
        round
            .balls
            .iter()
            .all(|(colour, req)| match budget.get(colour.as_str()) {
                Some(b) => b >= req,
                None => false,
            })
    })
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Game>) -> usize {
    let budget = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    input
        .iter()
        .filter(|&g| possible(g, &budget))
        .map(|g| g.nr)
        .sum()
}

fn power(game: &Game) -> usize {
    let mut req = HashMap::new();
    game.rounds.iter().for_each(|r| {
        r.balls.iter().for_each(|(colour, count)| {
            let c: usize = *count;
            let entry = req.entry(colour).or_insert(c);
            if *entry < c {
                *entry = c;
            }
        })
    });
    req.values().product()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<Game>) -> usize {
    input.iter().map(power).sum()
}
