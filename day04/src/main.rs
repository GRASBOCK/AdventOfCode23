use regex::Regex;

type Scratchcard = (usize, Vec<u32>, Vec<u32>);

fn parse_input(input: &str) -> Vec<Scratchcard> {
    fn parse_line(line: &str) -> Scratchcard {
        let re = Regex::new(r"Card\s*(?<id>\d*):").unwrap();
        let caps = re.captures(line).unwrap();
        let id = caps["id"].parse::<usize>().unwrap();

        let after_colon = line.split(":").skip(1).next().expect("Nothing after \":\"");
        let mut iterator = after_colon.split("|");
        let extract_numbers = |numbers_str: &str| -> Vec<u32>{
            numbers_str.split(" ").filter(|n_str| n_str.len() > 0).map(|n_str| n_str.parse::<u32>().unwrap()).collect()
        };
        let winning_numbers = extract_numbers(iterator.next().unwrap());
        let have_numbers = extract_numbers(iterator.next().unwrap());
        (id, winning_numbers, have_numbers)
    }
    
    input.lines().map(|line| parse_line(line)).collect()
}

fn solve_part1(scratchcards: &Vec<Scratchcard>) -> u32 {
    let mut total = 0;
    for (_id, winning, have) in scratchcards {
        let mut worth = 0; 
        for w in winning{
            if have.iter().find(|&h| h == w).is_some(){
                if worth == 0{
                    worth = 1;
                }else{
                    worth *= 2;
                }
            }
        }
        total += worth;        
    }
    total
}

fn solve_part2(scratchcards: &Vec<Scratchcard>) -> usize {
    let mut scratchcard_copies: Vec<usize> = vec![0; scratchcards.len()];
    let last_id = scratchcards.last().unwrap().0;
    for (id, winning, have) in scratchcards {
        let wins = winning.iter().fold(0, |acc, w| {
            if have.iter().find(|&h| h == w).is_some(){
                return acc + 1
            }
            acc
        });
        for offset in 0..wins{
            let copy_id = id + offset + 1;
            scratchcard_copies[copy_id - 1] += 1+scratchcard_copies[id - 1];
            
            if last_id == copy_id{
                break;
            }
        }
    }
    scratchcard_copies.iter().sum::<usize>() + scratchcards.len()
}

fn main() {
    let input = include_str!("../input");
    let games = parse_input(input);

    println!("Part 1: {}", solve_part1(&games));
    println!("Part 2: {}", solve_part2(&games));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = 
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    macro_rules! example_scratchcards {
        () => {
            vec![
                (1, vec![41, 48, 83, 86, 17], vec![83, 86,  6, 31, 17,  9, 48, 53]),
                (2, vec![13, 32, 20, 16, 61], vec![61, 30, 68, 82, 17, 32, 24, 19]),
                (3, vec![ 1, 21, 53, 59, 44], vec![69, 82, 63, 72, 16, 21, 14,  1]),
                (4, vec![41, 92, 73, 84, 69], vec![59, 84, 76, 51, 58,  5, 54, 83]),
                (5, vec![87, 83, 26, 28, 32], vec![88, 30, 70, 12, 93, 22, 82, 36]),
                (6, vec![31, 18, 13, 56, 72], vec![74, 77, 10, 23, 35, 67, 36, 11]),
            ]
        };
    }

    #[test]
    fn test_parsing() {
        let scratchcards = example_scratchcards!();
        assert_eq!(parse_input(&EXAMPLE), scratchcards);
    }

    #[test]
    fn test_solve_part1() {
        let scratchcards = example_scratchcards!();
        assert_eq!(solve_part1(&scratchcards), 13);
    }

    #[test]
    fn test_solve_part2() {
        let scratchcards = example_scratchcards!();
        assert_eq!(solve_part2(&scratchcards), 30);
    }
}
