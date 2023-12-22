use std::collections::HashSet;

use aoc_parse::{parser, prelude::*};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Block {
    xf: usize,
    yf: usize,
    zf: usize,
    xt: usize,
    yt: usize,
    zt: usize,
}

impl Block {
    fn bottom(&self) -> usize {
        self.zf.min(self.zt)
    }

    fn top(&self) -> usize {
        self.zf.max(self.zt)
    }

    fn height(&self) -> usize {
        self.top() - self.bottom() + 1
    }

    fn footprint(&self) -> Vec<(usize, usize)> {
        (self.xf.min(self.xt)..=self.xf.max(self.xt))
            .flat_map(|x| (self.yf.min(self.yt)..=self.yf.max(self.yt)).map(move |y| (x, y)))
            .collect()
    }
}

type Task = Vec<Block>;

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Task {
    let p = parser!(lines(
        xf:usize "," yf:usize "," zf:usize "~" xt:usize "," yt: usize "," zt:usize => Block {xf,yf,zf,xt,yt,zt}
    ));
    p.parse(input).unwrap()
}

#[aoc(day22, part1)]
fn solve_part1(input: &Task) -> usize {
    let xn = input.iter().flat_map(|b| [b.xf, b.xt]).min().unwrap();
    let yn = input.iter().flat_map(|b| [b.yf, b.yt]).min().unwrap();
    assert_eq!(xn, 0);
    assert_eq!(yn, 0);

    let xx = input.iter().flat_map(|b| [b.xf, b.xt]).max().unwrap();
    let yx = input.iter().flat_map(|b| [b.yf, b.yt]).max().unwrap();

    let mut support: Vec<Vec<usize>> = (0..input.len()).map(|_| Vec::new()).collect();
    let mut work: Vec<Vec<(usize, usize)>> = (0..=xx)
        .map(|_| (0..=yx).map(|_| (0, usize::MAX)).collect())
        .collect();

    let mut queue = input.clone();
    queue.sort_by_key(|b| b.bottom());

    let mut bearing = HashSet::new();

    for (i, block) in queue.iter().enumerate() {
        let footprint = block.footprint();
        let level = footprint.iter().map(|(x, y)| work[*x][*y].0).max().unwrap();
        let support: HashSet<usize> = footprint
            .iter()
            .filter_map(|(x, y)| {
                if work[*x][*y].0 == level {
                    Some(work[*x][*y].1)
                } else {
                    None
                }
            })
            .filter(|i| *i != usize::MAX)
            .collect();
        if support.len() == 1 {
            bearing.extend(support);
        }
        for (x, y) in footprint {
            work[x][y] = (level + block.height(), i);
        }
    }

    queue.len() - bearing.len()
}

#[aoc(day22, part2)]
fn solve_part2(input: &Task) -> usize {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 5);
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 2);
    }
}
