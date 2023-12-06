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
    (0..=t).map(|j| j * (t - j)).filter(|&d1| d1 > d).count() as u64
}

fn concat(nrs: &Vec<u32>) -> u64 {
    nrs.iter().fold(0, |acc, &t| acc * 10u64.pow(t.to_string().len() as u32) + (t as u64))
}

#[aoc(day6, part1)]
pub fn solve_part1((times, distances): &(Vec<u32>, Vec<u32>)) -> u64 {
    times.iter().zip(distances).map(|(&t, &d)| number_of_ways(t as u64, d as u64)).product()
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