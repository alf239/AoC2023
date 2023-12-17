use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};

use aoc_parse::{parser, prelude::*};

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub enum Dir {
    N,
    W,
    S,
    E,
}

type Task = Vec<Vec<usize>>;

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Task {
    let p = parser!(lines(digit+));
    p.parse(input).unwrap()
}

const B: usize = 2;

type Node = (usize, usize, Dir, usize);

fn try_schedule(
    dist: &mut HashMap<Node, i64>,
    work: &mut PriorityQueue<Node, i64>,
    m: &Vec<Vec<usize>>,
    i: usize,
    j: usize,
    dir: Dir,
    b: usize,
    hl: i64,
) {
    let x = dist.entry((i, j, dir, b)).or_insert(i64::MIN);
    let proposed_hl = hl - m[i][j] as i64;
    if *x >= proposed_hl {
        return;
    }
    *x = proposed_hl;
    work.push((i, j, dir, b), proposed_hl);
}

#[aoc(day17, part1)]
fn solve_part1(input: &Task) -> i64 {
    let h = input.len();
    let w = input[0].len();
    let mut seen = HashSet::new();
    let mut work: PriorityQueue<Node, i64> = PriorityQueue::new();
    let mut dist: HashMap<Node, i64> = HashMap::new();
    dist.insert((0, 0, Dir::S, B), 0);
    dist.insert((0, 0, Dir::E, B), 0);
    work.push((0, 0, Dir::S, B), 0);
    while !work.is_empty() {
        let ((i, j, d, b), hl) = work.pop().unwrap();
        if !seen.insert((i, j, d, b)) {
            continue;
        }
        match d {
            Dir::N => {
                if b > 0 && i > 0 {
                    try_schedule(&mut dist, &mut work, input, i - 1, j, Dir::N, b - 1, hl);
                }
                if j > 0 {
                    try_schedule(&mut dist, &mut work, input, i, j - 1, Dir::W, B, hl);
                }
                if j < w - 1 {
                    try_schedule(&mut dist, &mut work, input, i, j + 1, Dir::E, B, hl);
                }
            }
            Dir::S => {
                if b > 0 && i < w - 1 {
                    try_schedule(&mut dist, &mut work, input, i + 1, j, Dir::S, b - 1, hl);
                }
                if j > 0 {
                    try_schedule(&mut dist, &mut work, input, i, j - 1, Dir::W, B, hl);
                }
                if j < w - 1 {
                    try_schedule(&mut dist, &mut work, input, i, j + 1, Dir::E, B, hl);
                }
            }
            Dir::W => {
                if b > 0 && j > 0 {
                    try_schedule(&mut dist, &mut work, input, i, j - 1, Dir::W, b - 1, hl);
                }
                if i > 0 {
                    try_schedule(&mut dist, &mut work, input, i - 1, j, Dir::N, B, hl);
                }
                if i < h - 1 {
                    try_schedule(&mut dist, &mut work, input, i + 1, j, Dir::S, B, hl);
                }
            }
            Dir::E => {
                if b > 0 && j < w - 1 {
                    try_schedule(&mut dist, &mut work, input, i, j + 1, Dir::E, b - 1, hl);
                }
                if i > 0 {
                    try_schedule(&mut dist, &mut work, input, i - 1, j, Dir::N, B, hl);
                }
                if i < h - 1 {
                    try_schedule(&mut dist, &mut work, input, i + 1, j, Dir::S, B, hl);
                }
            }
        }
    }
    let mut hl = i64::MIN;
    for d in [Dir::N, Dir::E] {
        for b in 0..=3 {
            match dist.get(&(h - 1, w - 1, d, b)) {
                Some(&hl1) => hl = hl.max(hl1),
                None => {}
            }
        }
    }
    -hl
}

#[aoc(day17, part2)]
fn solve_part2(input: &Task) -> usize {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 102);
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 2);
    }
}
