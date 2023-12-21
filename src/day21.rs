use std::collections::{HashMap, HashSet, VecDeque};

use aoc_parse::{parser, prelude::*};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Loc {
    Start,
    Plot,
    Rock,
}

type Task = Vec<Vec<Loc>>;

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Task {
    use Loc::*;
    let loc = parser!({
        "S" => Start,
        "." => Plot,
        "#" => Rock,
    });
    let p = parser!(lines(loc+));
    p.parse(input).unwrap()
}

fn part1(input: &Task, n: usize) -> usize {
    let mut is = usize::MAX;
    let mut js = 0;
    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == Loc::Start {
                is = i;
                js = j;
                break;
            }
        }
        if is != usize::MAX {
            break;
        }
    }

    let mut steps: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == Loc::Rock {
                continue;
            }
            let mut voisins: Vec<(usize, usize)> = Vec::new();
            if i > 0 && input[i - 1][j] != Loc::Rock {
                voisins.push((i - 1, j));
            }
            if j > 0 && input[i][j - 1] != Loc::Rock {
                voisins.push((i, j - 1));
            }
            if i < input.len() - 1 && input[i + 1][j] != Loc::Rock {
                voisins.push((i + 1, j));
            }
            if j < input[i].len() - 1 && input[i][j + 1] != Loc::Rock {
                voisins.push((i, j + 1));
            }
            steps.insert((i, j), voisins);
        }
    }

    let mut state = vec![(is, js)];
    for _ in 0..n {
        let mut next = HashSet::new();
        for s in state.iter() {
            next.extend(steps.get(s).unwrap());
        }
        state.clear();
        state.extend(next.iter());
    }
    state.len()
}

#[aoc(day21, part1)]
fn solve_part1(input: &Task) -> usize {
    part1(input, 64)
}

fn part2(input: &Task, n: usize) -> usize {
    let mut is = usize::MAX;
    let mut js = 0;
    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == Loc::Start {
                is = i;
                js = j;
                break;
            }
        }
        if is != usize::MAX {
            break;
        }
    }

    let mut steps: HashMap<(usize, usize), Vec<(i32, i32)>> = HashMap::new();
    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == Loc::Rock {
                continue;
            }
            let mut voisins: Vec<(i32, i32)> = Vec::new();
            if input[(i + input.len() - 1) % input.len()][j] != Loc::Rock {
                voisins.push((-1, 0));
            }
            if input[i][(j + input[i].len() - 1) % input[i].len()] != Loc::Rock {
                voisins.push((0, -1));
            }
            if input[(i + 1) % input.len()][j] != Loc::Rock {
                voisins.push((1, 0));
            }
            if input[i][(j + 1) % input[i].len()] != Loc::Rock {
                voisins.push((0, 1));
            }
            steps.insert((i, j), voisins);
        }
    }

    let mut state = vec![(is as i32, js as i32)];
    for _ in 0..n {
        let mut next = HashSet::new();
        for (ii, jj) in state.iter() {
            let ni = normalize(*ii, input.len());
            let nj = normalize(*jj, input[0].len());

            next.extend(
                steps
                    .get(&(ni, nj))
                    .unwrap()
                    .iter()
                    .map(|(di, dj)| (ii + di, jj + dj)),
            );
        }
        state.clear();
        state.extend(next.iter());
    }
    state.len()
}

fn normalize(x: i32, h: usize) -> usize {
    ((x % h as i32) + h as i32) as usize % h
}

#[aoc(day21, part2)]
fn solve_part2(input: &Task) -> usize {
    part2(input, 26501365)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;

    #[test]
    fn example1() {
        let parsed = input_generator(INPUT);
        let result1 = part1(&parsed, 6);
        assert_eq!(result1, 16);
        assert_eq!(part2(&parsed, 6), 16);
        assert_eq!(part2(&parsed, 10), 50);
    }

    #[test]
    fn example2() {
        let parsed = input_generator(INPUT);
        assert_eq!(part2(&parsed, 100), 6536);
    }

    #[test]
    fn example3() {
        let parsed = input_generator(INPUT);
        assert_eq!(part2(&parsed, 500), 167004);
        assert_eq!(part2(&parsed, 1000), 668697);
        assert_eq!(part2(&parsed, 5000), 16733044);
    }
}
