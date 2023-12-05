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
    maps.iter().fold(seed, |x, rules| {
        match rules.iter().find(|&m| m.covers(x)) {
            Some(m) => m.translate(x),
            None => x,
        }
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

#[aoc(day5, part2)]
pub fn solve_part2(input: &Task) -> u32 {
    let s = &input.seeds;
    let seeds: Vec<u32> = (0..s.len()/2)
        .flat_map(|i| (s[i * 2]..s[i * 2] + s[i * 2 + 1]))
        .collect();
    seeds
        .iter()
        .map(|&seed| location(seed, &input.maps))
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
