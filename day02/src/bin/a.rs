type CubeSet = (u32, u32, u32);
type Game = (u32, Vec<CubeSet>);

fn parse_input(input: &str) -> Vec<Game> {
    fn parse_line(line: &str) -> Game {
        (0, vec![])
    }
    
    input.lines().map(|line| parse_line(line)).collect()
}

const N_RED: u32 = 12;
const N_GREEN: u32 = 13;
const N_BLUE: u32 = 14;

fn solve(games: &Vec<Game>) -> u32 {
    let mut total = 0;
    for (id, sets) in games {
        let mut possible = true;
        for &(red, green, blue) in sets {
            if red > N_RED || green > N_GREEN || blue > N_BLUE {
                possible = false;
            }
        }
        if possible{
            total += id;
        }
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

    const EXAMPLE: &'static str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    macro_rules! example_games {
        () => {
            vec![
                (1, vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)]),
                (2, vec![(0, 2, 1), (1, 3, 4), (0, 1, 1)]),
                (3, vec![(20, 8, 6), (4, 13, 5), (1, 5, 0)]),
                (4, vec![(3, 1, 6), (6, 3, 0), (14, 3, 15)]),
                (5, vec![(6, 3, 1), (1, 2, 2)]),
            ]
        };
    }

    #[test]
    fn test_parsing() {
        let games = example_games!();
        assert_eq!(parse_input(&EXAMPLE), games);
    }

    #[test]
    fn test_solve() {
        let games = example_games!();
        assert_eq!(solve(&games), 8);
    }
}
