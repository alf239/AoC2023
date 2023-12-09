use aoc_parse::{parser, prelude::*};

pub struct Task {
    seqs: Vec<Vec<i32>>,
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Task {
    let p = parser!(seqs:lines(repeat_sep(i32, " ")) => Task { seqs });
    p.parse(input).unwrap()
}

fn extrapolate(xs: &Vec<i32>) -> i32 {
    if xs.iter().all(|&x| x == 0) {
        0
    } else {
        let row: Vec<i32> = xs.windows(2).map(|pair| pair[1] - pair[0]).collect();
        let prev = extrapolate(&row);
        xs[xs.len() - 1] + prev
    }
}

#[aoc(day9, part1)]
fn solve_part1(input: &Task) -> i32 {
    input.seqs.iter().map(extrapolate).sum()
}

#[aoc(day9, part2)]
fn solve_part2(input: &Task) -> i32 {
    let reversed = Task {
        seqs: input
            .seqs
            .iter()
            .map(|xs| xs.iter().copied().rev().collect())
            .collect(),
    };
    solve_part1(&reversed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 114);
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 2);
    }
}
