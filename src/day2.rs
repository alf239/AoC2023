use std::{cmp::max, collections::HashMap};

use aoc_parse::{parser, prelude::*};

pub struct Ball {
    colour: String,
    count: usize,
}

pub struct Game {
    nr: usize,
    rounds: Vec<Vec<Ball>>,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    let p = parser!(lines(
        "Game " nr:usize ": "
        rounds:repeat_sep(
            balls:repeat_sep(
                count:usize " " colour:string(alpha*) => Ball { colour, count },
                ", "),
            "; ") => Game { nr, rounds }
    ));
    p.parse(input).unwrap()
}

fn possible(game: &Game, budget: &HashMap<&str, usize>) -> bool {
    game.rounds.iter().all(|round| {
        round
            .iter()
            .all(|ball| match budget.get(ball.colour.as_str()) {
                Some(&b) => b >= ball.count,
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
    for r in game.rounds.iter() {
        for ball in r.iter() {
            let entry = req.entry(&ball.colour).or_insert(ball.count);
            *entry = max(*entry, ball.count);
        }
    }
    req.values().product()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<Game>) -> usize {
    input.iter().map(power).sum()
}
