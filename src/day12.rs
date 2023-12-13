use aoc_parse::{parser, prelude::*};

type Task = Vec<(String, Vec<usize>)>;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Task {
    let p = parser!(lines(
       string(char_of(".#?")+) " " repeat_sep(usize, ",")
    ));
    p.parse(input).unwrap()
}

fn arr(s: &str, jp: &[usize]) -> u64 {
    let vec: Vec<char> = s.chars().collect();
    arrangements(&vec, jp)
}

fn arrangements(s: &[char], jp: &[usize]) -> u64 {
    match (s, jp) {
        ([], []) => 1,
        ([], _) => 0,
        (_, []) if s.iter().all(|&c| ".?".contains(c)) => 1,
        (_, []) => 0,
        (['.', rest @ ..], _) => arrangements(rest, jp),
        (['#', rest @ ..], [nr, tail @ ..])
            if s.len() >= *nr
                && rest[..nr - 1].iter().all(|&c| "?#".contains(c))
                && (s.len() == *nr || ".?".contains(rest[nr - 1])) =>
        {
            if *nr >= rest.len() {
                arrangements(&[], tail)
            } else {
                arrangements(&rest[*nr..], tail)
            }
        }
        (['?', rest @ ..], [nr, tail @ ..])
            if s.len() >= *nr
                && rest[..nr - 1].iter().all(|&c| "?#".contains(c))
                && (s.len() == *nr || ".?".contains(rest[nr - 1])) =>
        {
            let if_skip = arrangements(rest, jp);
            if *nr >= rest.len() {
                arrangements(&[], tail) + if_skip
            } else {
                arrangements(&rest[*nr..], tail) + if_skip
            }
        }
        (['?', rest @ ..], _) => arrangements(rest, jp),
        _ => 0,
    }
}

#[aoc(day12, part1)]
fn solve_part1(input: &Task) -> u64 {
    input.iter().map(|(s, jp)| arr(s, jp)).sum()
}

#[aoc(day12, part2)]
fn solve_part2(input: &Task) -> u64 {
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_arrangements() {
        assert_eq!(arr("???.###", &vec![1, 1, 3]), 1);
        assert_eq!(arr(".??..??...?##.", &vec![1, 1, 3]), 4);
        assert_eq!(arr("?#?#?#?#?#?#?#?", &vec![1, 3, 1, 6]), 1);
        assert_eq!(arr("????.#...#...", &vec![4, 1, 1]), 1);
        assert_eq!(arr("????.######..#####.", &vec![1, 6, 5]), 4);
        assert_eq!(arr("?###????????", &vec![3, 2, 1]), 10);
    }

    #[test]
    fn example1() {
        let input = r#"
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#
            .trim();
        let parsed = input_generator(input);
        let result1 = solve_part1(&parsed);
        assert_eq!(result1, 21);
        let result2 = solve_part2(&parsed);
        assert_eq!(result2, 2);
    }
}
