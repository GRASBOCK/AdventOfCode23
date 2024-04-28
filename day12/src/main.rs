#[derive(PartialEq, Debug)]
struct Row<'a>{
    springs: &'a str,
    groups: Vec<usize>, 
}

#[derive(PartialEq, Debug)]
struct PuzzleInput<'a> {
    rows: Vec<Row<'a>>,
}

fn parse_row(line: &str) -> Row{
    let mut parts = line.split_whitespace();
    let springs = parts.next().unwrap();
    let records = parts.next().unwrap();
    let groups = records.split(",").map(|s|{
        s.parse::<usize>().unwrap()
    }).collect();
    Row { springs, groups }
}

fn parse_input(input: &str) -> PuzzleInput {
    let rows = input.lines().map(parse_row).collect();
    PuzzleInput { rows }
}

fn solve_part1(input: &PuzzleInput) -> usize {
    0
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

    const EXAMPLE: &'static str = 
"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    macro_rules! example_parsed {
        () => {
            PuzzleInput {
                rows: vec![
                    Row{
                        springs: "???.###",
                        groups: vec![1,1,3]
                    },
                    Row{
                        springs: ".??..??...?##.",
                        groups: vec![1,1,3]
                    },
                    Row{
                        springs: "?#?#?#?#?#?#?#?",
                        groups: vec![1,3,1,6]
                    },
                    Row{
                        springs: "????.#...#...",
                        groups: vec![4,1,1]
                    },
                    Row{
                        springs: "????.######..#####.",
                        groups: vec![1,6,5]
                    },
                    Row{
                        springs: "?###????????",
                        groups: vec![3,2,1]
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
