use std::{collections::HashSet, iter::repeat};

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

fn arr2(s: &str, jp: &[usize]) -> u64 {
    let s = repeat(s).take(5).collect::<Vec<_>>().join("?");
    let vec: Vec<char> = s.chars().collect();
    let jp1: Vec<usize> = jp.iter().cycle().take(jp.len() * 5).cloned().collect();
    arrangements(&vec, &jp1)
}

fn arrangements(s1: &[char], jp: &[usize]) -> u64 {
    let mut s2 = Vec::from(s1);
    s2.insert(0, '.');
    let s: Vec<char> = s2.iter().cloned().collect(); 
    let empty = HashSet::from([0]);
    let mut ls: Vec<HashSet<usize>> = (0..s.len() + 1).map(|_| empty.clone()).collect();
    let mut cur = empty.clone();
    for (j, &c) in s.iter().enumerate() {
        match c {
            '#' => {
                cur = cur.iter().map(|x| x + 1).collect();
            }
            '.' => {
                cur = empty.clone();
            }
            '?' => {
                cur = cur.iter().map(|x| x + 1).collect();
                cur.insert(0);
            }
            _ => panic!("Unknown character {}", c),
        }
        ls[j + 1] = cur.clone();
    }
    let mut dp = vec![vec![0u64; s.len() + 1]; jp.len() + 1];
    dp[0][0] = 1;
    for i in 1..dp.len() {
        dp[i][0] = 0;
    }
    let mut saw_hash = false;
    for j in 1..dp[0].len() {
        saw_hash |= s[j - 1] == '#';
        dp[0][j] = if saw_hash { 0 } else { 1 };
    }
    for i in 1..dp.len() {
        for j in 1..dp[i].len() {
            let c = s[j - 1];
            match c {
                '.' => dp[i][j] = dp[i][j - 1],
                '#' => {
                    let n = jp[i - 1];
                    let consume_nr = if ls[j].contains(&n) {
                        dp[i - 1][j - n - 1]
                    } else {
                        0
                    };
                    dp[i][j] = consume_nr;
                }
                '?' => {
                    let n = jp[i - 1];
                    let consume_nr = if ls[j].contains(&n) {
                        dp[i - 1][j - n - 1]
                    } else {
                        0
                    };
                    dp[i][j] = consume_nr + dp[i][j - 1];
                }
                _ => panic!("Unknown character {}", c),
            }
        }
    }
    dp[jp.len()][s.len()]
}

#[aoc(day12, part1)]
fn solve_part1(input: &Task) -> u64 {
    input.iter().map(|(s, jp)| arr(s, jp)).sum()
}

#[aoc(day12, part2)]
fn solve_part2(input: &Task) -> u64 {
    input.iter().map(|(s, jp)| arr2(s, jp)).sum()
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
    fn known_arrangements2() {
        assert_eq!(arr2("???.###", &vec![1, 1, 3]), 1);
        assert_eq!(arr2(".??..??...?##.", &vec![1, 1, 3]), 16384);
        assert_eq!(arr2("?#?#?#?#?#?#?#?", &vec![1, 3, 1, 6]), 1);
        assert_eq!(arr2("????.#...#...", &vec![4, 1, 1]), 16);
        assert_eq!(arr2("????.######..#####.", &vec![1, 6, 5]), 2500);
        assert_eq!(arr2("?###????????", &vec![3, 2, 1]), 506250);
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
        assert_eq!(result2, 525152);
    }
}
