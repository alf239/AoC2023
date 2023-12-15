use aoc_parse::{parser, prelude::*};

fn hash(s: &str) -> usize {
    let mut res: usize = 0;
    for &c in s.as_bytes() {
        res += c as usize;
        res *= 17;
        res = res % 256;
    }
    res
}

#[aoc(day15, part1)]
fn solve_part1(input: &str) -> usize {
    input.trim().split(",").map(|s| hash(s)).sum()
}

#[derive(PartialEq, Eq, Debug)]
enum Cmd {
    Rm(String),
    Put(String, usize),
}

fn parse(s: &str) -> Cmd {
    use Cmd::*;
    let p = parser!({
        name:string(lower+) "=" val:usize => Put(name, val),
        name:string(lower+) "-" => Rm(name),
    });
    p.parse(s).unwrap()
}

#[aoc(day15, part2)]
fn solve_part2(input: &str) -> usize {
    let program: Vec<Cmd> = input.trim().split(",").map(|s| parse(s)).collect();
    let mut hm: Vec<Vec<(String, usize)>> = (0..256).map(|_| Vec::new()).collect();
    for cmd in program {
        match cmd {
            Cmd::Rm(name) => {
                let h = hash(&name);
                let i = hm[h].iter().position(|(s, _)| *s == name);
                match i {
                    Some(ix) => {
                        hm[h].remove(ix);
                    }
                    None => {}
                }
            }
            Cmd::Put(name, value) => {
                let h = hash(&name);
                let i = hm[h].iter().position(|(s, _)| *s == name);
                match i {
                    Some(ix) => hm[h][ix] = (name, value),
                    None => hm[h].push((name, value)),
                }
            }
        }
    }
    hm.iter()
        .enumerate()
        .flat_map(|(i, bucket)| {
            bucket
                .iter()
                .enumerate()
                .map(move |(j, (_, v))| (i + 1) * (j + 1) * v)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn example1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7\n\n";
        let result1 = solve_part1(&input);
        assert_eq!(result1, 1320);
        let result2 = solve_part2(&input);
        assert_eq!(result2, 145);
    }
}
