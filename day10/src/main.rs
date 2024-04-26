#[derive(PartialEq, Debug, Clone)]
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

#[derive(PartialEq, Debug)]
struct PuzzleInput {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl PuzzleInput{
    fn get<'a>(&'a self, coords: (i64, i64)) -> Option<&'a Tile>{
        let i = self.index(coords)?;
        Some(&self.tiles[i])
    }

    fn xy(&self, i: usize) -> (i64, i64){
        return ((i % self.width) as i64, (i / self.width) as i64)
    }

    fn index(&self, (x, y): (i64, i64)) -> Option<usize>{
        if x < 0 || self.width as i64 - 1 < x{
            return None
        }
        if y < 0 || self.height as i64 - 1 < y{
            return None
        }
        return Some(y as usize * self.width + x as usize)
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


fn solve_part1(input: &PuzzleInput) -> u64 {
    0
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
    fn test_solve_part1() {
        let input = example_parsed!();
        assert_eq!(solve_part1(&input), 114);
    }
}
