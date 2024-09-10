use std::{
    collections::BTreeMap,
    fmt::{self, Display},
};

#[derive(PartialEq, Debug, Clone)]
struct Workflow<'a> {
    conditions: Vec<Condition<'a>>,
    next: &'a str,
}

#[derive(PartialEq, Debug, Clone)]
struct Condition<'a> {
    idx: usize,
    less: bool,
    number: usize,
    next: &'a str,
}

#[derive(PartialEq, Debug, Clone)]
struct PuzzleInput<'a> {
    workflows: BTreeMap<&'a str, Workflow<'a>>,
    parts: Vec<[usize; 4]>,
}

fn parse_condition<'a>(text: &'a str) -> Condition<'a> {
    let mut it = text.split(&['<', '>', ':']);
    let idx = match it.next().unwrap() {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!("unknown category"),
    };
    let operator_index = text.find(['<', '>']).unwrap();
    let less = match &text[operator_index..operator_index + 1] {
        "<" => true,
        ">" => false,
        _ => panic!("unknown operator"),
    };
    let number = it.next().unwrap().to_string().parse::<usize>().unwrap();
    let next = it.next().unwrap();
    Condition {
        idx,
        less,
        number,
        next,
    }
}

fn parse_part(line: &str) -> [usize; 4] {
    let trimmed: String = line.replace(['x', 'a', 'm', 's', '{', '}', '=', '\n', ','], " ");
    let numbers_vec: Vec<usize> = trimmed
        .split_whitespace()
        .map(|n| {
            n.to_string()
                .parse::<usize>()
                .expect(format!("failed to parse '{n}'").as_str())
        })
        .collect();
    let mut numbers = [0usize; 4];
    numbers.copy_from_slice(&numbers_vec[0..4]);
    numbers
}

fn parse_input(input: &str) -> PuzzleInput {
    let mut sections = input.split("\n\n");
    let workflows = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut sections = line.split("{");
            let name = sections.next().unwrap();
            let rules_string = sections.next().unwrap();
            let rules_string = &rules_string[0..rules_string.len() - 1]; // without last character
            let rules: Vec<&str> = rules_string.split(",").collect();
            let next = rules[rules.len() - 1];
            let conditions = rules[0..rules.len() - 1]
                .iter()
                .map(|s| parse_condition(*s))
                .collect();
            (name, Workflow { conditions, next })
        })
        .collect();
    let parts = sections.next().unwrap().lines().map(parse_part).collect();
    PuzzleInput { workflows, parts }
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

    const EXAMPLE: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_parsing_part() {
        let part_text = "{x=787,m=2655,a=1222,s=2876}";
        assert_eq!(parse_part(part_text), [787, 2655, 1222, 2876]);
    }

    #[test]
    fn test_parsing_condition() {
        assert_eq!(parse_condition("a<2006:qkq"), Condition{idx: 2, less: true, number: 2006, next: "qkq"});
        assert_eq!(parse_condition("s<537:gd"), Condition{idx: 3, less: true, number: 537, next: "gd"});
        assert_eq!(parse_condition("m>1548:A"), Condition{idx: 1, less: false, number: 1548, next: "A"});
    }

    #[test]
    fn test_parsing() {
        let input = parse_input(EXAMPLE);
        dbg!(input);
    }

    #[test]
    fn test_solve_part1() {
        let input = parse_input(EXAMPLE);
        assert_eq!(solve_part1(&input), 62);
    }

    #[test]
    fn test_solve_part2() {
        let input = parse_input(EXAMPLE);
        assert_eq!(solve_part2(&input), 51);
    }
}
