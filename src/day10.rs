use std::collections::{HashMap, HashSet, VecDeque};

use aoc_parse::{parser, prelude::*};

type Coords = (i32, i32);
type Task = HashMap<Coords, char>;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Task {
    let p = parser!(lines(char_of("|-LJ7F.S")+));
    let rows = p.parse(input).unwrap();
    rows.iter()
        .enumerate()
        .flat_map(|(i, r)| {
            r.iter().enumerate().filter_map(move |(j, c)| match c {
                0 => Some(((i as i32, j as i32), '|')),
                1 => Some(((i as i32, j as i32), '-')),
                2 => Some(((i as i32, j as i32), 'L')),
                3 => Some(((i as i32, j as i32), 'J')),
                4 => Some(((i as i32, j as i32), '7')),
                5 => Some(((i as i32, j as i32), 'F')),
                7 => Some(((i as i32, j as i32), 'S')),
                _ => None,
            })
        })
        .collect()
}

fn try_schedule(
    coords: Coords,
    d: usize,
    filter: &str,
    map: &Task,
    work: &mut VecDeque<(Coords, usize)>,
) {
    if map.get(&coords).map_or(false, |&c| filter.contains(c)) {
        work.push_back((coords, d + 1));
    }
}

#[aoc(day10, part1)]
fn solve_part1(input: &Task) -> usize {
    let start = input
        .iter()
        .find_map(|(coords, &c)| if c == 'S' { Some(*coords) } else { None })
        .unwrap();
    let mut dist: HashMap<Coords, usize> = HashMap::new();
    let mut work: VecDeque<(Coords, usize)> = VecDeque::from([(start, 0)]);
    while !work.is_empty() {
        let ((i, j), d) = work.pop_front().unwrap();
        if dist.contains_key(&(i, j)) {
            continue;
        }
        dist.insert((i, j), d);
        let c = input.get(&(i, j)).unwrap();
        match c {
            '|' => {
                try_schedule((i - 1, j), d, "7F|", input, &mut work);
                try_schedule((i + 1, j), d, "JL|", input, &mut work);
            }
            '-' => {
                try_schedule((i, j - 1), d, "L-F", input, &mut work);
                try_schedule((i, j + 1), d, "J-7", input, &mut work);
            }
            'L' => {
                try_schedule((i - 1, j), d, "7F|", input, &mut work);
                try_schedule((i, j + 1), d, "J-7", input, &mut work);
            }
            'J' => {
                try_schedule((i, j - 1), d, "L-F", input, &mut work);
                try_schedule((i - 1, j), d, "7F|", input, &mut work);
            }
            '7' => {
                try_schedule((i, j - 1), d, "L-F", input, &mut work);
                try_schedule((i + 1, j), d, "JL|", input, &mut work);
            }
            'F' => {
                try_schedule((i, j + 1), d, "J-7", input, &mut work);
                try_schedule((i + 1, j), d, "JL|", input, &mut work);
            }
            'S' => {
                try_schedule((i - 1, j), d, "7F|", input, &mut work);
                try_schedule((i + 1, j), d, "JL|", input, &mut work);
                try_schedule((i, j - 1), d, "L-F", input, &mut work);
                try_schedule((i, j + 1), d, "J-7", input, &mut work);
            }
            _ => (),
        }
    }

    *dist.values().max().unwrap()
}

#[aoc(day10, part2)]
fn solve_part2(input: &Task) -> i32 {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"
.....
.S-7.
.|.|.
.L-J.
....."#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 4);
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 2);
    }

    #[test]
    fn example2() {
        let input = r#"
..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 8);
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 2);
    }
}
