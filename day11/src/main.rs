type Coordinate = (i64, i64);

#[derive(PartialEq, Debug)]
struct PuzzleInput {
    observations: Vec<Coordinate>,
}

fn parse_input(input: &str) -> PuzzleInput {
    let mut observations = vec![];
    for (y, line) in input.lines().enumerate(){
        for (x, c) in line.bytes().enumerate(){
            if b'#' == c{
                observations.push((x as i64, y as i64));
            }
            
        }
    }
    PuzzleInput { observations }
}

fn expand(observations: &Vec<Coordinate>, factor: i64) -> Vec<Coordinate>{
    let empty = |ax: &dyn Fn(&Coordinate) -> i64| -> Vec<i64>{
        let max = observations.iter().map(ax).max().unwrap();
        let mut empty = vec![];
        for c in 0..max as usize{
            let mut is_empty = true;
            for o in observations.iter(){
                if c as i64 == ax(o){
                    is_empty = false;
                    break
                }
            }
            if is_empty{
                empty.push(c as i64);
            }
        }
        empty
    };
    let empty_columns = empty(&|(x, _)|{ *x});
    let empty_rows = empty(&|(_, y)|{ *y});

    let mut expanded = observations.clone();
    for (i, o) in observations.iter().enumerate(){
        for x in empty_columns.iter(){
            if o.0 > *x{
                expanded[i].0 += factor-1;
            }else {
                break;
            }
        }
        for y in empty_rows.iter(){
            if o.1 > *y{
                expanded[i].1 += factor-1;
            }else {
                break;
            }
        }
    }
    expanded
}

fn distance(a: &Coordinate, b: &Coordinate) -> usize{
    ((a.0-b.0).abs() + (a.1-b.1).abs()) as usize
}

fn sum_expenaded_distances(input: &PuzzleInput, factor: i64) -> usize{
    let expanded: Vec<Coordinate> = expand(&input.observations, factor);
    let mut sum = 0;
    for (i, icoord) in expanded.iter().enumerate(){
        for jcoord in expanded[i+1..].iter(){
            sum += distance(icoord, jcoord);
        }
    }
    sum
}

fn solve_part1(input: &PuzzleInput) -> usize {
    sum_expenaded_distances(input, 2)
}

fn solve_part2(input: &PuzzleInput) -> usize {
    sum_expenaded_distances(input, 1000000)
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
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    macro_rules! example_parsed {
        () => {
            PuzzleInput {
                observations: vec![
                    (3, 0), (7, 1), (0, 2), (6, 4), (1, 5), (9, 6), (7, 8), (0, 9), (4, 9),
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
    fn test_distance() {
        assert_eq!(distance(&(1, 5), &(4, 9)), 7);
        assert_eq!(distance(&(4, 9), &(1, 5)), 7);
    }

    #[test]
    fn test_expansion() {
        let input = example_parsed!();
        assert_eq!(expand(&input.observations, 1), vec![
            (4, 0), (9, 1), (0, 2), (8, 5), (1, 6), (12, 7), (9, 10), (0, 11), (5, 11)
        ]);
    }

    #[test]
    fn test_solve_part1() {
        let input = example_parsed!();
        assert_eq!(solve_part1(&input), 374);
    }

    #[test]
    fn test_solve_part2() {
        let input = example_parsed!();
        assert_eq!(sum_expenaded_distances(&input, 10), 1030);
        assert_eq!(sum_expenaded_distances(&input, 100), 8410);
    }
}
