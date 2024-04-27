use std::collections::HashMap;

type Coordinate = (i64, i64);

#[derive(PartialEq, Debug)]
struct PuzzleInput {
    observations: Vec<Coordinate>,
}

fn parse_input(input: &str) -> PuzzleInput {
    let mut observations = vec![];
    for (y, line) in input.lines().enumerate(){
        for (x, c) in line.bytes().enumerate(){
            if b'#' == c{
                observations.push((x as i64, y as i64));
            }
            
        }
    }
    PuzzleInput { observations }
}


fn solve_part1(input: &PuzzleInput) -> usize {
    10
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
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    macro_rules! example_parsed {
        () => {
            PuzzleInput {
                observations: vec![
                    (3, 0), (7, 1), (0, 2), (6, 4), (1, 5), (9, 6), (7, 8), (0, 9), (4, 9),
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
        assert_eq!(solve_part1(&input), 374);
    }
}
