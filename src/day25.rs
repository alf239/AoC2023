use std::collections::HashSet;

use aoc_parse::{parser, prelude::*};
use rand::Rng;

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
pub struct Node {
    name: String,
    links: Vec<String>,
}
type Task = Vec<Node>;

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Task {
    let p = parser!(lines(
        name:string(lower+) ": " links:repeat_sep(string(lower+), " ") => Node { name, links }
    ));
    p.parse(input).unwrap()
}

#[aoc(day25, part1)]
fn solve_part1(input: &Task) -> usize {
    let mut nodes = HashSet::new();
    for n in input.iter() {
        nodes.insert(n.name.clone());
        nodes.extend(n.links.iter().cloned());
    }

    let mut nodes: Vec<String> = nodes.iter().cloned().collect();
    nodes.sort();
    let resolve_name = |n: &str| nodes.iter().position(|n1| n == n1).unwrap();

    let mut edges: Vec<(usize, usize)> = Vec::new();
    for node in input.iter() {
        let u = resolve_name(&node.name);
        for link in node.links.iter() {
            let v = resolve_name(link);
            edges.push((u, v));
        }
    }

    let sizes: Vec<usize> = (0..nodes.len()).map(|_| 1).collect();

    let mut rng = rand::thread_rng();
    loop {
        let mut we = edges.clone();
        let mut ws = sizes.clone();

        for _ in 0..ws.len() - 2 {
            let e = rng.gen_range(0..we.len());
            let (u, v) = we.remove(e);

            ws[u] += ws[v];
            ws[v] = 0;

            for edge in we.iter_mut() {
                if edge.0 == v {
                    edge.0 = u;
                }
                if edge.1 == v {
                    edge.1 = u;
                }
            }
            we.retain(|(u, v)| *u != *v);
        }
        if we.len() == 3 {
            return ws[we[0].0] * ws[we[0].1];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"
jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 54);
    }
}
