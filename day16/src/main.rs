
use std::fmt::{self, Display};

#[derive(PartialEq, Debug, Clone)]
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

extern crate bitflags;


bitflags::bitflags! {
    #[derive(PartialEq, Debug, Clone, Copy)]
    struct Beam: u8 {
        const None = 0b00000000;
        const FromUp = 0b00000001;
        const FromDown = 0b00000010;
        const FromLeft = 0b00000100;
        const FromRight = 0b00001000;
    }
}

impl fmt::Display for Beam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Beam::FromDown || *self == Beam::FromUp{
            write!(f, "|")?;
        }else if *self == Beam::FromLeft || *self == Beam::FromRight{
            write!(f, "-")?;
        }else{
            write!(f, ".")?;
        }
        Ok(())
    }
}

impl From<&(i32, i32)> for Beam {
    fn from(v: &(i32, i32)) -> Beam {
        if v.0 > 0{
            Beam::FromLeft
        }else if v.0 < 0{
            Beam::FromRight
        }else if v.1 > 0{
            Beam::FromUp
        }else if v.1 < 0{
            Beam::FromDown
        }else{
            Beam::None
        }
    }
}

#[derive(Debug, Clone)]
struct LightParticle {
    pos: (i32, i32),
    velocity: (i32, i32),
}

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

type PuzzleInput = Grid<Tile>;

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

fn light_particle_simulation(p: LightParticle, mirrors: &Grid<Tile>, beams: &mut Grid<Beam>){
    // check what happens
    if let Some(index) = mirrors.coord2index(p.pos){
        let t = &mirrors.tiles[index];
        let b = &beams.tiles[index];
        let vb = Beam::from(&p.velocity);
        if *b == vb{
            // already traversed
            return;
        }else{
            beams.tiles[index] = *b | vb;
            let paricle_velocities = match t{
                BSM => if p.velocity == (1, 0){
                    // from the left
                    vec![(0, 1)]
                }else if p.velocity == (-1, 0){
                    // from the right
                    vec![(0, -1)]
                }else if p.velocity == (0, 1){
                    // from the top
                    vec![(1, 0)]
                }else{
                    // from the bottom
                    vec![(-1, 0)]
                },
                FSM => if p.velocity == (1, 0){
                    // from the left
                    vec![(0, -1)]
                }else if p.velocity == (-1, 0){
                    // from the right
                    vec![(0, 1)]
                }else if p.velocity == (0, 1){
                    // from the top
                    vec![(-1, 0)]
                }else{
                    // from the bottom
                    vec![(1, 0)]
                },
                LRS => if vb == Beam::FromDown || vb == Beam::FromUp{
                    vec![(1, 0), (-1, 0)]
                }else{
                    vec![p.velocity]
                },
                UDS => if vb == Beam::FromRight || vb == Beam::FromLeft{
                    vec![(0, 1), (0, -1)]
                }else{
                    vec![p.velocity]
                },
                E => vec![p.velocity],
            };
            for v in paricle_velocities{
                let p = LightParticle{
                    pos: (p.pos.0 + v.0, p.pos.1 + v.1), 
                    velocity: v
                };
                light_particle_simulation(p, mirrors, beams)
            }
        }
    }else{
        // particle outside of grid
        return;
    }  
}

fn number_energized_tiles(p: LightParticle, mirrors: &Grid<Tile>) -> usize {
    let beam_tiles: Box<[Beam]> = vec![Beam::empty(); mirrors.width*mirrors.height].iter().copied().collect();
    let mut beams = Grid::<Beam>{tiles: beam_tiles, width:mirrors.width, height: mirrors.height};
    light_particle_simulation(p, mirrors, &mut beams);
    beams.tiles.iter().fold(0, |acc, b| if *b != Beam::None { acc + 1} else{ acc })
}

fn solve_part1(mirrors: &Grid<Tile>) -> usize {
    let p = LightParticle{
        pos: (0,0), 
        velocity: (1, 0)
    };
    number_energized_tiles(p, mirrors)
}

fn solve_part2(mirrors: &Grid<Tile>) -> usize {
    let mut starting_particles = vec![];
    for x in 0..mirrors.width{
        starting_particles.push(LightParticle{pos: (x.try_into().unwrap(), 0), velocity: (0, 1)});
        starting_particles.push(LightParticle{pos: (x.try_into().unwrap(), (mirrors.height-1).try_into().unwrap()), velocity: (0, -1)});
    }
    for y in 0..mirrors.height{
        starting_particles.push(LightParticle{pos: (0, y.try_into().unwrap()), velocity: (1, 0)});
        starting_particles.push(LightParticle{pos: ((mirrors.width-1).try_into().unwrap(), y.try_into().unwrap()), velocity: (-1, 0)});
    }
    starting_particles.iter().map(|p| number_energized_tiles(p.clone(), mirrors)).max().unwrap()
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
        assert_eq!(solve_part2(&input), 51);
    }
}
