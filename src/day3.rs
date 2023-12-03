use std::collections::HashMap;

const RADIX: u32 = 10;

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut symbols: HashMap<(i32, i32), char> = HashMap::new();
    let mut numbers: Vec<(u32, (i32, i32))> = Vec::new();

    input.lines().enumerate().for_each(|(i, l)| {
        let mut acc: u32 = 0;
        let mut pos: (i32, i32) = (0, 0);
        l.trim().chars().enumerate().for_each(|(j, c)| {
            if c.is_digit(RADIX) {
                if acc == 0 {
                    pos = (i as i32, j as i32);
                }
                acc = acc * RADIX + c.to_digit(RADIX).unwrap();
            } else {
                if acc != 0 {
                    numbers.push((acc, pos));
                }
                acc = 0;
                if c != '.' {
                    symbols.insert((i as i32, j as i32), c);
                }
            }
        });
        if acc != 0 {
            numbers.push((acc, pos));
        }
    });

    numbers.iter().filter_map(|(number, (y, x))| {
        let len= number.to_string().len() as i32;
        let lt = x - 1;
        let rt = x + len;
        let vert = (lt..=rt).flat_map(|xn| [(y - 1, xn), (y + 1, xn)]).any(|pos| symbols.contains_key(&pos));
        let sides = symbols.contains_key(&(*y, lt)) || symbols.contains_key(&(*y, rt));
        if vert || sides {
            Some(number)
        } else {
            None
        }
    }).sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> usize {
    2
}

