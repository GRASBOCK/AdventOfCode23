use std::str;

#[derive(PartialEq, Debug)]
struct Pattern {
    // assumes, that rows and columns are all smaller than 64 characters long
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

fn pattern_number(p: &Pattern) -> usize{
    if let Some(hi) = detect_reflection(&p.rows){
        hi*100
    }else if let Some(vi) = detect_reflection(&p.columns){
        vi
    }else{
        0
    }
}

fn solve_part1(input: &PuzzleInput) -> usize {
    input.patterns.iter().map(pattern_number).sum()
}

fn solve_part2(input: &PuzzleInput) -> usize {
    input.patterns.iter().enumerate().map(|(i, p)|{
        println!("p: {}", i);
        let original = pattern_number(p);
        // crazy inefficient solution
        // try out every possibility
        for ri in 0..p.rows.len(){
            for ci in 0..p.columns.len(){
                let mut rows = p.rows.clone();
                let mut columns = p.columns.clone();
                rows[ri] = rows[ri] ^ (1u64 << (p.columns.len()-1 - ci));
                columns[ci] = columns[ci] ^ (1u64 << ri);
                println!("p row: {:#020b}", p.rows[ri]);
                println!("n row: {:#020b}\n", rows[ri]);
                let num = pattern_number(&Pattern { rows, columns });
                if num != 0 && num != original{
                    println!("Found");
                    return num
                }
            }
        }
        panic!("No reflections after removing smudges");
    }).sum()
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
#....#..#

#.##....##.#.
#.##....##.#.
.#.#....#.#.#
.###....###..
#.#.####.#.#.
.####...###.#
#..######..#.
..#..##..#..#
.#........#.#";

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
                    Pattern{
                        rows: vec![
                            0b1011000011010,
                            0b1011000011010,
                            0b0101000010101,
                            0b0111000011100,
                            0b1010111101010,
                            0b0111100011101,
                            0b1001111110010,
                            0b0010011001001,
                            0b0100000000101,
                        ],
                        columns: vec![
                            0b001010011,
                            0b100101100,
                            0b010111011,
                            0b001101111,
                            0b001110000,
                            0b011010000,
                            0b011010000,
                            0b001010000,
                            0b001101111,
                            0b010111011,
                            0b100101100,
                            0b001010011,
                            0b110100100,
                        ]
                    }
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
        assert_eq!(detect_reflection(&input.patterns[2].rows), Some(1));
        //vertical
        assert_eq!(detect_reflection(&input.patterns[0].columns), Some(5));
        assert_eq!(detect_reflection(&input.patterns[1].columns), None);
        assert_eq!(detect_reflection(&input.patterns[2].columns), None);
    }

    #[test]
    fn test_solve_part1() {
        let input = example_parsed!();
        assert_eq!(solve_part1(&input), 505);
    }

    #[test]
    fn test_solve_part2() {
        let input = example_parsed!();
        assert_eq!(solve_part2(&input), 406);
    }
}
