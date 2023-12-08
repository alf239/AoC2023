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

#[derive(Debug)]
struct FoundLoop {
    start: usize,
    len: usize,
    flag_at: usize,
}

fn hare_tortoise<T, F, P>(seed: T, step: F, flag: P) -> FoundLoop
where
    T: Copy,
    T: Eq,
    F: Fn(T) -> T,
    P: Fn(T) -> bool,
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

            hare = seed;
            tortoise = seed;
            for _ in 0..len {
                hare = step(hare);
            }
            let mut start = 0;
            loop {
                hare = step(hare);
                tortoise = step(tortoise);
                start += 1;
                if hare == tortoise {
                    break;
                }
            }

            hare = seed;
            let mut flag_at = 0;
            for i in 0..start + len {
                if flag(hare) {
                    flag_at = i;
                    break;
                }
                hare = step(hare);
            }

            return FoundLoop {
                start,
                len,
                flag_at,
            };
        }
    }
}

// https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let mut s = 0;
    let mut old_s = 1;
    let mut r = b;
    let mut old_r = a;

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
    }

    let bezout_t = if b == 0 { 0 } else { (old_r - old_s * a) / b };

    (old_s, bezout_t, old_r)
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

    let cycles: Vec<FoundLoop> = starts
        .iter()
        .map(|&s| {
            hare_tortoise(
                (s, 0),
                |(s, i)| {
                    let cmd = prog[i];
                    let (left, right) = input.net.get(s).unwrap();
                    (if cmd == 'L' { left } else { right }, (i + 1) % prog_len)
                },
                |(s, _)| s.ends_with("Z"),
            )
        })
        .collect();

    cycles.iter().fold(1, |acc, fl| {
        let flag_at = fl.flag_at as i64;
        let (_, _, r) = extended_gcd(acc, flag_at);
        acc * flag_at / r
    })
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
