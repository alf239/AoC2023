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
            from:string(alpha+) " = ("
            left:string(alpha+) ", "
            right:string(alpha+) ")"
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

#[aoc(day8, part2)]
fn solve_part2(input: &Task) -> usize {
    2
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
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 2);
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
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 2);
    }
}
