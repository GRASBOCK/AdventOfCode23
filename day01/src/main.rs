fn first_digit(line: &str) -> u32{
    for ch in line.chars(){
        if ch.is_ascii_digit(){
            return ch.to_digit(10).unwrap()
        }
    }
    panic!("No digits in line {}", line)
}

fn last_digit(line: &str) -> u32{
    for ch in line.chars().rev(){
        if ch.is_ascii_digit(){
            return ch.to_digit(10).unwrap()
        }
    }
    panic!("No digits in line {}", line)
}

fn solve_part1(input: &str) -> u32{
    let mut total = 0u32; 
    for line in input.lines(){
        total += first_digit(line)*10 + last_digit(line);
    }
    return total;
}

fn main(){
    let input = include_str!("../input");
    println!("Part 1: {}", solve_part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_digit() {
        let line = "uf61ab72c2efg";
        assert_eq!(first_digit(&line), 6);
    }

    #[test]
    fn test_last_digit() {
        let line = "apf1ab58c2feh";
        assert_eq!(last_digit(&line), 2);
    }

    #[test]
    fn test() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(solve_part1(&input), 142);
    }
}