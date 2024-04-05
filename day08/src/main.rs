use std::collections::BTreeMap;

#[derive(PartialEq, Debug)]
enum Instruction {
    Left,
    Right,
}
type Node<'a> = &'a str;

#[derive(PartialEq, Debug)]
struct Map<'a> {
    instructions: Vec<Instruction>,
    nodes: BTreeMap<Node<'a>, (Node<'a>, Node<'a>)>,
}

fn parse_input(input: &str) -> Map {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let instructions = first_line
        .chars()
        .map(|c| {
            if c == 'L' {
                Instruction::Left
            } else {
                Instruction::Right
            }
        })
        .collect::<Vec<Instruction>>();
    lines.next(); // empty line
    fn extract(line: &str) -> (&str, (&str, &str)) {
        let node = &line[0..3];
        let left_node = &line[7..10];
        let right_node = &line[12..15];
        (node, (left_node, right_node))
    }
    let nodes = lines
        .map(extract)
        .collect::<BTreeMap<Node, (Node, Node)>>();

    Map {
        instructions,
        nodes,
    }
}

fn solve_part1(map: &Map) -> u64 {
    todo!()
}

fn main() {
    let input = include_str!("../input");
    let map = parse_input(input);

    println!("Part 1: {}", solve_part1(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &'static str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    const EXAMPLE2: &'static str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    macro_rules! example_1_map {
        () => {
            Map {
                instructions: vec![Instruction::Right, Instruction::Left],
                nodes: BTreeMap::from([
                    ("AAA", ("BBB", "CCC")),
                    ("BBB", ("DDD", "EEE")),
                    ("CCC", ("ZZZ", "GGG")),
                    ("DDD", ("DDD", "DDD")),
                    ("EEE", ("EEE", "EEE")),
                    ("GGG", ("GGG", "GGG")),
                    ("ZZZ", ("ZZZ", "ZZZ")),
                ]),
            }
        };
    }

    macro_rules! example_2_map {
        () => {
            Map {
                instructions: vec![Instruction::Left, Instruction::Left, Instruction::Right],
                nodes: BTreeMap::from([
                    ("AAA", ("BBB", "BBB")),
                    ("BBB", ("AAA", "ZZZ")),
                    ("ZZZ", ("ZZZ", "ZZZ")),
                ]),
            }
        };
    }

    #[test]
    fn test_parsing1() {
        let map = example_1_map!();
        assert_eq!(parse_input(&EXAMPLE1), map);
    }

    #[test]
    fn test_parsing2() {
        let map = example_2_map!();
        assert_eq!(parse_input(&EXAMPLE2), map);
    }

    #[test]
    fn test_solve_part1() {
        let steps = example_1_map!();
        assert_eq!(solve_part1(&steps), 2);

        let steps = example_2_map!();
        assert_eq!(solve_part1(&steps), 6);
    }
}
