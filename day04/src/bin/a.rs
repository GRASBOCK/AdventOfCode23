type Scratchcard = (u32, Vec<u32>, Vec<u32>);

fn parse_input(input: &str) -> Vec<Scratchcard> {
    fn parse_line(line: &str) -> Scratchcard {
        (0, vec![], vec![])
    }
    
    input.lines().map(|line| parse_line(line)).collect()
}

fn solve(scratchcards: &Vec<Scratchcard>) -> u32 {
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

fn main() {
    let input = include_str!("../../input");
    let games = parse_input(input);

    println!("{}", solve(&games));
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
    fn test_solve() {
        let scratchcards = example_scratchcards!();
        assert_eq!(solve(&scratchcards), 13);
    }
}
