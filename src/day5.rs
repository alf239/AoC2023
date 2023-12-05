use std::{collections::VecDeque, ops::Range};

use aoc_parse::{parser, prelude::*};

pub struct Map {
    dst: u32,
    src: u32,
    len: u32,
}

impl Map {
    fn covers(&self, x: u32) -> bool {
        self.src <= x && self.src + self.len >= x
    }

    fn translate(&self, x: u32) -> u32 {
        x + self.dst - self.src
    }

    fn map_range(&self, range: Range<u32>) -> (Option<Range<u32>>, Vec<Range<u32>>) {
        if range.end <= self.src || range.start >= self.src + self.len {
            (None, vec![range])
        } else if range.end <= self.src + self.len && range.start >= self.src {
            (
                Some(self.translate(range.start)..self.translate(range.end)),
                Vec::new(),
            )
        } else if range.start <= self.src && range.end >= self.src + self.len {
            let mut leftovers = Vec::new();
            if range.start < self.src {
                leftovers.push(range.start..self.src);
            }
            if range.end > self.src + self.len {
                leftovers.push(self.src + self.len..range.end);
            }
            (Some(self.dst..self.dst + self.len), leftovers)
        } else if range.start <= self.src {
            (
                Some(self.dst..self.translate(range.end)),
                vec![(range.start..self.src)],
            )
        } else {
            (
                Some(self.translate(range.start)..self.dst + self.len),
                vec![(self.src + self.len..range.end)],
            )
        }
    }
}

pub struct Task {
    seeds: Vec<u32>,
    maps: Vec<Vec<Map>>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Task {
    let p = parser!(
        seeds:line("seeds: " repeat_sep(u32, " "+))
        line("")
        maps:sections(
            line(string(any_char+))
            maps:lines(dst:u32 " " src:u32 " " len:u32 => Map { dst, src, len })
            => maps
        )
        => Task { seeds, maps }
    );
    p.parse(input).unwrap()
}

fn location(seed: u32, maps: &Vec<Vec<Map>>) -> u32 {
    maps.iter()
        .fold(seed, |x, rules| match rules.iter().find(|&m| m.covers(x)) {
            Some(m) => m.translate(x),
            None => x,
        })
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Task) -> u32 {
    input
        .seeds
        .iter()
        .map(|&seed| location(seed, &input.maps))
        .min()
        .unwrap()
}

fn step2(xs: &Vec<Range<u32>>, map: &Vec<Map>) -> Vec<Range<u32>> {
    let mut work: VecDeque<Range<u32>> = xs.iter().cloned().collect();
    let mut result = Vec::new();
    for m in map {
        let len = work.len();
        for _ in 0..len {
            let range = work.pop_front().unwrap();
            let (mapped, leftover) = m.map_range(range);
            result.extend(mapped.iter().cloned());
            work.extend(leftover.iter().cloned());
        }
    }
    result.extend(work.iter().cloned());
    result
}

fn locations(seeds: &Vec<Range<u32>>, maps: &Vec<Vec<Map>>) -> Vec<Range<u32>> {
    let mut prev = seeds.clone();
    for m in maps {
        prev = step2(&prev, m);
    }
    prev
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Task) -> u32 {
    let seeds: Vec<Range<u32>> = input
        .seeds
        .chunks(2)
        .map(|def| def[0]..def[0] + def[1])
        .collect();
    locations(&seeds, &input.maps)
        .iter()
        .map(|r| r.start)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#
            .trim();
        let parsed = input_generator(input);
        let answer = solve_part1(&parsed);
        assert_eq!(answer, 35);
        let answer2 = solve_part2(&parsed);
        assert_eq!(answer2, 46);
    }
}
