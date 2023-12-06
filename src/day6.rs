use std::cmp::{max, min};

use aoc_parse::{parser, prelude::*};

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> (Vec<u32>, Vec<u32>) {
    let p = parser!(
        line("Time:" " "+ times:repeat_sep(u32, " "+) => times)
        line("Distance:" " "+ distances:repeat_sep(u32, " "+) => distances)
    );
    p.parse(input).unwrap()
}

fn number_of_ways(t: u64, d: u64) -> u64 {
    // x * (t - x) = d => x^ 2 - tx + d = 0
    let discriminant = ((t * t) as i64) - 4 * d as i64;
    if discriminant > 0 {
        let sqrt_disc = (discriminant as f64).sqrt();
        let root1 = (t as f64 - sqrt_disc) / 2.0;
        let root2 = (t as f64 + sqrt_disc) / 2.0;

        let first = max(0, (root1 + 1.0).floor() as i64);
        let last = min(t as i64, (root2 - 1.0).ceil() as i64);

        if last >= first {
            (1 + last - first) as u64
        } else {
            0
        }
    } else {
        0
    }
}

fn concat(nrs: &Vec<u32>) -> u64 {
    nrs.iter().fold(0, |acc, &t| {
        acc * 10u64.pow(t.to_string().len() as u32) + (t as u64)
    })
}

#[aoc(day6, part1)]
pub fn solve_part1((times, distances): &(Vec<u32>, Vec<u32>)) -> u64 {
    times
        .iter()
        .zip(distances)
        .map(|(&t, &d)| number_of_ways(t as u64, d as u64))
        .product()
}

#[aoc(day6, part2)]
pub fn solve_part2((times, distances): &(Vec<u32>, Vec<u32>)) -> u64 {
    let time = concat(times);
    let distance = concat(distances);
    // println!("{}, {}", time, distance);
    number_of_ways(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "Time:      7  15   30\nDistance:  9  40  200";
        let parsed = input_generator(input);
        let answer1 = solve_part1(&parsed);
        assert_eq!(answer1, 288);
        let answer2 = solve_part2(&parsed);
        assert_eq!(answer2, 71503);
    }
}
