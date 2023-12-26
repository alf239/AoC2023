use aoc_parse::{parser, prelude::*};
use nalgebra::{Matrix4, Vector4};

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

fn dist(a: &Stone, b: &Stone, t: i64) -> i64 {
    (a.x + a.dx * t - b.x - b.dx * (t + 1)).abs()
        + (a.y + a.dy * t - b.y - b.dy * (t + 1)).abs()
        + (a.z + a.dz * t - b.z - b.dz * (t + 1)).abs()
}

#[aoc(day24, part2)]
fn solve_part2(input: &Task) -> usize {
    let s1 = input[0];
    let s2 = input[1];
    let s3 = input[2];
    let s4 = input[3];
    let s5 = input[4];

    let a = Matrix4::new(
        (s2.dy - s1.dy) as f64,
        (s2.y - s1.y) as f64,
        (s1.dx - s2.dx) as f64,
        (s1.x - s2.x) as f64,
        (s3.dy - s2.dy) as f64,
        (s3.y - s2.y) as f64,
        (s2.dx - s3.dx) as f64,
        (s2.x - s3.x) as f64,
        (s4.dy - s3.dy) as f64,
        (s4.y - s3.y) as f64,
        (s3.dx - s4.dx) as f64,
        (s3.x - s4.x) as f64,
        (s5.dy - s4.dy) as f64,
        (s5.y - s4.y) as f64,
        (s4.dx - s5.dx) as f64,
        (s4.x - s5.x) as f64,
    );
    let b = Vector4::new(
        (s1.x * s1.dy - s1.y * s1.dx - s2.x * s2.dy + s2.y * s2.dx) as f64,
        (s2.x * s2.dy - s2.y * s2.dx - s3.x * s3.dy + s3.y * s3.dx) as f64,
        (s3.x * s3.dy - s3.y * s3.dx - s4.x * s4.dy + s4.y * s4.dx) as f64,
        (s4.x * s4.dy - s4.y * s4.dx - s5.x * s5.dy + s5.y * s5.dx) as f64,
    );
    let decomp = a.lu();
    let res1 = decomp.solve(&b).unwrap();

    let x = res1[0];
    let y = res1[2];

    let a = Matrix4::new(
        (s2.dz - s1.dz) as f64,
        (s2.z - s1.z) as f64,
        (s1.dx - s2.dx) as f64,
        (s1.x - s2.x) as f64,
        (s3.dz - s2.dz) as f64,
        (s3.z - s2.z) as f64,
        (s2.dx - s3.dx) as f64,
        (s2.x - s3.x) as f64,
        (s4.dz - s3.dz) as f64,
        (s4.z - s3.z) as f64,
        (s3.dx - s4.dx) as f64,
        (s3.x - s4.x) as f64,
        (s5.dz - s4.dz) as f64,
        (s5.z - s4.z) as f64,
        (s4.dx - s5.dx) as f64,
        (s4.x - s5.x) as f64,
    );
    let b = Vector4::new(
        (s1.x * s1.dz - s1.z * s1.dx - s2.x * s2.dz + s2.z * s2.dx) as f64,
        (s2.x * s2.dz - s2.z * s2.dx - s3.x * s3.dz + s3.z * s3.dx) as f64,
        (s3.x * s3.dz - s3.z * s3.dx - s4.x * s4.dz + s4.z * s4.dx) as f64,
        (s4.x * s4.dz - s4.z * s4.dx - s5.x * s5.dz + s5.z * s5.dx) as f64,
    );
    let decomp = a.lu();
    let res2 = decomp.solve(&b).unwrap();
    let z = res2[2];


    (-x - y - z).round() as usize
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
        assert_eq!(result2, 47);
    }
}
