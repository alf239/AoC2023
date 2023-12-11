use aoc_parse::{parser, prelude::*};

type Task = Vec<Vec<bool>>;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Task {
    let p = parser!(lines(
       star:char_of(".#")+ => star.iter().map(|&c| c == 1).collect()
    ));
    p.parse(input).unwrap()
}

fn solve(input: &Task, scale: i64) -> i64 {
    let galaxies: Vec<(usize, usize)> = input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(j, &c)| if c { Some((i, j)) } else { None })
        })
        .collect();
    let mut dy = vec![0; input.len()];
    let mut corr: i64 = 0;
    for i in 0..dy.len() {
        if input[i].iter().all(|c| !c) {
            corr += scale - 1;
        }
        dy[i] = corr + i as i64;
    }
    let mut dx = vec![0; input[0].len()];
    corr = 0;
    for j in 0..dx.len() {
        if input.iter().all(|row| !row[j]) {
            corr += scale - 1;
        }
        dx[j] = corr + j as i64;
    }
    let mut result = 0;
    for i in 0..galaxies.len() {
        let (y1, x1) = galaxies[i];
        let x11 = dx[x1];
        let y11 = dy[y1];
        for j in i + 1..galaxies.len() {
            let (y2, x2) = galaxies[j];
            let x21 = dx[x2];
            let y21 = dy[y2];

            let d = (x21 - x11).abs() + (y21 - y11).abs();
            result += d;
        }
    }

    result
}

#[aoc(day11, part1)]
fn solve_part1(input: &Task) -> i64 {
    solve(input, 2)
}

#[aoc(day11, part2)]
fn solve_part2(input: &Task) -> i64 {
    solve(input, 1_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve(&parsed, 2);
        assert_eq!(result1, 374);
        let result2 = solve(&parsed, 10);
        assert_eq!(result2, 1030);
        let result3 = solve(&parsed, 100);
        assert_eq!(result3, 8410);
    }
}
