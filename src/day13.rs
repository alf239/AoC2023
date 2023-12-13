use std::{collections::HashSet, iter::repeat};

use aoc_parse::{parser, prelude::*};

type Task = Vec<Vec<Vec<usize>>>;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Task {
    let p = parser!(
        repeat_sep(lines( char_of(".#")+), line(""))
    );
    p.parse(input).unwrap()
}

fn sum1_1d(xs: &Vec<usize>) -> usize {
    for i in 1..xs.len() {
        if xs[..i]
            .iter()
            .rev()
            .zip(xs[i..].iter())
            .all(|(&a, &b)| a == b)
        {
            return i;
        }
    }
    0
}

fn bits(x: usize) -> usize {
    let mut n = x;
    n = ((0xaaaaaaaa & n) >> 1) + (0x55555555 & n);
    n = ((0xcccccccc & n) >> 2) + (0x33333333 & n);
    n = ((0xf0f0f0f0 & n) >> 4) + (0x0f0f0f0f & n);
    n = ((0xff00ff00 & n) >> 8) + (0x00ff00ff & n);
    n = ((0xffff0000 & n) >> 16) + (0x0000ffff & n);
    return n;
}

fn sum2_1d(xs: &Vec<usize>) -> usize {
    for i in 1..xs.len() {
        if xs[..i]
            .iter()
            .rev()
            .zip(xs[i..].iter())
            .map(|(&a, &b)| bits(a ^ b))
            .sum::<usize>() == 1
        {
            return i;
        }
    }
    0
}

fn summarise_lines(m: &Vec<Vec<usize>>) -> (Vec<usize>, Vec<usize>) {
    let hor: Vec<usize> = m
        .iter()
        .map(|s| s.iter().fold(0, |acc, c| 2 * acc + c))
        .collect();
    let ver: Vec<usize> = (0..m[0].len())
        .map(|j| m.iter().fold(0, |acc, s| 2 * acc + s[j]))
        .collect();
    (hor, ver)
}

fn sum1(m: &Vec<Vec<usize>>) -> usize {
    let (hor, ver) = summarise_lines(m);

    sum1_1d(&hor) * 100 + sum1_1d(&ver)
}

#[aoc(day13, part1)]
fn solve_part1(input: &Task) -> usize {
    input.iter().map(sum1).sum()
}

fn sum2(m: &Vec<Vec<usize>>) -> usize {
    let (hor, ver) = summarise_lines(m);

    sum2_1d(&hor) * 100 + sum2_1d(&ver)
}

#[aoc(day13, part2)]
fn solve_part2(input: &Task) -> usize {
    input.iter().map(sum2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 405);
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 400);
    }
}
