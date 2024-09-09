use std::fmt::{self, Display};

#[derive(PartialEq, Debug, Clone)]
struct Grid<T> {
    tiles: Box<[T]>,
    width: usize,
    height: usize,
}

impl<T: Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.coord2index((x as i32, y as i32)).unwrap();
                write!(f, "{}", self.tiles[index])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Grid<T> {
    fn coord2index(&self, p: (i32, i32)) -> Option<usize> {
        if p.0 < 0 || p.1 < 0 || p.0 as usize >= self.width || p.1 as usize >= self.height {
            None
        } else {
            Some(p.1 as usize * self.width + p.0 as usize)
        }
    }
    fn index2coord(&self, index: usize) -> Option<(i32, i32)> {
        if index >= self.width * self.height {
            None
        } else {
            Some(((index % self.width) as i32, (index / self.width) as i32))
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Color<T> {
    tiles: Box<[T]>,
    width: usize,
    height: usize,
}

#[derive(PartialEq, Debug, Clone)]
struct DigPlan {
    instructions: Vec<((i32, i32), usize, [u8; 3])>,
}

type PuzzleInput = DigPlan;

fn parse_input(input: &str) -> PuzzleInput {
    let instructions = input
        .lines()
        .map(|l| {
            let mut split = l.split_ascii_whitespace();
            let dir_char = split.next().unwrap().as_bytes()[0];
            let dir = match dir_char {
                b'U' => U,
                b'D' => D,
                b'L' => L,
                b'R' => R,
                _ => panic!("Invalid character"),
            };
            let steps = split.next().unwrap().to_string().parse::<usize>().unwrap();
            let color_hex = &split.next().unwrap()[2..8];
            let color_boxxed: Box<[u8]> = (0..color_hex.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&color_hex[i..i + 2], 16).unwrap())
                .collect();
            let mut color = [0u8; 3];
            color.clone_from_slice(&color_boxxed);
            (dir, steps, color)
        })
        .collect();
    DigPlan { instructions }
}

const U: (i32, i32) = (0, -1);
const D: (i32, i32) = (0, 1);
const L: (i32, i32) = (-1, 0);
const R: (i32, i32) = (1, 0);

fn solve_part1(input: &PuzzleInput) -> usize {
    0
}

fn solve_part2(_input: &PuzzleInput) -> usize {
    0
}

fn main() {
    // run in release mode, because otherwise recursion causes stack overflow
    let input = include_str!("../input");
    let inp = parse_input(input);

    println!("Part 1: {}", solve_part1(&inp));
    println!("Part 2: {}", solve_part2(&inp));
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

    macro_rules! example_parsed {
        () => {
            PuzzleInput {
                instructions: vec![
                    (R, 6, [0x70, 0xc7, 0x10]),
                    (D, 5, [0x0d, 0xc5, 0x71]),
                    (L, 2, [0x57, 0x13, 0xf0]),
                    (D, 2, [0xd2, 0xc0, 0x81]),
                    (R, 2, [0x59, 0xc6, 0x80]),
                    (D, 2, [0x41, 0x1b, 0x91]),
                    (L, 5, [0x8c, 0xee, 0xe2]),
                    (U, 2, [0xca, 0xa1, 0x73]),
                    (L, 1, [0x1b, 0x58, 0xa2]),
                    (U, 2, [0xca, 0xa1, 0x71]),
                    (R, 2, [0x78, 0x07, 0xd2]),
                    (U, 3, [0xa7, 0x7f, 0xa3]),
                    (L, 2, [0x01, 0x52, 0x32]),
                    (U, 2, [0x7a, 0x21, 0xe3]),
                ],
            }
        };
    }

    #[test]
    fn test_parsing() {
        let inp = example_parsed!();
        assert_eq!(parse_input(EXAMPLE), inp);
    }

    #[test]
    fn test_solve_part1() {
        let input = example_parsed!();
        assert_eq!(solve_part1(&input), 62);
    }

    #[test]
    fn test_solve_part2() {
        let input = example_parsed!();
        assert_eq!(solve_part2(&input), 51);
    }
}
