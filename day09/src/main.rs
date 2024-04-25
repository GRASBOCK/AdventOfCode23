use itertools::Itertools; // 0.10.0

type SensorValue = i64;
type History = Vec<SensorValue>;

#[derive(PartialEq, Debug)]
struct PuzzleInput {
    histories: Vec<History>,
}

fn parse_input(input: &str) -> PuzzleInput {
    fn extract(line: &str) -> History {
        line
            .split_whitespace()
            .map(|val_str| val_str.parse::<SensorValue>().unwrap())
            .collect()
    }
    let histories = input.lines().map(extract).collect::<Vec<History>>();

    PuzzleInput { histories }
}

fn sequences(history: &[SensorValue]) -> Vec<Vec<SensorValue>> {
    fn differences(sequence: &[SensorValue]) -> Vec<SensorValue> {
        sequence
            .iter()
            .tuple_windows()
            .map(|(prev, next)| next - prev)
            .collect()
    }
    let mut sequences = vec![history.to_owned()];
    let all_zero = |seq: &Vec<SensorValue>| -> bool {
        for val in seq {
            if *val != 0 {
                return false;
            }
        }
        true
    };
    while !all_zero(&sequences[sequences.len() - 1]) {
        let last_sequence = &sequences[sequences.len() - 1];
        sequences.push(differences(last_sequence));
    }
    let last_index = sequences.len() - 1;
    sequences[last_index].push(0);
    for i in (0..sequences.len() - 1).rev() {
        let delta = sequences[i + 1].last().unwrap();
        let current_value = sequences[i].last().unwrap();
        let extrapolated = current_value + delta;
        sequences[i].push(extrapolated);
    }
    sequences
}

fn solve_part1(input: &PuzzleInput) -> SensorValue {
    input
        .histories
        .iter()
        .map(|hist| *sequences(hist)[0].last().unwrap())
        .sum()
}

fn main() {
    let input = include_str!("../input");
    let input = parse_input(input);

    println!("Part 1: {}", solve_part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    macro_rules! example_parsed {
        () => {
            PuzzleInput {
                histories: vec![
                    vec![0, 3, 6, 9, 12, 15],
                    vec![1, 3, 6, 10, 15, 21],
                    vec![10, 13, 16, 21, 30, 45],
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
    fn test_sequences() {
        let input = example_parsed!();
        assert_eq!(
            sequences(&input.histories[0]),
            vec![
                vec![0, 3, 6, 9, 12, 15, 18],
                vec![3, 3, 3, 3, 3, 3],
                vec![0, 0, 0, 0, 0],
            ]
        );
        assert_eq!(
            sequences(&input.histories[1]),
            vec![
                vec![1, 3, 6, 10, 15, 21, 28],
                vec![2, 3, 4, 5, 6, 7],
                vec![1, 1, 1, 1, 1],
                vec![0, 0, 0, 0],
            ]
        );
        assert_eq!(
            sequences(&input.histories[2]),
            vec![
                vec![10, 13, 16, 21, 30, 45, 68],
                vec![3, 3, 5, 9, 15, 23],
                vec![0, 2, 4, 6, 8],
                vec![2, 2, 2, 2],
                vec![0, 0, 0],
            ]
        );
    }

    #[test]
    fn test_solve_part1() {
        let input = example_parsed!();
        assert_eq!(solve_part1(&input), 114);
    }
}
