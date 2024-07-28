
use std::fmt;

#[derive(PartialEq, Debug, Clone, PartialOrd, Eq, Ord)]
enum Tile {
    BSM, // backward-slash-mirror
    FSM, // forward-slash-mirror
    LRS, // left right splitter
    UDS, // up down splitter
    E, // empty
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BSM => write!(f, "\\")?,
            FSM => write!(f, "/")?,
            LRS => write!(f, "-")?,
            UDS => write!(f, "|")?,
            E => write!(f, ".")?,
        }
        Ok(())
    }
}


use Tile::*;

#[derive(PartialEq, Debug, Clone)]
struct Grid {
    tiles: Box<[Tile]>,
    width: usize,
    height: usize,
}

type PuzzleInput = Grid;

impl Grid {
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

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.coord2index((x as i32, y as i32)).unwrap();
                let t = &self.tiles[index];
                write!(f, "{}", t)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse_input(input: &str) -> PuzzleInput {
    let tiles: Box<[Tile]> = input
        .as_bytes()
        .iter()
        .filter_map(|b| match b {
            b'\\' => Some(BSM),
            b'/' => Some(FSM),
            b'-' => Some(LRS),
            b'|' => Some(UDS),
            b'.' => Some(E),
            b'\n' => None,
            _ => panic!("invalid character {}", *b as char),
        })
        .collect();
    let width = input.find('\n').expect("no new line found");
    let height = tiles.len() / width;
    PuzzleInput {
        tiles,
        width,
        height,
    }
}


fn solve_part1(grid: &Grid) -> usize {
    0
}

fn solve_part2(grid: &Grid) -> usize {
    0
}

fn main() {
    let input = include_str!("../input");
    let inp = parse_input(input);

    println!("Part 1: {}", solve_part1(&inp));
    println!("Part 2: {}", solve_part2(&inp));
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE: &str = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

    macro_rules! example_parsed {
        () => {
            PuzzleInput {
                tiles: Box::new([
E,UDS,E,E,E,BSM,E,E,E,E,
UDS,E,LRS,E,BSM,E,E,E,E,E,
E,E,E,E,E,UDS,LRS,E,E,E,
E,E,E,E,E,E,E,E,UDS,E,
E,E,E,E,E,E,E,E,E,E,
E,E,E,E,E,E,E,E,E,BSM,
E,E,E,E,FSM,E,BSM,BSM,E,E,
E,LRS,E,LRS,FSM,E,E,UDS,E,E,
E,UDS,E,E,E,E,LRS,UDS,E,BSM,
E,E,FSM,FSM,E,UDS,E,E,E,E,
                ]),
                width: 10,
                height: 10,
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
        assert_eq!(solve_part1(&input), 46);
    }

    #[test]
    fn test_solve_part2() {
        let input = example_parsed!();
        assert_eq!(solve_part2(&input), 0);
    }
}
