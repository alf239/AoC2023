use num_integer::Integer;
use std::collections::HashMap;

use aoc_parse::{parser, prelude::*};

const AAA: &str = "AAA";
const ZZZ: &str = "ZZZ";

pub struct Task {
    prog: String,
    net: HashMap<String, (String, String)>,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Task {
    let p = parser!(
        prog:line(string(alpha+))
        line("")
        nodes:lines(
            from:string(alnum+) " = ("
            left:string(alnum+) ", "
            right:string(alnum+) ")"
            => (from, (left, right)))
        => Task { prog, net: nodes.into_iter().collect::<HashMap<String, (String, String)>>() }
    );
    p.parse(input).unwrap()
}

#[aoc(day8, part1)]
fn solve_part1(input: &Task) -> usize {
    let prog: Vec<char> = input.prog.chars().into_iter().collect();
    let mut n: usize = 0;
    let aaa = AAA.to_string();
    let mut node: &String = &aaa;
    while node != ZZZ {
        let cmd = prog[n % prog.len()];
        n += 1;
        let (left, right) = input.net.get(node).unwrap();
        node = if cmd == 'L' { left } else { right };
    }
    n
}

fn hare_tortoise<T, F>(seed: T, step: F) -> usize
where
    T: Copy,
    T: Eq,
    F: Fn(T) -> T,
{
    let mut hare: T = seed;
    let mut tortoise: T = seed;

    loop {
        hare = step(step(hare));
        tortoise = step(tortoise);
        if hare == tortoise {
            let mut len: usize = 0;
            loop {
                hare = step(hare);
                len += 1;
                if hare == tortoise {
                    break;
                }
            }

            return len;
        }
    }
}

#[aoc(day8, part2)]
fn solve_part2(input: &Task) -> i64 {
    let prog: Vec<char> = input.prog.chars().into_iter().collect();
    let prog_len = prog.len();
    let starts: Vec<&String> = input
        .net
        .keys()
        .into_iter()
        .filter(|&node| node.ends_with("A"))
        .collect();

    starts
        .iter()
        .map(|&s| {
            hare_tortoise((s, 0), |(s, i)| {
                let cmd = prog[i];
                let (left, right) = input.net.get(s).unwrap();
                (if cmd == 'L' { left } else { right }, (i + 1) % prog_len)
            })
        })
        .fold(1, |acc, cycle_len| acc.gcd_lcm(&(cycle_len as i64)).1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 2);
    }

    #[test]
    fn example2() {
        let input = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 6);
    }

    #[test]
    fn example_pt2() {
        let input = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#
            .trim();
        let parsed = input_generator(input);
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 6);
    }
}
