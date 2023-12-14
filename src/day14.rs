use std::{collections::HashMap, mem::swap};

use aoc_parse::{parser, prelude::*};

type Task = Vec<Vec<usize>>;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Task {
    let p = parser!(
        lines(char_of(".#O")+)
    );
    p.parse(input).unwrap()
}

fn load(state: &Task) -> usize {
    let n = state.len();
    let mut weight = 0;
    for (i, row) in state.iter().enumerate() {
        for (_, c) in row.iter().enumerate() {
            if *c == 2 {
                weight += n - i;
            }
        }
    }
    weight
}

fn step(input: &Task, work: &mut Task, h: i32, w: i32, di: i32, dj: i32) {
    for i in 0..work.len() {
        for j in 0..work[i].len() {
            work[i][j] = 0;
        }
    }
    let di1 = if di == 0 { 1 } else { -di };
    let dj1 = if dj == 0 { 1 } else { -dj };
    let mut i = if di1 > 0 { 0 } else { h - 1 };
    let js = if dj1 > 0 { 0 } else { w - 1 };
    while i >= 0 && i < h {
        let row = &input[i as usize];
        let mut j = js;
        while j >= 0 && j < w {
            let c = row[j as usize];
            if c == 2 {
                let mut k = i as i32;
                let mut l = j as i32;
                while k + di >= 0
                    && l + dj >= 0
                    && k + di < h
                    && l + dj < w
                    && work[(k + di) as usize][(l + dj) as usize] == 0
                {
                    k += di;
                    l += dj;
                }
                work[k as usize][l as usize] = 2;
            } else {
                work[i as usize][j as usize] = c;
            }
            j += dj1;
        }
        i += di1;
    }
}

#[aoc(day14, part1)]
fn solve_part1(input: &Task) -> usize {
    let mut work: Vec<Vec<usize>> = input.into_iter().map(|r| vec![0; r.len()]).collect();
    step(
        input,
        &mut work,
        input.len() as i32,
        input[0].len() as i32,
        -1,
        0,
    );
    load(&work)
}

const SOME_PRIME: usize = 2_147_483_647;

fn hash(state: &Task) -> usize {
    let mut result = 37;
    for row in state {
        for c in row {
            result = (result * 17 + *c) % SOME_PRIME;
        }
    }
    result
}

#[aoc(day14, part2)]
fn solve_part2(input: &Task) -> usize {
    let n = 1000000000;
    let h = input.len() as i32;
    let w = input[0].len() as i32;
    let mut seen: HashMap<usize, usize> = HashMap::new();
    let mut weights: Vec<usize> = Vec::new();
    let mut state: Vec<Vec<usize>> = input.into_iter().cloned().collect();
    let mut work: Vec<Vec<usize>> = input.into_iter().map(|r| vec![0; r.len()]).collect();
    let cycle = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    weights.push(load(&state));
    for i in 0..n {
        for (di, dj) in cycle {
            step(&state, &mut work, h, w, di, dj);
            swap(&mut state, &mut work);
        }
        let hsh = hash(&state);
        match seen.get(&hsh) {
            Some(j) => {
                let cycle = i + 1 - j;
                let target = (n - j) % cycle + j;
                return weights[target];
            }
            None => {
                seen.insert(hsh, i + 1);
                weights.push(load(&state));
            }
        }
    }
    load(&state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 136);
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 64);
    }
}
