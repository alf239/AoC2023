use std::{collections::{HashMap, HashSet, VecDeque}, cmp::max};

use aoc_parse::{parser, prelude::*};

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Loc {
    Path,
    Left,
    Right,
    Up,
    Down,
}

type Coords = (usize, usize);
type Task = HashMap<Coords, Loc>;

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Task {
    let codes = vec![
        None,
        Some(Loc::Path),
        Some(Loc::Left),
        Some(Loc::Right),
        Some(Loc::Up),
        Some(Loc::Down),
    ];
    let p = parser!(lines(
        (x:char_of("#.<>^v") => codes[x])+
    ));
    p.parse(input)
        .unwrap()
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, c)| c.map(|l| ((i, j), l)))
        })
        .collect()
}

#[aoc(day23, part1)]
fn solve_part1(input: &Task) -> usize {
    let start = input.keys().min().unwrap().clone();
    let end = input.keys().max().unwrap().clone();

    let mut nodes: Vec<(usize, usize)> = vec![start, end];
    for (i, j) in input.keys() {
        if *i == 0 {
            continue; // skip the start
        }
        let neighbours = [(i - 1, *j), (i + 1, *j), (*i, j - 1), (*i, j + 1)]
            .iter()
            .filter(|&loc| input.contains_key(loc))
            .count();
        if neighbours > 2 {
            nodes.push((*i, *j));
        }
    }

    let mut edges: HashMap<(usize, usize), usize> = HashMap::new();
    for (nr, &node) in nodes.iter().enumerate().filter(|(_, &n)| n != end) {
        let mut seen: HashSet<Coords> = HashSet::new();
        let mut work = VecDeque::new();
        work.push_back((node, 0));
        while let Some((n @ (i, j), d)) = work.pop_front() {
            if seen.contains(&n) {
                continue;
            }
            seen.insert(n);
            if d > 0 {
                if let Some(dst) = nodes.iter().position(|&x| x == n) {
                    edges.insert((nr, dst), d);
                    continue;
                }
            }
            if i > 0
                && input
                    .get(&(i - 1, j))
                    .map_or(false, |&loc| loc != Loc::Down)
            {
                work.push_back(((i - 1, j), d + 1));
            }
            if input.get(&(i + 1, j)).map_or(false, |&loc| loc != Loc::Up) {
                work.push_back(((i + 1, j), d + 1));
            }
            if input
                .get(&(i, j - 1))
                .map_or(false, |&loc| loc != Loc::Right)
            {
                work.push_back(((i, j - 1), d + 1));
            }
            if input
                .get(&(i, j + 1))
                .map_or(false, |&loc| loc != Loc::Left)
            {
                work.push_back(((i, j + 1), d + 1));
            }
        }
    }

    let mut dist = vec![0; nodes.len()];
    let mut work = VecDeque::new();
    work.push_back(0);
    while let Some(n) = work.pop_front() {
        for ((f, t), d) in edges.iter() {
            if *f == n {
                if dist[n] + d > dist[*t] {
                    dist[*t] = dist[n] + d;
                }
                work.push_back(*t);
            }
        }
    }
    dist[1]
}

#[aoc(day23, part2)]
fn solve_part2(input: &Task) -> usize {
    let start = input.keys().min().unwrap().clone();
    let end = input.keys().max().unwrap().clone();

    let mut nodes: Vec<(usize, usize)> = vec![start, end];
    for (i, j) in input.keys() {
        if *i == 0 {
            continue; // skip the start
        }
        let neighbours = [(i - 1, *j), (i + 1, *j), (*i, j - 1), (*i, j + 1)]
            .iter()
            .filter(|&loc| input.contains_key(loc))
            .count();
        if neighbours > 2 {
            nodes.push((*i, *j));
        }
    }

    let mut edges: HashMap<(usize, usize), usize> = HashMap::new();
    for (nr, &node) in nodes.iter().enumerate().filter(|(_, &n)| n != end) {
        let mut seen: HashSet<Coords> = HashSet::new();
        let mut work = VecDeque::new();
        work.push_back((node, 0));
        while let Some((n @ (i, j), d)) = work.pop_front() {
            if seen.contains(&n) {
                continue;
            }
            seen.insert(n);
            if d > 0 {
                if let Some(dst) = nodes.iter().position(|&x| x == n) {
                    edges.insert((nr, dst), d);
                    continue;
                }
            }
            if i > 0 && input.contains_key(&(i - 1, j)) {
                work.push_back(((i - 1, j), d + 1));
            }
            if input.contains_key(&(i + 1, j)) {
                work.push_back(((i + 1, j), d + 1));
            }
            if input.contains_key(&(i, j - 1)) {
                work.push_back(((i, j - 1), d + 1));
            }
            if input.contains_key(&(i, j + 1)) {
                work.push_back(((i, j + 1), d + 1));
            }
        }
    }

    let mut dist = 0;
    let mut work: VecDeque<(usize, u64, usize)> = VecDeque::new();
    work.push_back((0, 1, 0));
    while let Some((n, mask, d)) = work.pop_front() {
        if n == 1 {
            dist = max(dist, d);
        }
        for ((f, t), dd) in edges.iter() {
            if *f == n {
                let step_mask = 1u64 << *t;
                if step_mask & mask == 0 {
                    work.push_back((*t, step_mask | mask, d + dd));
                }
            }
        }
    }
    dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 94);
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 154);
    }
}
