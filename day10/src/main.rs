use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Tile{
    VPipe,
    HPipe,
    NEPipe,
    NWPipe,
    SWPipe,
    SEPipe,
    Ground,
    Start,
}

const WEST_CONN: [Tile; 3] = [SEPipe, NEPipe, HPipe];
const NORTH_CONN: [Tile; 3] = [SEPipe, SWPipe, VPipe];
const SOUTH_CONN: [Tile; 3] = [NEPipe, NWPipe, VPipe];
const EAST_CONN: [Tile; 3] = [SWPipe, NWPipe, HPipe];

impl Tile {
    fn next(self, from: (i64, i64)) -> Option<(i64, i64)> {
        let next_dir = |a: (i64, i64), b: (i64, i64)| -> Option<(i64, i64)>{
            if from == a{
                return Some(b)
            }else if from == b{
                return Some(a)
            }else{
                None
            }
        };
        match self {
            VPipe => next_dir((0, -1), (0, 1)),
            HPipe => next_dir((1, 0), (-1, 0)),
            NWPipe => next_dir((0, -1), (-1, 0)),
            SWPipe => next_dir((0, 1), (-1, 0)),
            NEPipe => next_dir((0, -1), (1, 0)),
            SEPipe => next_dir((0, 1), (1, 0)),
            _ => None
        }
    }
}

use Tile::*;

struct Connections{
    north: Option<[Tile; 3]>,
    south: Option<[Tile; 3]>,
    west: Option<[Tile; 3]>,
    east: Option<[Tile; 3]>,
}

#[derive(PartialEq, Debug)]
struct PuzzleInput {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

type Coordinate = (i64, i64);

impl PuzzleInput{
    fn get_by_xy<'a>(&'a self, coords: Coordinate) -> Option<&'a Tile>{
        let i = self.index(coords)?;
        Some(&self.tiles[i])
    }

    fn xy(&self, i: usize) -> Coordinate{
        return ((i % self.width) as i64, (i / self.width) as i64)
    }

    fn index(&self, (x, y): Coordinate) -> Option<usize>{
        if x < 0 || self.width as i64 - 1 < x{
            return None
        }
        if y < 0 || self.height as i64 - 1 < y{
            return None
        }
        return Some(y as usize * self.width + x as usize)
    }

    fn iter_from(&self, xy: Coordinate, from: (i64, i64)) -> PipeSegment{
        return PipeSegment{
            tilemap: &self, curr: (xy, *self.get_by_xy(xy).unwrap()), from
        }
    }
}

struct PipeSegment<'a> {
    tilemap: &'a PuzzleInput,
    curr: (Coordinate, Tile),
    from: (i64, i64),
}

impl Iterator for PipeSegment<'_> {
    // We can refer to this type using Self::Item
    type Item = (Coordinate, Tile);

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    // We use Self::Item in the return type, so we can change
    // the type without having to update the function signatures.
    fn next(&mut self) -> Option<Self::Item> {
        let (coord, tile) = self.curr;
        if tile == Tile::Start{
            return None
        }
        let dir = tile.next(self.from)?;
        let next_coord = (coord.0 + dir.0, coord.1 + dir.1);
        let next_tile = self.tilemap.get_by_xy(next_coord)?;
        self.curr = (next_coord, *next_tile);
        self.from = (-dir.0, -dir.1);
        Some(self.curr)
    }
}


fn parse_input(input: &str) -> PuzzleInput {
    let mut tiles = vec![];
    for line in input.lines(){
        for c in line.bytes(){
            let tile = match c{
                b'|' => Tile::VPipe,
                b'-' => Tile::HPipe,
                b'L' => Tile::NEPipe,
                b'J' => Tile::NWPipe,
                b'7' => Tile::SWPipe,
                b'F' => Tile::SEPipe,
                b'.' => Tile::Ground,
                b'S' => Tile::Start,
                _ => panic!("Unknown tile: {}", c as char)
            };
            tiles.push(tile);
        }
    }
    let width = input.find('\n').unwrap();
    let height = tiles.len() / width;


    PuzzleInput { width, height, tiles }
}


fn solve_part1(input: &PuzzleInput) -> usize {
    // find the start
    let mut start_index: usize = 0;
    for (i, t) in input.tiles.iter().enumerate(){
        if *t == Tile::Start{
            start_index = i;
        }
    }
    let start_xy = input.xy(start_index);
    let directions = [(1,0), (0, 1), (-1, 0), (0, -1)];
    let mut begin = ((0, 0), (0, 0));
    for dir in directions{
        let xy = (start_xy.0 + dir.0, start_xy.1 + dir.1);
        let tile = input.get_by_xy(xy);
        if let Some(tile) = tile{
            let from = (-dir.0, -dir.1); 
            if let Some(next_dir) = tile.next(from){
                begin = (xy, from);
                break;
            }
        }
    }
    assert_ne!(begin.1, (0, 0), "invalid beginning");
    let pipes = input.iter_from(begin.0, begin.1);
    (pipes.count() + 1 ) / 2
}

fn solve_part2(input: &PuzzleInput) -> u64 {
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
"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    use super::Tile::*;

    macro_rules! example_parsed {
        () => {
            PuzzleInput {
                width: 5,
                height: 5,
                tiles: vec![
                    SWPipe, HPipe, SEPipe, SWPipe, HPipe,
                    Ground, SEPipe, NWPipe, VPipe, SWPipe,
                    Start, NWPipe, NEPipe, NEPipe, SWPipe,
                    VPipe, SEPipe, HPipe, HPipe, NWPipe,
                    NEPipe, NWPipe, Ground, NEPipe, NWPipe,
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
    fn test_iteration() {
        let input = example_parsed!();

        let pipes = input.iter_from((1, 2), (-1, 0));
        
        for p in pipes{
            dbg!(p);
        }

        let pipes = input.iter_from((1, 2), (-1, 0));
        assert_eq!(pipes.count() + 1, 16)
    }


    #[test]
    fn test_solve_part1() {
        let input = example_parsed!();
        assert_eq!(solve_part1(&input), 8);
    }
}
