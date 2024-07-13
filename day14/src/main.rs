use std::str;

#[derive(PartialEq, Debug, Clone)]
enum Tile{
    O, C, E
}

use Tile::*;

#[derive(PartialEq, Debug, Clone)]
struct PuzzleInput {
    tiles: Vec<Tile>,
    width: usize,
}

type Platform = PuzzleInput;

fn parse_input(input: &str) -> PuzzleInput {
    let tiles = input.as_bytes().iter().filter_map(|b| match b{
        b'O' => Some(O),
        b'#' => Some(C),
        b'.' => Some(E),
        b'\n' => None,
        _ => panic!("invalid character {}", *b as char)
    }).collect();
    let width = input.find("\n").expect("no new line found");
    PuzzleInput { tiles, width}
}

fn tilt_platform(platform: &Platform) -> Platform{
    platform.clone()
}

fn platform_load(platform: &Platform) -> usize{
    0
}

fn solve_part1(platform: &Platform) -> usize {
    platform_load(platform)
}

fn solve_part2(platform: &Platform) -> usize {
    10
}

fn main() {
    let input = include_str!("../input");
    let platform = parse_input(input);

    println!("Part 1: {}", solve_part1(&platform));
    println!("Part 2: {}", solve_part2(&platform));
}


#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE: &'static str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    macro_rules! example_parsed {
        () => {
            PuzzleInput {
                tiles: vec![
                O,E,E,E,E,C,E,E,E,E,
                O,E,O,O,C,E,E,E,E,C,
                E,E,E,E,E,C,C,E,E,E,
                O,O,E,C,O,E,E,E,E,O,
                E,O,E,E,E,E,E,O,C,E,
                O,E,C,E,E,O,E,C,E,C,
                E,E,O,E,E,C,O,E,E,O,
                E,E,E,E,E,E,E,O,E,E,
                C,E,E,E,E,C,C,C,E,E,
                C,O,O,E,E,C,E,E,E,E,],
                width: 10
            }
        };
    }

    #[test]
    fn test_parsing() {
        let platform = example_parsed!();
        assert_eq!(parse_input(&EXAMPLE), platform);
    }

    #[test]
    fn test_tilting() {
        let platform = example_parsed!();
        let platform_tilted = Platform {
            tiles: vec![
                O,O,O,O,E,C,E,O,E,E,
                O,O,E,E,C,E,E,E,E,C,
                O,O,E,E,O,C,C,E,E,O,
                O,E,E,C,E,O,O,E,E,E,
                E,E,E,E,E,E,E,E,C,E,
                E,E,C,E,E,E,E,C,E,C,
                E,E,O,E,E,C,E,O,E,O,
                E,E,O,E,E,E,E,E,E,E,
                C,E,E,E,E,C,C,C,E,E,
                C,E,E,E,E,C,E,E,E,E,],
            width: 10
        };
        assert_eq!(tilt_platform(&platform), platform_tilted);
    }

    #[test]
    fn test_solve_part1() {
        let input = example_parsed!();
        assert_eq!(solve_part1(&input), 136);
    }


    #[test]
    fn test_solve_part2() {
        let input = example_parsed!();
        assert_eq!(solve_part2(&input), 0);
    }
}
