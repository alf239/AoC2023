use std::collections::{HashMap, VecDeque};

use aoc_parse::{parser, prelude::*};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Node {
    Broadcaster,
    FlipFlop,
    Nand,
}

type Task = HashMap<String, (Node, Vec<String>)>;

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Task {
    use Node::*;
    let node = parser!({
        "broadcaster" => ("broadcaster".to_string(), Broadcaster),
        "%" name:string(alpha+) => (name, FlipFlop),
        "&" name:string(alpha+) => (name, Nand),
    });
    let p = parser!(lines(node " -> " repeat_sep(string(alpha+), ", ")));
    p.parse(input)
        .unwrap()
        .iter()
        .map(|((n, t), outs)| (n.clone(), (*t, outs.clone())))
        .collect()
}

#[aoc(day20, part1)]
fn solve_part1(input: &Task) -> u64 {
    let mut fflops = 0u64;
    let mut conj_masks = [0u64; 64];
    let mut conjs = [0u64; 64];

    let mut names: HashMap<String, usize> = input
        .iter()
        .enumerate()
        .map(|(i, (n, _))| (n.clone(), i))
        .collect();
    names.insert("rx".to_string(), names.len());
    names.insert("output".to_string(), names.len());

    let mut rev_names: Vec<String> = input.iter().map(|(n, _)| n.clone()).collect();
    rev_names.push("rx".to_string());
    rev_names.push("output".to_string());

    let mut nodes: Vec<Node> = input.iter().map(|(_, (t, _))| *t).collect();
    nodes.push(Node::FlipFlop);
    nodes.push(Node::FlipFlop);

    let mut outs: Vec<Vec<usize>> = input
        .iter()
        .map(|(_, (_, outs))| {
            outs.iter()
                .map(|n| names.get(n).unwrap_or_else(|| panic!("Unknown name {}", n)))
                .cloned()
                .collect()
        })
        .collect();
    outs.push(Vec::new());
    outs.push(Vec::new());

    for (i, (_, (_, out))) in input.iter().enumerate() {
        for n in out.iter() {
            let &j = names.get(n).unwrap_or_else(|| panic!("Unknown name {}", n));
            conj_masks[j] |= 1 << i;
        }
    }

    let &bcast = names.get("broadcaster").unwrap();

    let mut his = 0u64;
    let mut los = 0u64;
    let mut work = VecDeque::new();
    for _ in 0..1000 {
        // println!("Button push # {}", i);
        work.push_back((0, bcast, false));
        while let Some((src, dst, lvl)) = work.pop_front() {
            if lvl {
                his += 1;
            } else {
                los += 1;
            }
            // println!(
            //     "{} -{}-> {}",
            //     rev_names[src],
            //     if lvl { "high" } else { "low" },
            //     rev_names[dst]
            // );
            match nodes[dst] {
                Node::Broadcaster => {
                    for &o in outs[dst].iter() {
                        work.push_back((dst, o, lvl));
                    }
                }
                Node::FlipFlop if !lvl => {
                    let mask = 1u64 << dst;
                    fflops = fflops ^ mask;
                    let state = fflops & mask != 0;
                    for &o in outs[dst].iter() {
                        work.push_back((dst, o, state));
                    }
                }
                Node::FlipFlop => {}
                Node::Nand => {
                    if lvl {
                        conjs[dst] |= 1 << src;
                    } else {
                        conjs[dst] &= !(1 << src);
                    }
                    let out = conjs[dst] != conj_masks[dst];
                    for &o in outs[dst].iter() {
                        work.push_back((dst, o, out));
                    }
                }
            }
        }
    }
    his * los
}

#[aoc(day20, part2)]
fn solve_part2(input: &Task) -> u64 {
    let mut fflops = 0u64;
    let mut conj_masks = [0u64; 64];
    let mut conjs = [0u64; 64];

    let mut names: HashMap<String, usize> = input
        .iter()
        .enumerate()
        .map(|(i, (n, _))| (n.clone(), i))
        .collect();
    names.insert("rx".to_string(), names.len());
    names.insert("output".to_string(), names.len());

    let mut rev_names: Vec<String> = input.iter().map(|(n, _)| n.clone()).collect();
    rev_names.push("rx".to_string());
    rev_names.push("output".to_string());

    let mut nodes: Vec<Node> = input.iter().map(|(_, (t, _))| *t).collect();
    nodes.push(Node::FlipFlop);
    nodes.push(Node::FlipFlop);

    let mut outs: Vec<Vec<usize>> = input
        .iter()
        .map(|(_, (_, outs))| {
            outs.iter()
                .map(|n| names.get(n).unwrap_or_else(|| panic!("Unknown name {}", n)))
                .cloned()
                .collect()
        })
        .collect();
    outs.push(Vec::new());
    outs.push(Vec::new());

    for (i, (_, (_, out))) in input.iter().enumerate() {
        for n in out.iter() {
            let &j = names.get(n).unwrap_or_else(|| panic!("Unknown name {}", n));
            conj_masks[j] |= 1 << i;
        }
    }

    let &bcast = names.get("broadcaster").unwrap();

    let mut answer = 1u64;
    let mut terms = vec![
        "dx".to_string(),
        "jh".to_string(),
        "ck".to_string(),
        "cs".to_string(),
    ];
    let mut work = VecDeque::new();
    for i in 0..10000 {
        work.push_back((0, bcast, false));
        while let Some((src, dst, lvl)) = work.pop_front() {
            match nodes[dst] {
                Node::Broadcaster => {
                    for &o in outs[dst].iter() {
                        work.push_back((dst, o, lvl));
                    }
                }
                Node::FlipFlop if !lvl => {
                    let mask = 1u64 << dst;
                    fflops = fflops ^ mask;
                    let state = fflops & mask != 0;
                    for &o in outs[dst].iter() {
                        work.push_back((dst, o, state));
                    }
                }
                Node::FlipFlop => {}
                Node::Nand => {
                    if lvl {
                        conjs[dst] |= 1 << src;
                    } else {
                        conjs[dst] &= !(1 << src);
                    }
                    let out = conjs[dst] != conj_masks[dst];
                    if !out {
                        let name = &rev_names[dst];
                        if terms.contains(name) {
                            // println!("Conj {} activated at {}", name, i + 1);
                            answer *= i + 1;
                            terms.remove(terms.iter().position(|x| x == name).unwrap());
                        }
                    }
                    for &o in outs[dst].iter() {
                        work.push_back((dst, o, out));
                    }
                }
            }
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 32000000);
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 2);
    }

    #[test]
    fn example2() {
        let input = r#"
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 11687500);
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 2);
    }
}
