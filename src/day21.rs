use std::{
    collections::{HashMap, HashSet},
    mem::swap,
};

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

fn part2(input: &Task, n: usize) -> u64 {
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

    let mut seen = HashSet::new();
    let mut boundary: HashSet<(i32, i32)> = HashSet::from_iter([(is as i32, js as i32)]);
    let mut prev = 0u64;
    let mut count = 1u64;
    let mut old_count = 0;
    let mut old_old_count = 0;
    for i in 0..n {
        let mut next = HashSet::new();
        for (ii, jj) in boundary.iter() {
            let ni = normalize(*ii, input.len());
            let nj = normalize(*jj, input[0].len());

            next.extend(
                steps
                    .get(&(ni, nj))
                    .unwrap()
                    .iter()
                    .map(|(di, dj)| (ii + di, jj + dj))
                    .filter(|(i, j)| !seen.contains(&(*i, *j))),
            );
        }
        let nn = prev + next.len() as u64;
        prev = count;
        count = nn;

        // println!(
        //     "Step {}: {} {}",
        //     i,
        //     count,
        //     count as f64 / (0.001 + (0.84405 * (i * i) as f64))
        // );

        if i > 0 && (26501365 - i - 1) % input.len() == 0 {
            let cycles_left = (26501365 - i - 1) / input.len();
            if i > 5000 && cycles_left % 10 == 0 {
                println!("Step {}, {} cycles to go", i, cycles_left);
                let left = cycles_left as u64;
                let a = count - old_count;
                let d = (count - old_count) - (old_count - old_old_count);
                let a = a + d;
                let predict = count + left * (2 * a + (left - 1) * d) / 2;
                println!("Speed: {}, Acc: {}, Prognosis: {}", a, d, predict);
            }
            old_old_count = old_count;
            old_count = count;
        }

        swap(&mut seen, &mut boundary);
        swap(&mut boundary, &mut next);
    }
    count
}

fn normalize(x: i32, h: usize) -> usize {
    ((x % h as i32) + h as i32) as usize % h
}

#[aoc(day21, part2)]
fn solve_part2(input: &Task) -> u64 {
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
        assert_eq!(part2(&parsed, 500), 167004);
    }

    #[test]
    fn example3() {
        let parsed = input_generator(INPUT);
        assert_eq!(part2(&parsed, 1000), 668697);
    }

    #[test]
    fn example4() {
        let parsed = input_generator(INPUT);
        assert_eq!(part2(&parsed, 5000), 16733044);
    }
}
