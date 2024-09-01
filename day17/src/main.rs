
use std::fmt::{self, Display};

#[derive(PartialEq, Debug, Clone)]
struct Grid<T> {
    tiles: Box<[T]>,
    width: usize,
    height: usize,
}

impl<T: Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height{
            for x in 0..self.width{
                let index = self.coord2index((x as i32, y as i32)).unwrap();
                write!(f, "{}", self.tiles[index])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

type PuzzleInput = Grid<u8>;

impl <T> Grid<T> {
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

fn parse_input(input: &str) -> PuzzleInput {
    let tiles: Box<[u8]> = input
        .as_bytes()
        .iter()
        .filter_map(|b| match b {
            b'\n' => None,
            _ => Some(*b-48),
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


fn solve_part1(input: &PuzzleInput) -> usize {
    0
}

fn solve_part2(input: &PuzzleInput) -> usize {
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

    const EXAMPLE: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

    macro_rules! example_parsed {
        () => {
            PuzzleInput {
                tiles: Box::new([
                    2,4,1,3,4,3,2,3,1,1,3,2,3,
                    3,2,1,5,4,5,3,5,3,5,6,2,3,
                    3,2,5,5,2,4,5,6,5,4,2,5,4,
                    3,4,4,6,5,8,5,8,4,5,4,5,2,
                    4,5,4,6,6,5,7,8,6,7,5,3,6,
                    1,4,3,8,5,9,8,7,9,8,4,5,4,
                    4,4,5,7,8,7,6,9,8,7,7,6,6,
                    3,6,3,7,8,7,7,9,7,9,6,5,3,
                    4,6,5,4,9,6,7,9,8,6,8,8,7,
                    4,5,6,4,6,7,9,9,8,6,4,5,3,
                    1,2,2,4,6,8,6,8,6,5,5,6,3,
                    2,5,4,6,5,4,8,8,8,7,7,3,5,
                    4,3,2,2,6,7,4,6,5,5,5,3,3,
                ]),
                width: 13,
                height: 13,
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
        assert_eq!(solve_part2(&input), 51);
    }
}
