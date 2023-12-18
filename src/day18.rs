use aoc_parse::{parser, prelude::*};

pub struct Cmd {
    dir: usize,
    len: usize,
    rgb: u32,
}

type Task = Vec<Cmd>;

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Task {
    let p = parser!(
        lines(
            dir:char_of("RDLU") " " len:usize " (#" rgb:u32_hex ")" => Cmd { dir, len, rgb }
        )
    );
    p.parse(input).unwrap()
}

fn area<I>(cmds: I) -> i64
where
    I: Iterator<Item = (usize, usize)>,
{
    let mut prev = (0, 0);
    let mut area = 0i64;
    let mut perimeter = 1i64;
    for (dir, len) in cmds {
        let len = len as i64;
        let next = match dir {
            0 => (prev.0, prev.1 + len),
            1 => (prev.0 + len, prev.1),
            2 => (prev.0, prev.1 - len),
            3 => (prev.0 - len, prev.1),
            _ => panic!("Holy shit, what's {}", dir),
        };
        area += (prev.0 - next.0) * (prev.1 + next.1);
        perimeter += len;
        prev = next;
    }
    (area.abs() + perimeter) / 2 + 1
}

#[aoc(day18, part1)]
fn solve_part1(input: &Task) -> i64 {
    let cmds = input.iter().map(|cmd| (cmd.dir, cmd.len));
    area(cmds)
}

#[aoc(day18, part2)]
fn solve_part2(input: &Task) -> i64 {
    let cmds = input
        .iter()
        .map(|cmd| ((cmd.rgb % 16) as usize, (cmd.rgb / 16) as usize));
    area(cmds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = r#"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 62);
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 952408144115);
    }
}
