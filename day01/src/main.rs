fn first_digit(line: &str) -> Option<(usize, u8)>{
    for (i, ch) in line.bytes().enumerate(){
        if ch.is_ascii_digit(){
            return Some((i, ch - 48))
        }
    }
    None
}

fn last_digit(line: &str) -> Option<(usize, u8)>{
    for (i, ch) in line.bytes().enumerate().rev(){
        if ch.is_ascii_digit(){
            return Some((i, ch - 48))
        }
    }
    None
}

const WORD_DIGITS: [&'static str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn first_digit_word(line: &str) -> Option<(usize, u8)>{
    let mut d = None;
    for (i, word) in WORD_DIGITS.iter().enumerate(){
        let val = i + 1;
        if let Some(index) = line.find(word){
            if d.is_none(){
                d = Some((index, val as u8));
            }else if index < d.unwrap().0{
                d = Some((index, val as u8));
            }
        }
    }
    
    return d
}

fn last_digit_word(line: &str) -> Option<(usize, u8)>{
    let find_next_word = |offset: usize| -> Option<(usize, u8)>{
        for (i, word) in WORD_DIGITS.iter().enumerate(){
            let val = i + 1;
            let res = line[offset..].find(word);
            if let Some(index) = res{
                return Some((index+offset, val as u8))
            }
        }
        return None
    };
    let mut word = None;
    let mut next_index = 0;
    while let Some((i, val)) = find_next_word(next_index){
        word = Some((i, val));
        next_index = i+1;
    }
    return word
}

fn solve_part1(input: &str) -> u32{
    let mut total = 0u32; 
    for line in input.lines(){
        total += (first_digit(line).unwrap().1*10 + last_digit(line).unwrap().1) as u32;
    }
    return total;
}

fn solve_part2(input: &str) -> u32{
    let mut total = 0u32; 
    for line in input.lines(){
        let fd = {
            let d = first_digit(line);
            let w = first_digit_word(line);
            if let Some((wi, wv)) = w{
                if let Some((di, dv)) = d{
                    if wi < di{
                        wv
                    }else{
                        dv
                    }
                }else{
                    wv
                }
            }else{
                d.unwrap().1
            }
        };
        let ld = {
            let d = last_digit(line);
            let w = last_digit_word(line);
            if let Some((wi, wv)) = w{
                if let Some((di, dv)) = d{
                    if wi > di{
                        wv
                    }else{
                        dv
                    }
                }else{
                    wv
                }
            }else{
                d.unwrap().1
            }
        };
        total += (fd*10 + ld) as u32;
    }
    return total;
}

fn main(){
    let input = include_str!("../input");
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &'static str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    const EXAMPLE_2: &'static str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    const EXAMPLE_3: &'static str = "tzgrvrkgbs7cfzf2eight76eight
";

    #[test]
    fn test_first_digit() {
        let mut lines = EXAMPLE_1.lines();
        assert_eq!(first_digit(lines.next().unwrap()), Some((0, 1)));
        assert_eq!(first_digit(lines.next().unwrap()), Some((3, 3)));
        assert_eq!(first_digit(lines.next().unwrap()), Some((1, 1)));
        assert_eq!(first_digit(lines.next().unwrap()), Some((4, 7)));

        let mut lines = EXAMPLE_2.lines();
        assert_eq!(first_digit(lines.next().unwrap()), Some((3, 1)));
        assert_eq!(first_digit(lines.next().unwrap()), None);
        assert_eq!(first_digit(lines.next().unwrap()), Some((6, 2)));
        assert_eq!(first_digit(lines.next().unwrap()), Some((6, 3)));
        assert_eq!(first_digit(lines.next().unwrap()), Some((0, 4)));
        assert_eq!(first_digit(lines.next().unwrap()), Some((8, 2)));
        assert_eq!(first_digit(lines.next().unwrap()), Some((0, 7)));
    }

    #[test]
    fn test_last_digit() {
        let mut lines = EXAMPLE_1.lines();
        assert_eq!(last_digit(lines.next().unwrap()), Some((4, 2)));
        assert_eq!(last_digit(lines.next().unwrap()), Some((7, 8)));
        assert_eq!(last_digit(lines.next().unwrap()), Some((9, 5)));
        assert_eq!(last_digit(lines.next().unwrap()), Some((4, 7)));

        let mut lines = EXAMPLE_2.lines();
        assert_eq!(last_digit(lines.next().unwrap()), Some((3, 1)));
        assert_eq!(last_digit(lines.next().unwrap()), None);
        assert_eq!(last_digit(lines.next().unwrap()), Some((6, 2)));
        assert_eq!(last_digit(lines.next().unwrap()), Some((6, 3)));
        assert_eq!(last_digit(lines.next().unwrap()), Some((15, 2)));
        assert_eq!(last_digit(lines.next().unwrap()), Some((10, 4)));
        assert_eq!(last_digit(lines.next().unwrap()), Some((0, 7)));
    }

    #[test]
    fn test_first_digit_word() {
        let mut lines = EXAMPLE_1.lines();
        assert_eq!(first_digit_word(lines.next().unwrap()), None);
        assert_eq!(first_digit_word(lines.next().unwrap()), None);
        assert_eq!(first_digit_word(lines.next().unwrap()), None);
        assert_eq!(first_digit_word(lines.next().unwrap()), None);

        let mut lines = EXAMPLE_2.lines();
        assert_eq!(first_digit_word(lines.next().unwrap()), Some((0, 2)));
        assert_eq!(first_digit_word(lines.next().unwrap()), Some((0, 8)));
        assert_eq!(first_digit_word(lines.next().unwrap()), Some((3, 1)));
        assert_eq!(first_digit_word(lines.next().unwrap()), Some((1, 2)));
        assert_eq!(first_digit_word(lines.next().unwrap()), Some((1, 9)));
        assert_eq!(first_digit_word(lines.next().unwrap()), Some((1, 1)));
        assert_eq!(first_digit_word(lines.next().unwrap()), Some((6, 6)));

        let mut lines = EXAMPLE_3.lines();
        assert_eq!(first_digit_word(lines.next().unwrap()), Some((16, 8)));
    }

    #[test]
    fn test_last_digit_word() {
        let mut lines = EXAMPLE_1.lines();
        assert_eq!(last_digit_word(lines.next().unwrap()), None);
        assert_eq!(last_digit_word(lines.next().unwrap()), None);
        assert_eq!(last_digit_word(lines.next().unwrap()), None);
        assert_eq!(last_digit_word(lines.next().unwrap()), None);

        let mut lines = EXAMPLE_2.lines();
        assert_eq!(last_digit_word(lines.next().unwrap()), Some((4, 9)));
        assert_eq!(last_digit_word(lines.next().unwrap()), Some((7, 3)));
        assert_eq!(last_digit_word(lines.next().unwrap()), Some((7, 3)));
        assert_eq!(last_digit_word(lines.next().unwrap()), Some((7, 4)));
        assert_eq!(last_digit_word(lines.next().unwrap()), Some((10, 7)));
        assert_eq!(last_digit_word(lines.next().unwrap()), Some((3, 8)));
        assert_eq!(last_digit_word(lines.next().unwrap()), Some((6, 6)));

        let mut lines = EXAMPLE_3.lines();
        assert_eq!(last_digit_word(lines.next().unwrap()), Some((23, 8)));
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(&EXAMPLE_1), 142);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(&EXAMPLE_2), 281);
    }
}