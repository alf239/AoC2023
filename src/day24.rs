use aoc_parse::{parser, prelude::*};

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct Stone {
    x: i64,
    y: i64,
    z: i64,
    dx: i64,
    dy: i64,
    dz: i64,
}
type Task = Vec<Stone>;

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Task {
    let p = parser!(lines(
        x:i64 "," " "+ y:i64 "," " "+ z:i64 " @" " "+ dx:i64 "," " "+ dy:i64 "," " "+ dz:i64 => Stone { x, y,z,dx,dy,dz}
    ));
    p.parse(input).unwrap()
}

fn intersect(a: &Stone, b: &Stone, from: i64, to: i64) -> bool {
    let bx = b.x - a.x;
    let by = b.y - a.y;
    let det = a.dx * b.dy - a.dy * b.dx;
    if det == 0 {
        println!("{:?} and {:?} are parallel", a, b);
        return false;
    }
    let t2 = bx * a.dy - by * a.dx;
    if t2.signum() != det.signum() {
        return false;
    }

    let t1 = bx * b.dy - by * b.dx;
    if t1.signum() != det.signum() {
        return false;
    }

    let t = t1 as f64 / det as f64;
    let px = a.x as f64 + t * a.dx as f64;
    let py = a.y as f64 + t * a.dy as f64;

    px >= from as f64 && px <= to as f64 && py >= from as f64 && py <= to as f64
}

fn solve1(input: &Task, from: i64, to: i64) -> usize {
    (0..input.len() - 1)
        .flat_map(|i| (i + 1..input.len()).map(move |j| (i, j)))
        .filter(|&(i, j)| intersect(&input[i], &input[j], from, to))
        .count()
}

#[aoc(day24, part1)]
fn solve_part1(input: &Task) -> usize {
    let orig_from = 200_000_000_000_000;
    let orig_to = 400_000_000_000_000;
    solve1(&input, orig_from, orig_to)
}

#[aoc(day24, part2)]
fn solve_part2(input: &Task) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve1(&parsed, 7, 27);
        assert_eq!(result1, 2);
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 0);
    }
}
