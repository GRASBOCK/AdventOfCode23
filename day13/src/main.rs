use std::str;

#[derive(PartialEq, Debug)]
struct Pattern {
    rows: Vec<u64>,
    columns: Vec<u64>,
}

#[derive(PartialEq, Debug)]
struct PuzzleInput {
    patterns: Vec<Pattern>,
}

fn parse_pattern(pattern_str: &str) -> Pattern {
    let mut rows = vec![];
    let width = pattern_str.find('\n').unwrap();
    let mut columns = vec![0; width]; 
    for (i, line) in pattern_str.lines().enumerate(){

        let mut row = 0u64;
        for (j, c) in line.bytes().rev().enumerate(){
            if c == b'#'{
                row = row | (1u64 << j);
            }
        }
        rows.push(row);
        for (j, c) in line.bytes().enumerate(){
            if c == b'#'{
                columns[j] = columns[j] | (1u64 << i);
            }
        }
    }
    Pattern { rows, columns }
}

fn parse_input(input: &str) -> PuzzleInput {
    let patterns = input.split("\n\n").map(parse_pattern).collect();
    PuzzleInput { patterns }
}

fn detect_reflection(columns: &Vec<u64>) -> Option<usize>{
    let n = columns.len();
    for i in 1..n{
        let size = std::cmp::min(i, n-i);
        let before = columns[i-size..i].iter().rev();
        let after = columns[i..i+size].iter();
        if before.eq(after){
            return Some(i)
        }
    }
    None
}

fn solve_part1(input: &PuzzleInput) -> usize {
    input.patterns.iter().map(|p|{
        if let Some(hi) = detect_reflection(&p.rows){
            hi*100
        }else if let Some(vi) = detect_reflection(&p.columns){
            vi
        }else{
            0
        }
    }).sum()
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
"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    macro_rules! example_parsed {
        () => {
            PuzzleInput {
                patterns: vec![
                    Pattern{
                        rows: vec![
                            0b101100110,
                            0b001011010,
                            0b110000001,
                            0b110000001,
                            0b001011010,
                            0b001100110,
                            0b101011010,
                        ],
                        columns: vec![
                            0b1001101,
                            0b0001100,
                            0b1110011,
                            0b0100001,
                            0b1010010,
                            0b1010010,
                            0b0100001,
                            0b1110011,
                            0b0001100,
                        ],
                    },
                    Pattern{
                        rows: vec![
                            0b100011001,
                            0b100001001,
                            0b001100111,
                            0b111110110,
                            0b111110110,
                            0b001100111,
                            0b100001001,
                        ],
                        columns: vec![
                            0b1011011,
                            0b0011000,
                            0b0111100,
                            0b0111100,
                            0b0011001,
                            0b1000011,
                            0b0111100,
                            0b0111100,
                            0b1100111,
                        ],
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
    fn test_reflection_detection() {
        let input = example_parsed!();
        // horizontal
        assert_eq!(detect_reflection(&input.patterns[0].rows), None);
        assert_eq!(detect_reflection(&input.patterns[1].rows), Some(4));
        //vertical
        assert_eq!(detect_reflection(&input.patterns[0].columns), Some(5));
        assert_eq!(detect_reflection(&input.patterns[1].columns), None);
    }

    #[test]
    fn test_solve_part1() {
        let input = example_parsed!();
        assert_eq!(solve_part1(&input), 405);
    }
}
