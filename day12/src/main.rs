use std::str;

#[derive(PartialEq, Debug)]
struct Row<'a> {
    springs: &'a str,
    groups: Vec<usize>,
}

#[derive(PartialEq, Debug)]
struct PuzzleInput<'a> {
    rows: Vec<Row<'a>>,
}

fn parse_row(line: &str) -> Row {
    let mut parts = line.split_whitespace();
    let springs = parts.next().unwrap();
    let records = parts.next().unwrap();
    let groups = records
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    Row { springs, groups }
}

fn parse_input(input: &str) -> PuzzleInput {
    let rows = input.lines().map(parse_row).collect();
    PuzzleInput { rows }
}

fn arrangements(parent_springs: &[u8], offset: usize, groups: &[usize]) -> usize {
    let mut springs = Vec::from(parent_springs);
    // gather matches
    let n = groups[0];
    //println!("{} {} {:?}", str::from_utf8(springs.as_slice()).unwrap(), offset, groups);
    let check_next = |springs: &[u8], i: usize| -> usize {
        if groups.len() > 1 {
            arrangements(springs, i, &groups[1..])
        } else {
            println!("{} [DONE]", str::from_utf8(springs).unwrap());
            1
        }
    };
    let mut arr = 0;
    let pattern_matches = |springs: &[u8], start: usize| -> Option<(Vec<u8>, usize)> {
        let mut new_springs = Vec::from(springs);
        if start + n > springs.len() {
            // not enough space for this group
            return None;
        }
        let leftover_damaged = groups.len()-1 + groups.iter().sum::<usize>();
        if leftover_damaged + start > springs.len() {
            // not enough space for other groups
            return None;
        }
        // there is still space for the group
        let mut ok = true;
        for j in 0..n {
            if springs[start + j] == b'.' {
                // the group finished
                ok = false;
                break;
            }else{
                new_springs[start + j] = b'#';
            }
        }
        if ok {
            // the group can be full of '#'
            if springs.len() > start + n {
                // it's not over yet. There need to be a '.' after the group
                if springs[start + n] != b'#' {
                    // there is no '#' after the group
                    new_springs[start + n] = b'.';
                    Some((new_springs, start + n + 1))
                }else{
                    // there is a '#' which means it's connecting with another group (bad)
                    None
                }
            } else {
                // it's at the end of the springs list. finished
                Some((new_springs, start + n))
            }
        }else{
            // pattern doesn't match
            None
        }
    };
    for (i, c) in parent_springs.iter().enumerate().skip(offset) {
        match c {
            b'#' => {
                if let Some((filled_springs, next_starting_index)) = pattern_matches(springs.as_slice(), i) {
                    arr += check_next(filled_springs.as_slice(), next_starting_index);
                }
                break;
            }
            b'.' => {
                continue;
            }
            b'?' => {
                if let Some((filled_springs, next_starting_index)) = pattern_matches(springs.as_slice(), i) {
                    arr += check_next(filled_springs.as_slice(), next_starting_index);
                }
                springs[i] = b'.';
            }
            _ => panic!("invalid character {c}"),
        }
    }
    arr
}

fn solve_part1(input: &PuzzleInput) -> usize {
    input
        .rows
        .iter()
        .map(|r| {
            println!("{} {:?}", str::from_utf8(r.springs.as_bytes()).unwrap(), &r.groups);
            arrangements(r.springs.as_bytes(), 0, &r.groups)
        })
        .sum()
}

fn solve_part2(input: &PuzzleInput) -> usize {
    0
}

fn main() {
    let input = include_str!("../input");
    let input = parse_input(input);

    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
.?????...? 1,1,1
#????????.#?#?????? 2,1,1,5,1
";

    macro_rules! example_parsed {
        () => {
            PuzzleInput {
                rows: vec![
                    Row {
                        springs: "???.###",
                        groups: vec![1, 1, 3],
                    },
                    Row {
                        springs: ".??..??...?##.",
                        groups: vec![1, 1, 3],
                    },
                    Row {
                        springs: "?#?#?#?#?#?#?#?",
                        groups: vec![1, 3, 1, 6],
                    },
                    Row {
                        springs: "????.#...#...",
                        groups: vec![4, 1, 1],
                    },
                    Row {
                        springs: "????.######..#####.",
                        groups: vec![1, 6, 5],
                    },
                    Row {
                        springs: "?###????????",
                        groups: vec![3, 2, 1],
                    },
                    Row {
                        springs: ".?????...?",
                        groups: vec![1,1,1],
                    },
                    Row {
                        springs: "#????????.#?#??????",
                        groups: vec![2,1,1,5,1],
                    },
                ],
            }
        };
    }

    #[test]
    fn test_parsing() {
        let input = example_parsed!();
        assert_eq!(parse_input(&EXAMPLE), input);
    }

    #[test]
    fn test_arrangements() {
        let input = example_parsed!();
        let arr = |i: usize| arrangements(&input.rows[i].springs.as_bytes(),0, &input.rows[i].groups);
        assert_eq!(arr(0), 1);
        assert_eq!(arr(1), 4);
        assert_eq!(arr(2), 1);
        assert_eq!(arr(3), 1);
        assert_eq!(arr(4), 4);
        assert_eq!(arr(5), 10);
        assert_eq!(arr(6), 7);
        assert_eq!(arr(7), 36);
    }

    #[test]
    fn test_solve_part1() {
        let input = example_parsed!();
        assert_eq!(solve_part1(&input), 21);
    }

    #[test]
    fn test_solve_part2() {
        let input = example_parsed!();
        assert_eq!(solve_part2(&input), 24);
    }
}
