use std::{
    collections::{BTreeMap},
    fmt, str,
};

#[derive(PartialEq, Debug, Clone, PartialOrd, Eq, Ord)]
enum Tile {
    O,
    C,
    E,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            O => write!(f, "O")?,
            C => write!(f, "#")?,
            E => write!(f, ".")?,
        }
        Ok(())
    }
}

use Tile::*;

#[derive(PartialEq, Debug, Clone)]
struct PuzzleInput {
    tiles: Box<[Tile]>,
    width: usize,
    height: usize,
}

type Platform = PuzzleInput;

impl Platform {
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

impl fmt::Display for Platform {
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
    North,
    South,
    West,
    East,
}

fn parse_input(input: &str) -> PuzzleInput {
    let tiles: Box<[Tile]> = input
        .as_bytes()
        .iter()
        .filter_map(|b| match b {
            b'O' => Some(O),
            b'#' => Some(C),
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

fn tilt_platform(platform: &Platform, direction: Direction) -> Platform {
    let mut platform = platform.clone();
    let (origins, offset) = match direction {
        Direction::North => {
            let origins: Vec<(i32, i32)> = (0..platform.width).map(|x| (x as i32, 0)).collect();
            (origins, (0, 1))
        }
        Direction::South => {
            let origins: Vec<(i32, i32)> = (0..platform.width)
                .map(|x| (x as i32, platform.height as i32 - 1))
                .collect();
            (origins, (0, -1))
        }
        Direction::West => {
            let origins: Vec<(i32, i32)> = (0..platform.height).map(|y| (0, y as i32)).collect();
            (origins, (1, 0))
        }
        Direction::East => {
            let origins: Vec<(i32, i32)> = (0..platform.height)
                .map(|y| (platform.width as i32 - 1, y as i32))
                .collect();
            (origins, (-1, 0))
        }
    };
    let next = |p: (i32, i32)| (p.0 + offset.0, p.1 + offset.1);
    let swap = |tiles: &mut Box<[Tile]>, i1: usize, i2: usize| {
        let t1 = tiles[i1].clone();
        tiles[i1] = tiles[i2].clone();
        tiles[i2] = t1.clone();
    };
    let mut tiles = platform.tiles.clone();
    for origin in origins {
        let mut cursor = origin;
        let mut empty_space = None;
        while let Some(i) = platform.coord2index(cursor) {
            //dbg!(cursor, i);
            match tiles[i] {
                O => {
                    if let Some(ei) = empty_space {
                        swap(&mut tiles, i, ei);
                        let next_empty_index = platform
                            .coord2index(next(platform.index2coord(ei).unwrap()))
                            .unwrap();
                        empty_space = Some(next_empty_index);
                    }
                }
                C => {
                    empty_space = None;
                }
                E => {
                    if empty_space.is_none() {
                        empty_space = Some(i)
                    }
                }
            }
            cursor = next(cursor);
        }
    }
    platform.tiles = tiles;
    platform
}

fn platform_load(platform: &Platform, direction: Direction) -> usize {
    let (origins, offset) = match direction {
        Direction::North => {
            let origins: Vec<(i32, i32)> = (0..platform.width)
                .map(|x| (x as i32, platform.height as i32 - 1))
                .collect();
            (origins, (0, -1))
        }
        Direction::South => {
            let origins: Vec<(i32, i32)> = (0..platform.width).map(|x| (x as i32, 0)).collect();
            (origins, (0, 1))
        }
        Direction::West => {
            let origins: Vec<(i32, i32)> = (0..platform.height)
                .map(|y| (platform.width as i32 - 1, y as i32))
                .collect();
            (origins, (-1, 0))
        }
        Direction::East => {
            let origins: Vec<(i32, i32)> = (0..platform.height).map(|y| (0, y as i32)).collect();
            (origins, (1, 0))
        }
    };
    let next = |p: (i32, i32)| (p.0 + offset.0, p.1 + offset.1);
    let mut total_load = 0;
    for origin in origins {
        let mut cursor = origin;
        let mut load = 1;
        while let Some(i) = platform.coord2index(cursor) {
            if platform.tiles[i] == O {
                total_load += load;
            }
            cursor = next(cursor);
            load += 1;
        }
    }
    total_load
}

fn solve_part1(platform: &Platform) -> usize {
    let tilted = tilt_platform(platform, Direction::North);
    platform_load(&tilted, Direction::North)
}

fn cycle(platform: &Platform) -> Platform {
    let platform = tilt_platform(platform, Direction::North);
    let platform = tilt_platform(&platform, Direction::West);
    let platform = tilt_platform(&platform, Direction::South);
    
    tilt_platform(&platform, Direction::East)
}

fn solve_part2(platform: &Platform) -> usize {
    let mut last_cycle_platform = platform.clone();
    let mut store: BTreeMap<Box<[Tile]>, Box<[Tile]>> = BTreeMap::new();
    let n = 1000000000;
    let mut i = 0;
    while i < n {
        let stored_platform = store.get(&last_cycle_platform.tiles);
        let platform = if let Some(previous) = stored_platform {
            // it repeats!
            let mut j = 1;
            let mut tiles = previous;
            while tiles != &last_cycle_platform.tiles {
                tiles = store.get(tiles).unwrap();
                j += 1;
            }
            println!("Repeates after {j} cycles");
            let iterations_to_skip = (n - i) / j * j;
            i += iterations_to_skip;
            Platform {
                tiles: previous.clone(),
                width: last_cycle_platform.width,
                height: last_cycle_platform.height,
            }
        } else {
            let p = cycle(&last_cycle_platform);
            store.insert(last_cycle_platform.tiles.clone(), p.tiles.clone());
            p
        };

        if last_cycle_platform == platform {
            last_cycle_platform = platform;
            break;
        } else {
            last_cycle_platform = platform;
        }
        i += 1;
    }
    platform_load(&last_cycle_platform, Direction::North)
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

    const EXAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    const EXAMPLE_1_CYCLE: &str = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....";

    const EXAMPLE_2_CYCLE: &str = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O";

    const EXAMPLE_3_CYCLE: &str = ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";

    macro_rules! example_parsed {
        () => {
            PuzzleInput {
                tiles: Box::new([
                    O, E, E, E, E, C, E, E, E, E, O, E, O, O, C, E, E, E, E, C, E, E, E, E, E, C,
                    C, E, E, E, O, O, E, C, O, E, E, E, E, O, E, O, E, E, E, E, E, O, C, E, O, E,
                    C, E, E, O, E, C, E, C, E, E, O, E, E, C, O, E, E, O, E, E, E, E, E, E, E, O,
                    E, E, C, E, E, E, E, C, C, C, E, E, C, O, O, E, E, C, E, E, E, E,
                ]),
                width: 10,
                height: 10,
            }
        };
    }

    #[test]
    fn test_parsing() {
        let platform = example_parsed!();
        assert_eq!(parse_input(EXAMPLE), platform);
    }

    #[test]
    fn test_tilting() {
        let platform = example_parsed!();
        let platform_tilted = Platform {
            tiles: Box::new([
                O, O, O, O, E, C, E, O, E, E, O, O, E, E, C, E, E, E, E, C, O, O, E, E, O, C, C, E,
                E, O, O, E, E, C, E, O, O, E, E, E, E, E, E, E, E, E, E, E, C, E, E, E, C, E, E, E,
                E, C, E, C, E, E, O, E, E, C, E, O, E, O, E, E, O, E, E, E, E, E, E, E, C, E, E, E,
                E, C, C, C, E, E, C, E, E, E, E, C, E, E, E, E,
            ]),
            width: 10,
            height: 10,
        };
        assert_eq!(tilt_platform(&platform, Direction::North), platform_tilted);
    }

    #[test]
    fn test_cycles() {
        let platform = example_parsed!();
        let platform = cycle(&platform);
        assert_eq!(
            parse_input(EXAMPLE_1_CYCLE),
            platform,
            "cycle 1 doesnt match"
        );
        let platform = cycle(&platform);
        assert_eq!(
            parse_input(EXAMPLE_2_CYCLE),
            platform,
            "cycle 2 doesnt match"
        );
        let platform = cycle(&platform);
        assert_eq!(
            parse_input(EXAMPLE_3_CYCLE),
            platform,
            "cycle 3 doesnt match"
        );
    }

    #[test]
    fn test_solve_part1() {
        let input = example_parsed!();
        assert_eq!(solve_part1(&input), 136);
    }

    #[test]
    fn test_solve_part2() {
        let input = example_parsed!();
        assert_eq!(solve_part2(&input), 64);
    }
}
