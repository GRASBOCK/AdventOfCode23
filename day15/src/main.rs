fn hash(input: &str) -> usize {
    input.as_bytes()
                .iter()
                .fold(0usize, |cv, v| ((cv + *v as usize) * 17) % 256)
}

fn solve_part1(input: &str) -> usize {
    input
        .lines().next().unwrap().split(",")
        .map(|step| {
            hash(step)
        })
        .sum()
}

fn solve_part2(input: &str) -> usize {
    0
}

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("cm-"), 253);
        assert_eq!(hash("qp=3"), 97);
        assert_eq!(hash("cm=2"), 47);
        assert_eq!(hash("qp-"), 14);
        assert_eq!(hash("pc=4"), 180);
        assert_eq!(hash("ot=9"), 9);
        assert_eq!(hash("ab=5"), 197);
        assert_eq!(hash("pc-"), 48);
        assert_eq!(hash("pc=6"), 214);
        assert_eq!(hash("ot=7"), 231);
    }

    #[test]
    fn test_solve_part1() {
        let input = EXAMPLE;
        assert_eq!(solve_part1(&input), 1320);
    }

    #[test]
    fn test_solve_part2() {
        let input = EXAMPLE;
        assert_eq!(solve_part2(&input), 406);
    }
}
