use std::{collections::HashMap, ops::Range};

use aoc_parse::{parser, prelude::*};

#[derive(Debug, Clone)]
pub enum Rule {
    Gt(usize, usize, String),
    Lt(usize, usize, String),
    Jump(String),
}

pub struct Task {
    workflows: HashMap<String, Vec<Rule>>,
    parts: Vec<[usize; 4]>,
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Task {
    use Rule::*;
    let p = parser!(
        workflows:lines(
            string(lower+) "{"
            repeat_sep(
            {
                name:string(alpha+) => Jump(name),
                name:char_of("xmas") ">" limit:usize ":" target:string(alpha+) => Gt(name, limit, target),
                name:char_of("xmas") "<" limit:usize ":" target:string(alpha+) => Lt(name, limit, target),
            }, ",") "}")
        line("")
        parts:lines("{x=" x:usize ",m=" m:usize ",a=" a:usize ",s=" s:usize "}" => [x,m,a,s])
        => Task {
            workflows: workflows.iter().cloned().collect(),
            parts,
        }
    );
    p.parse(input).unwrap()
}

fn accepted(wfs: &HashMap<String, Vec<Rule>>, wf: &str, part: &[usize]) -> bool {
    if wf == "A" {
        return true;
    }
    if wf == "R" {
        return false;
    }
    match wfs.get(wf) {
        Some(wf) => {
            for rule in wf {
                use Rule::*;

                match rule {
                    Gt(name, lim, go) if part[*name] > *lim => return accepted(wfs, go, part),
                    Lt(name, lim, go) if part[*name] < *lim => return accepted(wfs, go, part),
                    Jump(go) => return accepted(wfs, go, part),
                    _ => {}
                }
            }
            panic!("Out of rules!");
        }
        None => panic!("Don't know rule {}", wf),
    }
}

#[aoc(day19, part1)]
fn solve_part1(input: &Task) -> i64 {
    input
        .parts
        .iter()
        .filter(|p| accepted(&input.workflows, "in", *p))
        .flat_map(|p| p.iter().map(|x| *x as i64))
        .sum()
}

type Parts = [Range<usize>; 4];

fn intersect(a: Range<usize>, b: Range<usize>) -> Option<Range<usize>> {
    let start = std::cmp::max(a.start, b.start);
    let end = std::cmp::min(a.end, b.end);

    if start < end {
        Some(start..end)
    } else {
        None
    }
}

fn cut_at(p: Parts, name: usize, lim: usize) -> (Option<Parts>, Option<Parts>) {
    let lo = intersect(p[name].clone(), 1..lim + 1);
    let hi = intersect(p[name].clone(), lim + 1..4001);
    (
        lo.map(|r| {
            let mut res = p.clone();
            res[name] = r;
            res
        }),
        hi.map(|r| {
            let mut res = p.clone();
            res[name] = r;
            res
        }),
    )
}

fn size(p: &[Range<usize>]) -> i64 {
    p.iter().map(|x| x.len() as i64).product()
}

fn collect_accepted(
    wfs: &HashMap<String, Vec<Rule>>,
    wf: &str,
    candidate: &Parts,
    result: &mut Vec<Parts>,
) {
    let mut rem = candidate.clone();
    if wf == "A" {
        result.push(rem);
        return;
    }
    if wf == "R" {
        return;
    }
    match wfs.get(wf) {
        Some(wf) => {
            for rule in wf {
                use Rule::*;

                match rule {
                    Gt(name, lim, go) => {
                        let (lo, hi) = cut_at(rem, *name, *lim);
                        if let Some(h) = hi {
                            collect_accepted(wfs, go, &h, result);
                        }
                        match lo {
                            Some(l) => rem = l,
                            None => return,
                        };
                    }
                    Lt(name, lim, go) => {
                        let (lo, hi) = cut_at(rem, *name, *lim - 1);
                        if let Some(l) = lo {
                            collect_accepted(wfs, go, &l, result);
                        }
                        match hi {
                            Some(h) => rem = h,
                            None => return,
                        };
                    }
                    Jump(go) => {
                        collect_accepted(wfs, go, &rem, result);
                        return;
                    }
                }
            }
            panic!("Out of rules!");
        }
        None => panic!("Don't know rule {}", wf),
    }
}

#[aoc(day19, part2)]
fn solve_part2(input: &Task) -> i64 {
    let mut accepted: Vec<Parts> = Vec::new();
    let seed = [1..4001, 1..4001, 1..4001, 1..4001];
    collect_accepted(&input.workflows, "in", &seed, &mut accepted);
    accepted.iter().map(|rs| size(rs)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 19114);
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 167409079868000);
    }
}
