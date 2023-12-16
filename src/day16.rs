use std::collections::{HashMap, HashSet, VecDeque};
use rayon::prelude::*;

use aoc_parse::{parser, prelude::*};

type Coords = (i32, i32);

#[derive(Clone, Copy)]
pub enum Tile {
    Hor,
    Ver,
    Fwd,
    Bck,
}

pub struct Task {
    h: i32,
    w: i32,
    m: HashMap<Coords, Tile>,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Task {
    use Tile::*;
    let p = parser!(
        lines({
            "." => None,
            "/" => Some(Fwd),
            "\\" => Some(Bck),
            "|" => Some(Ver),
            "-" => Some(Hor),
        }+)
    );
    let tiles = p.parse(input).unwrap();
    let m: HashMap<Coords, Tile> = tiles
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, c)| c.map(|x| ((i as i32, j as i32), x)))
        })
        .collect();
    Task {
        h: tiles.len() as i32,
        w: tiles[0].len() as i32,
        m,
    }
}

type Particle = (Coords, Coords);

fn eval(input: &Task, seed: Particle) -> usize {
    let mut seen: HashSet<Particle> = HashSet::new();
    let mut work: VecDeque<Particle> = VecDeque::from([seed]);
    while !work.is_empty() {
        let p @ (pos @ (i, j), (di, dj)) = work.pop_front().unwrap();
        if i < 0 || j < 0 || i >= input.h || j >= input.w {
            continue;
        }
        if !seen.insert(p) {
            continue;
        }
        match input.m.get(&pos) {
            None => {
                work.push_back(((i + di, j + dj), (di, dj)));
            }
            Some(Tile::Hor) if di == 0 => {
                work.push_back(((i + di, j + dj), (di, dj)));
            }
            Some(Tile::Ver) if dj == 0 => {
                work.push_back(((i + di, j + dj), (di, dj)));
            }
            Some(Tile::Hor) => {
                work.push_back(((i, j + 1), (0, 1)));
                work.push_back(((i, j - 1), (0, -1)));
            }
            Some(Tile::Ver) => {
                work.push_back(((i + 1, j), (1, 0)));
                work.push_back(((i - 1, j), (-1, 0)));
            }
            Some(Tile::Bck) => {
                work.push_back(((i + dj, j + di), (dj, di)));
            }
            Some(Tile::Fwd) => {
                work.push_back(((i - dj, j - di), (-dj, -di)));
            }
        }
    }
    let energised: HashSet<Coords> = HashSet::from_iter(seen.iter().map(|p| p.0));
    energised.len()
}

#[aoc(day16, part1)]
fn solve_part1(input: &Task) -> usize {
    let seed = ((0, 0), (0, 1));
    eval(input, seed)
}

#[aoc(day16, part2)]
fn solve_part2(input: &Task) -> usize {
    let max_hor = (0..input.h)
        .flat_map(|i| [((i, 0), (0, 1)), ((i, input.w - 1), (0, -1))])
        .par_bridge()
        .map(|p| eval(input, p))
        .max()
        .unwrap();
    let max_ver = (0..input.w)
        .flat_map(|j| [((0, j), (1, 0)), ((input.h - 1, j), (-1, 0))])
        .par_bridge()
        .map(|p| eval(input, p))
        .max()
        .unwrap();
    max_hor.max(max_ver)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 46);
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 51);
    }
}
