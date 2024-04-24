fn first_digit(line: &str) -> (usize, u8){
    for (i, ch) in line.bytes().enumerate(){
        if ch.is_ascii_digit(){
            return (i, ch - 48)
        }
    }
    panic!("No digits in line {}", line)
}

fn last_digit(line: &str) -> (usize, u8){
    for (i, ch) in line.bytes().enumerate().rev(){
        if ch.is_ascii_digit(){
            return (i, ch - 48)
        }
    }
    panic!("No digits in line {}", line)
}

fn solve_part1(input: &str) -> u32{
    let mut total = 0u32; 
    for line in input.lines(){
        total += (first_digit(line).1*10 + last_digit(line).1) as u32;
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
        assert_eq!(first_digit(&line), (2, 6));
    }

    #[test]
    fn test_last_digit() {
        let line = "apf1ab58c2feh";
        assert_eq!(last_digit(&line), (9, 2));
    }

    #[test]
    fn test() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(solve_part1(&input), 142);
    }
}