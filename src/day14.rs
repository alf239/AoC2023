use aoc_parse::{parser, prelude::*};

type Task = Vec<Vec<usize>>;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Task {
    let p = parser!(
        lines( char_of(".#O")+)
    );
    p.parse(input).unwrap()
}

#[aoc(day14, part1)]
fn solve_part1(input: &Task) -> usize {
    let mut work: Vec<Vec<usize>> = input.into_iter().map(|r| vec![0; r.len()]).collect();
    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 2 {
                let mut k = i;
                while k > 0 && work[k - 1][j] == 0 {
                    k -= 1;
                }
                work[k][j] = 2;
            } else {
                work[i][j] = *c;
            }
        }
    }
    let n = work.len();
    let mut weight = 0;
    for (i, row) in work.iter().enumerate() {
        for (_, c) in row.iter().enumerate() {
            if *c == 2 {
                weight += n - i;
            }
        }
    }
    weight
}

#[aoc(day14, part2)]
fn solve_part2(input: &Task) -> usize {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 136);
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 2);
    }
}
