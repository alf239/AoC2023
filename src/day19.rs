use std::{collections::HashMap, ops::Range};

use aoc_parse::{parser, prelude::*};

#[derive(Debug, Clone)]
pub enum Rule {
    Gt(String, usize, String),
    Lt(String, usize, String),
    Jump(String),
}

pub struct Task {
    workflows: HashMap<String, Vec<Rule>>,
    parts: Vec<HashMap<String, usize>>,
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
                name:string(lower+) ">" limit:usize ":" target:string(alpha+) => Gt(name, limit, target),
                name:string(lower+) "<" limit:usize ":" target:string(alpha+) => Lt(name, limit, target),
            }, ",") "}")
        line("")
        parts:lines("{" repeat_sep(string(lower+) "=" usize, ",") "}")
        => Task {
            workflows: workflows.iter().cloned().collect(),
            parts: parts.iter().map(|rs| rs.iter().cloned().collect()).collect(),
        }
    );
    p.parse(input).unwrap()
}

fn accepted(wfs: &HashMap<String, Vec<Rule>>, wf: &str, part: &HashMap<String, usize>) -> bool {
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
                    Gt(name, lim, go) if *part.get(name).unwrap() > *lim => {
                        return accepted(wfs, go, part);
                    }
                    Lt(name, lim, go) if *part.get(name).unwrap() < *lim => {
                        return accepted(wfs, go, part);
                    }
                    Jump(go) => {
                        return accepted(wfs, go, part);
                    }
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
        .filter(|p| accepted(&input.workflows, "in", p))
        .flat_map(|p| p.values().map(|&x| x as i64))
        .sum()
}

#[derive(Debug, Clone)]
struct Ratings {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

fn intersect_ranges(range1: Range<usize>, range2: Range<usize>) -> Option<Range<usize>> {
    let start = std::cmp::max(range1.start, range2.start);
    let end = std::cmp::min(range1.end, range2.end);

    if start < end {
        Some(start..end)
    } else {
        None
    }
}

impl Ratings {
    fn cut_gt(&self, name: &str, lim: usize) -> (Option<Ratings>, Option<Ratings>) {
        let low = 1..lim + 1;
        let high = lim + 1..4001;
        match name {
            "x" => (
                intersect_ranges(self.x.clone(), low).map(|n| Ratings {
                    x: n,
                    ..self.clone()
                }),
                intersect_ranges(self.x.clone(), high).map(|n| Ratings {
                    x: n,
                    ..self.clone()
                }),
            ),
            "m" => (
                intersect_ranges(self.m.clone(), low).map(|n| Ratings {
                    m: n,
                    ..self.clone()
                }),
                intersect_ranges(self.m.clone(), high).map(|n| Ratings {
                    m: n,
                    ..self.clone()
                }),
            ),
            "a" => (
                intersect_ranges(self.a.clone(), low).map(|n| Ratings {
                    a: n,
                    ..self.clone()
                }),
                intersect_ranges(self.a.clone(), high).map(|n| Ratings {
                    a: n,
                    ..self.clone()
                }),
            ),
            "s" => (
                intersect_ranges(self.s.clone(), low).map(|n| Ratings {
                    s: n,
                    ..self.clone()
                }),
                intersect_ranges(self.s.clone(), high).map(|n| Ratings {
                    s: n,
                    ..self.clone()
                }),
            ),
            _ => panic!("Unknown name {}", name),
        }
    }

    fn len(&self) -> i64 {
        self.x.len() as i64 * self.m.len() as i64 * self.a.len() as i64 * self.s.len() as i64
    }
}

fn collect_accepted(
    wfs: &HashMap<String, Vec<Rule>>,
    wf: &str,
    candidate: &Ratings,
    result: &mut Vec<Ratings>,
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
                        let (lo, hi) = rem.cut_gt(name, *lim);
                        if let Some(h) = hi {
                            collect_accepted(wfs, go, &h, result);
                        }
                        match lo {
                            Some(l) => rem = l,
                            None => return,
                        };
                    }
                    Lt(name, lim, go) => {
                        let (lo, hi) = rem.cut_gt(name, *lim - 1);
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
    let mut accepted: Vec<Ratings> = Vec::new();
    let seed = Ratings {
        x: 1..4001,
        m: 1..4001,
        a: 1..4001,
        s: 1..4001,
    };
    collect_accepted(&input.workflows, "in", &seed, &mut accepted);
    accepted.iter().map(|rs| rs.len()).sum()
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
