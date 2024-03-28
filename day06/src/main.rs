fn parse_input(input: &str) -> Vec<(u64, u64)> {
    let mut lines = input.lines();
    let extract_numbers = |line: &str| -> Vec<u64>{
        line.split(":").skip(1).next().unwrap().split(" ").filter(|s| s.len() > 0).map(|x| x.parse::<u64>().unwrap()).collect()
    };
    let durations: Vec<u64> = extract_numbers(lines.next().unwrap());
    let distances: Vec<u64> = extract_numbers(lines.next().unwrap());
    assert_eq!(durations.len(), distances.len());
    (0..durations.len()).map(|i| (durations[i], distances[i])).collect()
}


fn solve_part1(races: &Vec<(u64, u64)>) -> u64{
    0
}

fn main() {
    let input = include_str!("../input");
    let races = parse_input(input);

    println!("Part 1: {}", solve_part1(&races));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = 
"Time:      7  15   30
Distance:  9  40  200";

    macro_rules! example_races {
        () => {
            vec![(7, 9), (15, 40), (30, 200)]
        };
    }

    #[test]
    fn test_parsing() {
        let races = example_races!();
        assert_eq!(parse_input(&EXAMPLE), races);
    }

    #[test]
    fn test_solve_part1() {
        let races = example_races!();
        assert_eq!(solve_part1(&races), 288);
    }
}
