use std::collections::{HashSet, VecDeque};

use aoc_parse::{parser, prelude::*};

pub struct Cmd {
    dir: usize,
    len: usize,
    rgb: u32,
}

type Task = Vec<Cmd>;

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Task {
    let p = parser!(
        lines(
            dir:char_of("URDL") " " len:usize " (#" rgb:u32_hex ")" => Cmd { dir, len, rgb }
        )
    );
    p.parse(input).unwrap()
}

#[aoc(day18, part1)]
fn solve_part1(input: &Task) -> usize {
    let mut m: HashSet<(i32, i32)> = HashSet::new();
    let mut i = 0;
    let mut j = 0;
    m.insert((i, j));
    for Cmd { dir, len, rgb: _ } in input {
        for _ in 0..*len {
            match *dir {
                0 => i -= 1,
                1 => j += 1,
                2 => i += 1,
                3 => j -= 1,
                _ => panic!("Holy shit, what's {}", *dir),
            }
            m.insert((i, j));
        }
    }

    let seed = find_seed(&m);

    let mut work = VecDeque::new();
    work.push_back(seed);
    while let Some((i, j)) = work.pop_front() {
        if m.contains(&(i, j)) {
            continue;
        }
        m.insert((i, j));
        work.push_back((i + 1, j));
        work.push_back((i - 1, j));
        work.push_back((i, j + 1));
        work.push_back((i, j - 1));
    }

    // dump(&m);
    m.len()
}

fn find_seed(m: &HashSet<(i32, i32)>) -> (i32, i32) {
    let mni = m.iter().map(|p| p.0).min().unwrap();
    let mnj = m.iter().map(|p| p.1).min().unwrap();
    let mxj = m.iter().map(|p| p.1).max().unwrap();

    let seed_i = mni + 1;
    let mut seed_j = 0;
    let mut saw_hash = false;
    for j in mnj..=mxj {
        let contains = m.contains(&(seed_i, j));
        if saw_hash && !contains {
            seed_j = j;
            break;
        }
        if contains {
            saw_hash = true;
        }
    }
    (seed_i, seed_j)
}

#[aoc(day18, part2)]
fn solve_part2(input: &Task) -> i64 {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 62);
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 952408144115);
    }
}
