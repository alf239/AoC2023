use std::collections::HashMap;

const RADIX: u32 = 10;
type Coords = (i32, i32);

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> (HashMap<Coords, char>, Vec<(u32, Coords)>) {
    let mut symbols: HashMap<Coords, char> = HashMap::new();
    let mut numbers: Vec<(u32, Coords)> = Vec::new();

    input.lines().enumerate().for_each(|(i, l)| {
        let mut acc: u32 = 0;
        let mut pos: Coords = (0, 0);
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
    (symbols, numbers)
}

fn neighbours(number: u32, pos: Coords, symbols: &HashMap<Coords, char>) -> Vec<Coords> {
    let (y, x) = pos;
    let len = number.to_string().len() as i32;
    let lt = x - 1;
    let rt = x + len;
    let mut nbrs: Vec<Coords> = (lt..=rt)
        .flat_map(|xn| [(y - 1, xn), (y + 1, xn)])
        .filter_map(|pos| {
            if symbols.contains_key(&pos) {
                Some(pos)
            } else {
                None
            }
        })
        .collect();
    if symbols.contains_key(&(y, lt)) {
        nbrs.push((y, lt));
    }
    if symbols.contains_key(&(y, rt)) {
        nbrs.push((y, rt));
    }
    nbrs
}

#[aoc(day3, part1)]
pub fn solve_part1((symbols, numbers): &(HashMap<Coords, char>, Vec<(u32, Coords)>)) -> u32 {
    numbers
        .iter()
        .filter_map(|(number, pos)| {
            if neighbours(*number, *pos, symbols).is_empty() {
                None
            } else {
                Some(number)
            }
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2((symbols, numbers): &(HashMap<Coords, char>, Vec<(u32, Coords)>)) -> u64 {
    let stars: HashMap<Coords, char> = symbols
        .iter()
        .filter(|(_, v)| **v == '*')
        .map(|(&key, &value)| (key, value))
        .collect();

    let mut gears: HashMap<Coords, Vec<u32>> = stars.iter().map(|(&key, _)| (key, Vec::new())).collect();

    numbers.iter().for_each(|(number, pos)| {
        let nbrs = neighbours(*number, *pos, &stars);
        nbrs.iter().for_each(|p| {
            gears.get_mut(p).unwrap().push(*number);
        })
    });
    let good_gears: Vec<(u64, u64)> = gears
        .values()
        .filter_map(|nrs| {
            if nrs.len() == 2 {
                Some((nrs[0] as u64, nrs[1] as u64))
            } else {
                None
            }
        })
        .collect();
    good_gears.iter().map(|(a, b)| a * b).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_example() {
        let example = "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598.."
            .trim();
        let input = input_generator(example);
        let result = solve_part2(&input);
        assert_eq!(result, 467835);
    }

    // https://www.reddit.com/r/adventofcode/comments/189q9wv/2023_day_3_another_sample_grid_to_use/
    #[test]
    fn reddit() {
        let example = "
            12.......*..
            +.........34
            .......-12..
            ..78........
            ..*....60...
            78..........
            .......23...
            ....90*12...
            ............
            2.2......12.
            .*.........*
            1.1.......56"
            .trim();
        let input = input_generator(example);
        let result1 = solve_part1(&input);
        assert_eq!(result1, 413);
        let result2 = solve_part2(&input);
        assert_eq!(result2, 6756);
    }
    
    // https://www.reddit.com/r/adventofcode/comments/189q9wv/comment/kbsrno0/
    #[test]
    fn reddit2() {
        let example = "
            .......5......
            ..7*..*.......
            ...*13*.......
            .......15....."
            .trim();
        let input = input_generator(example);
        let result2 = solve_part2(&input);
        assert_eq!(result2, 442);
    }
}
