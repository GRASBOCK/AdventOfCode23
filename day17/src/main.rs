
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

#[derive(Clone, Copy)]
struct Path{
    heatloss_since_origin: usize,
    from_dir: (i32, i32)
}

const UP: (i32, i32) = (0, -1);
const DOWN: (i32, i32) = (0, 1);
const LEFT: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (1, 0);

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.from_dir == UP{
            write!(f, "v")?;
        }else if self.from_dir == DOWN{
            write!(f, "^")?;
        }else if self.from_dir == LEFT{
            write!(f, ">")?;
        }else if self.from_dir == RIGHT{
            write!(f, "<")?;
        }else{
            write!(f, ".")?;
        }
        Ok(())
    }
}

fn determine_distance(xy: &(i32, i32), consecutive: u8, blocks: &Grid<u8>, paths: &mut Grid<Path>){
    let directions = [UP, DOWN, LEFT, RIGHT];
    let index = blocks.coord2index(*xy).unwrap();
    let p = paths.tiles[index];
    for dir in directions{
        if p.from_dir == dir{
            // don't check origin
            continue;
        }
        let mut consecutive = consecutive;
        if (-p.from_dir.0, -p.from_dir.1) != dir{
            // not same direction
            consecutive = 0;
        }else if consecutive >= 3{
            // don't allow more than 3 in the same direction
            continue;
        }
        let xy_next = (xy.0 + dir.0, xy.1 + dir.1);
        let next_i = blocks.coord2index(xy_next);
        if let Some(next_i) = next_i{
            let b_next = blocks.tiles[next_i];
            let p_next = &mut paths.tiles[next_i];
            let go_to_next_heatloss = p.heatloss_since_origin + b_next as usize;
            if p_next.heatloss_since_origin > go_to_next_heatloss{
                p_next.heatloss_since_origin = go_to_next_heatloss;
                p_next.from_dir = (-dir.0, -dir.1);
                determine_distance(&xy_next, consecutive+1, blocks, paths);
            }
        }
    }
}

fn total_heatloss_from_to(origin: &(i32, i32), target: &(i32, i32), blocks: &Grid<u8>) -> usize{
    let path_tiles: Box<[Path]> = vec![Path{heatloss_since_origin: usize::MAX, from_dir: (0, 0)}; blocks.width*blocks.height].iter().copied().collect();
    let mut paths = Grid::<Path>{tiles: path_tiles, width:blocks.width, height: blocks.height};
    
    let origin_i = blocks.coord2index(*origin).unwrap();
    paths.tiles[origin_i].heatloss_since_origin = 0;
    determine_distance(origin, 0, blocks, &mut paths);
    
    println!("{}", paths);

    let target_i = blocks.coord2index(*target).unwrap();    
    paths.tiles[target_i].heatloss_since_origin
}

fn solve_part1(input: &PuzzleInput) -> usize {
    total_heatloss_from_to(&(0, 0), &(input.width as i32-1, input.height as i32-1), input)
}

fn solve_part2(input: &PuzzleInput) -> usize {
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
        assert_eq!(solve_part1(&input), 102);
    }

    #[test]
    fn test_solve_part2() {
        let input = example_parsed!();
        assert_eq!(solve_part2(&input), 51);
    }
}
