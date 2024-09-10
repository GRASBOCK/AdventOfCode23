type Coord = [i32; 3];

#[derive(PartialEq, Debug, Clone)]
struct Brick {
    start: Coord,
    end: Coord
}

type PuzzleInput = Box<[Brick]>;

fn parse_input(input: &str) -> PuzzleInput {
    input.lines().map(|line|{
        let mut it = line.split('~');
        fn to_coord(s: &str) -> Coord{
            let mut it = s.split(',');
            let x = it.next().unwrap().parse::<i32>().unwrap();
            let y = it.next().unwrap().parse::<i32>().unwrap();
            let z = it.next().unwrap().parse::<i32>().unwrap();
            [x, y, z]
        }
        let start = to_coord(it.next().unwrap());
        let end = to_coord(it.next().unwrap());
        Brick { start, end }
    }).collect()
}

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

    const EXAMPLE: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    macro_rules! example_parsed {
        () => {
            [
                Brick{start: [1,0,1], end: [1,2,1]},
                Brick{start: [0,0,2], end: [2,0,2]},
                Brick{start: [0,2,3], end: [2,2,3]},
                Brick{start: [0,0,4], end: [0,2,4]},
                Brick{start: [2,0,5], end: [2,2,5]},
                Brick{start: [0,1,6], end: [2,1,6]},
                Brick{start: [1,1,8], end: [1,1,9]},
            ]
        };
    }

    #[test]
    fn test_parsing() {
        let inp = example_parsed!();
        assert_eq!(*parse_input(EXAMPLE), inp);
    }

    #[test]
    fn test_solve_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(solve_part1(&input), 19114);
    }

    #[test]
    fn test_solve_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(solve_part2(&input), 51);
    }
}
