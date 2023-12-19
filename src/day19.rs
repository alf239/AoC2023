use std::collections::HashMap;

use aoc_parse::{parser, prelude::*};

#[derive(Debug, Clone)]
pub enum Rule {
    Gt(String, usize, String),
    Lt(String, usize, String),
    Jump(String),
    Accept,
    Reject,
}

#[derive(Debug, Clone)]
pub struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

pub struct Task {
    workflows: HashMap<String, Workflow>,
    parts: Vec<HashMap<String, usize>>,
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Task {
    use Rule::*;
    let p = parser!(
            workflows:lines(name: string(lower+) "{" rules:repeat_sep(
                {
                    "A" => Accept,
                    "R" => Reject,
                    name:string(lower+) => Jump(name),
                    name:string(lower+) ">" limit:usize ":" target:string(alpha+) => Gt(name, limit, target),
                    name:string(lower+) "<" limit:usize ":" target:string(alpha+) => Lt(name, limit, target),
    }, ",") "}" => (name.clone(), Workflow {name, rules}))
            line("")
            parts:lines("{" repeat_sep(string(lower+) "=" usize, ",") "}")
            => Task {
                workflows: workflows.iter().cloned().collect(),
                parts: parts.iter().map(|rs| rs.iter().cloned().collect()).collect(),
            }
        );
    p.parse(input).unwrap()
}

fn accepted(rules: &HashMap<String, Workflow>, rule: &str, part: &HashMap<String, usize>) -> bool {
    if rule == "A" {
        return true;
    }
    if rule == "R" {
        return false;
    }
    match rules.get(rule) {
        Some(wf) => {
            for rule in &wf.rules {
                use Rule::*;

                match rule {
                    Gt(name, lim, go) if *part.get(name).unwrap() > *lim => {
                        return accepted(rules, &go, part);
                    }
                    Lt(name, lim, go) if *part.get(name).unwrap() < *lim => {
                        return accepted(rules, &go, part);
                    }
                    Jump(go) => {
                        return accepted(rules, &go, part);
                    }
                    Accept => return true,
                    Reject => return false,
                    _ => {}
                }
            }
            panic!("Out of rules!");
        }
        None => panic!("Don't know rule {}", rule),
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

#[aoc(day19, part2)]
fn solve_part2(input: &Task) -> i64 {
    2
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
        assert_eq!(result2, 2);
    }
}
