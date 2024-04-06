type SensorValue = u64;
type History = Vec<SensorValue>;

#[derive(PartialEq, Debug)]
struct PuzzleInput {
    histories: Vec<History>,
}

fn parse_input(input: &str) -> PuzzleInput {
    fn extract(line: &str) -> History {
        line.trim()
            .split_whitespace()
            .map(|val_str| val_str.parse::<SensorValue>().unwrap())
            .collect()
    }
    let histories = input.lines().map(extract).collect::<Vec<History>>();

    PuzzleInput { histories }
}

fn solve_part1(input: &PuzzleInput) -> usize {
    
    0
}

fn main() {
    let input = include_str!("../input");
    let input = parse_input(input);

    println!("Part 1: {}", solve_part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = 
"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    macro_rules! example_parsed {
        () => {
            PuzzleInput {
                histories: vec![
                    vec![ 0,  3,  6,  9, 12, 15],
                    vec![ 1,  3,  6, 10, 15, 21],
                    vec![10, 13, 16, 21, 30, 45],
                ]
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
        assert_eq!(solve_part1(&input), 114);
    }
}
