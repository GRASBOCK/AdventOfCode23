use std::{cmp::Ordering, collections::BTreeMap};

type Coord = [i32; 3];

type Dimensions = [Coord; 2];

type PuzzleInput = Box<[Dimensions]>;

fn parse_input(input: &str) -> PuzzleInput {
    input
        .lines()
        .map(|line| {
            let mut it = line.split('~');
            fn to_coord(s: &str) -> Coord {
                let mut it: std::str::Split<'_, char> = s.split(',');
                let x = it.next().unwrap().parse::<i32>().unwrap();
                let y = it.next().unwrap().parse::<i32>().unwrap();
                let z = it.next().unwrap().parse::<i32>().unwrap();
                [x, y, z]
            }
            let start = to_coord(it.next().unwrap());
            let end = to_coord(it.next().unwrap());
            [start, end]
        })
        .collect()
}

fn brick_layer_projected(dim: &Dimensions) -> Vec<[i32; 2]> {
    let x_length = (dim[1][0] - dim[0][0]).abs()+ 1;
    if x_length > 1 {
        let x_min = if dim[1][0] > dim[0][0] {
            dim[0][0]
        } else {
            dim[1][0]
        };
        return (0..x_length).map(|x| [x + x_min, dim[0][1]]).collect();
    }
    let y_length = (dim[1][1] - dim[0][1]).abs() + 1;
    if y_length > 1 {
        let y_min = if dim[1][1] > dim[0][1] {
            dim[0][1]
        } else {
            dim[1][1]
        };
        return (0..y_length).map(|y| [dim[0][0], y + y_min]).collect();
    }
    return vec![[dim[0][0], dim[0][1]]];
}

fn supporting_bricks_in_layer(layer: &Vec<([i32; 2], usize)>, coords_to_check: &Vec<[i32; 2]>) -> Vec<usize>{
    let mut indices: Vec<usize> = layer
        .iter()
        .filter_map(|(bc, brick_idx)| {
            for c in coords_to_check.iter() {
                if c == bc {
                    return Some(*brick_idx);
                }
            }
            return None;
        })
        .collect();
    // remove duplictaes
    indices.sort();
    return indices.iter().fold(Vec::<usize>::new(), |mut acc, &x|{
        if acc.iter().find(|&&a| a == x).is_none(){
            acc.push(x);
        }
        return acc;
    });
}

#[derive(Clone)]
struct SettledBrick {
    supported_by: Vec<usize>,
    supports: Vec<usize>,
}

type Tower = Vec<Vec<([i32; 2], usize)>>;
type Graph = Vec<SettledBrick>;

fn settle_bricks(input: &PuzzleInput) -> (Tower, Graph) {
    let mut sorted_bricks: Box<[(usize, Dimensions)]> =
        input.clone().iter().cloned().enumerate().collect();
    sorted_bricks.sort_by(|(_, a), (_, b)| a[0][2].cmp(&b[0][2]));
    let mut tower = Tower::new();
    let mut graph = vec![
        SettledBrick {
            supported_by: Vec::new(),
            supports: Vec::new()
        };
        input.len()
    ];
    fn settle(
        tower: &mut Tower,
        graph: &mut Graph,
        falling_brick_idx: usize,
        falling_brick_dimensions: &Dimensions,
    ) {
        let height =
            (falling_brick_dimensions[1][2] - falling_brick_dimensions[0][2]).abs() as usize + 1;
        // base coordinates of brick projected onto lowest z of brick
        let base: Vec<[i32; 2]> = brick_layer_projected(falling_brick_dimensions);
        // for each layer
        let mut resting_layer_idx = 0;
        for (layer_idx, bricks) in tower.iter().enumerate().rev() {
            let supporting_bricks: Vec<usize> = supporting_bricks_in_layer(bricks, &base);
            if !supporting_bricks.is_empty() {
                break;
            } else {
                resting_layer_idx = layer_idx + 1;
                for supporting_brick_idx in supporting_bricks {
                    graph[supporting_brick_idx].supports.push(falling_brick_idx);
                    graph[falling_brick_idx]
                        .supported_by
                        .push(supporting_brick_idx);
                }
            }
        }

        for z in 0..height {
            let layer_idx = resting_layer_idx + z;
            if layer_idx >= tower.len() {
                tower.push(Vec::new());
            }
            for b in base.iter() {
                tower[layer_idx].push((*b, falling_brick_idx));
            }
        }
    }
    for (i, b) in sorted_bricks.iter() {
        settle(&mut tower, &mut graph, *i, b);
    }
    (tower, graph)
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
                [[1, 0, 1], [1, 2, 1]], // brick index: 0
                [[0, 0, 2], [2, 0, 2]], // brick index: 1
                [[0, 2, 3], [2, 2, 3]], // brick index: 2
                [[0, 0, 4], [0, 2, 4]], // brick index: 3
                [[2, 0, 5], [2, 2, 5]], // brick index: 4
                [[0, 1, 6], [2, 1, 6]], // brick index: 5
                [[1, 1, 8], [1, 1, 9]], // brick index: 6
            ]
        };
    }

    macro_rules! example_projections {
        () => {
            [
                vec![[1, 0], [1, 1], [1, 2]], // brick index: 0
                vec![[0, 0], [1, 0], [2, 0]], // brick index: 1
                vec![[0, 2], [1, 2], [2, 2]], // brick index: 2
                vec![[0, 0], [0, 1], [0, 2]], // brick index: 3
                vec![[2, 0], [2, 1], [2, 2]], // brick index: 4
                vec![[0, 1], [1, 1], [2, 1]], // brick index: 5
                vec![[1, 1]], // brick index: 6
            ]
        };
    }

    macro_rules! example_settled {
        () => {
            vec![
                vec![([1, 0], 0), ([1, 1], 0), ([1, 2], 0)], // row 0
                vec![
                    ([0, 0], 1),
                    ([1, 0], 1),
                    ([2, 0], 1),
                    ([0, 2], 2),
                    ([1, 2], 2),
                    ([2, 2], 2),
                ], // row 1
                vec![
                    ([0, 0], 3),
                    ([0, 1], 3),
                    ([0, 2], 3),
                    ([2, 0], 4),
                    ([2, 1], 4),
                    ([2, 2], 4),
                ], // row 2
                vec![([0, 1], 5), ([1, 1], 5), ([2, 1], 5)], // row 3
                vec![([1, 1], 6)],                           // row 4
                vec![([1, 1], 6)],                           // row 5
            ]
        };
    }

    #[test]
    fn test_parsing() {
        let inp = example_parsed!();
        assert_eq!(*parse_input(EXAMPLE), inp);
    }

    #[test]
    fn test_brick_projections() {
        let input = parse_input(EXAMPLE);
        let projections = example_projections!();
        for i in 0..input.len(){
            assert_eq!(brick_layer_projected(&input[i]), projections[i]);
        }        
    }

    #[test]
    fn test_supporting_bricks_in_layer() {
        let layer = vec![([1, 0], 0), ([1, 1], 0), ([1, 2], 0)];
        assert_eq!(supporting_bricks_in_layer(&layer, &vec![[0, 0]]), vec![]);
        assert_eq!(supporting_bricks_in_layer(&layer, &vec![[1, 0]]), vec![0]); 
        assert_eq!(supporting_bricks_in_layer(&layer, &vec![[1, 1]]), vec![0]);
        let layer = vec![([0, 0], 0), ([1, 0], 0), ([2, 0], 1)];
        assert_eq!(supporting_bricks_in_layer(&layer, &vec![[0, 0]]), vec![0]);
        assert_eq!(supporting_bricks_in_layer(&layer, &vec![[1, 0]]), vec![0]); 
        assert_eq!(supporting_bricks_in_layer(&layer, &vec![[2, 0]]), vec![1]);
        assert_eq!(supporting_bricks_in_layer(&layer, &vec![[1, 0], [2, 0]]), vec![0, 1]);
        assert_eq!(supporting_bricks_in_layer(&layer, &vec![[1, 0], [2, 0], [0, 0]]), vec![0, 1]); 
    }

    #[test]
    fn test_settling() {
        let input = parse_input(EXAMPLE);
        let (tower, graph) = settle_bricks(&input);
        assert_eq!(tower, example_settled!());
    }

    #[test]
    fn test_solve_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(solve_part1(&input), 5);
    }

    #[test]
    fn test_solve_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(solve_part2(&input), 51);
    }
}
