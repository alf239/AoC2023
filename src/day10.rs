use std::collections::{HashMap, HashSet, VecDeque};

use aoc_parse::{parser, prelude::*};

type Coords = (i32, i32);
type Task = (usize, usize, HashMap<Coords, char>);

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Task {
    let p = parser!(lines(char_of("|-LJ7F.S")+));
    let rows = p.parse(input).unwrap();
    let map = rows
        .iter()
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
        .collect();
    (rows.len(), rows[0].len(), map)
}

fn try_schedule(
    coords: Coords,
    d: usize,
    filter: &str,
    map: &HashMap<Coords, char>,
    work: &mut VecDeque<(Coords, usize)>,
) {
    if map.get(&coords).map_or(false, |&c| filter.contains(c)) {
        work.push_back((coords, d + 1));
    }
}

fn dijkstra_loop(map: &HashMap<Coords, char>) -> HashMap<Coords, usize> {
    let start = map
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
        let c = map.get(&(i, j)).unwrap();
        match c {
            '|' => {
                try_schedule((i - 1, j), d, "7F|", map, &mut work);
                try_schedule((i + 1, j), d, "JL|", map, &mut work);
            }
            '-' => {
                try_schedule((i, j - 1), d, "L-F", map, &mut work);
                try_schedule((i, j + 1), d, "J-7", map, &mut work);
            }
            'L' => {
                try_schedule((i - 1, j), d, "7F|", map, &mut work);
                try_schedule((i, j + 1), d, "J-7", map, &mut work);
            }
            'J' => {
                try_schedule((i, j - 1), d, "L-F", map, &mut work);
                try_schedule((i - 1, j), d, "7F|", map, &mut work);
            }
            '7' => {
                try_schedule((i, j - 1), d, "L-F", map, &mut work);
                try_schedule((i + 1, j), d, "JL|", map, &mut work);
            }
            'F' => {
                try_schedule((i, j + 1), d, "J-7", map, &mut work);
                try_schedule((i + 1, j), d, "JL|", map, &mut work);
            }
            'S' => {
                try_schedule((i - 1, j), d, "7F|", map, &mut work);
                try_schedule((i + 1, j), d, "JL|", map, &mut work);
                try_schedule((i, j - 1), d, "L-F", map, &mut work);
                try_schedule((i, j + 1), d, "J-7", map, &mut work);
            }
            _ => (),
        }
    }
    dist
}

#[aoc(day10, part1)]
fn solve_part1(input: &Task) -> usize {
    let (_, _, map) = input;
    let dist = dijkstra_loop(map);
    *dist.values().max().unwrap()
}

enum Pos {
    In,
    Out,
    LineIn,
    LineOut
}

#[aoc(day10, part2)]
fn solve_part2(input: &Task) -> usize {
    let (h, w, map) = input;
    let dist = dijkstra_loop(map);
    let mut count = 0;
    let mut inside = false;
    for i in 0..*h as i32 {
        for j in 0..*w as i32 {
            let on_line = dist.contains_key(&(i, j));
            let c = map.get(&(i, j));
            if on_line && c.map_or(false, |&c| "|LJ".contains(c)) {
                inside = !inside;
            }
            if !on_line && inside {
                count += 1;
            }
        }
    }
    count
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
        assert_eq!(result2, 1);
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
        assert_eq!(result2, 1);
    }

    #[test]
    fn example3() {
        let input = r#"
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 23);
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 4);
    }

    #[test]
    fn example4() {
        let input = r#"
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 70);
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 8);
    }

    #[test]
    fn example5() {
        let input = r#"
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 80);
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 10);
    }
}
