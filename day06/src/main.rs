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

fn number_of_ways(race: &(u64, u64)) -> u64{
    let p = - (race.0 as f64);
    let q = race.1 as f64;
    fn pq_formel(p: f64, q: f64) -> (f64, f64){
        let a = -p/2.0;
        let b = ((p/2.0).powi(2) - q).sqrt();
        (a + b, a - b)
    }
    let (v_max, v_min) = pq_formel(p, q);
    let (t_press_min, t_press_max) = ((v_min + 0.55).round() as u64, (v_max-0.55).round() as u64);
    t_press_max - t_press_min + 1
}

fn solve_part1(races: &Vec<(u64, u64)>) -> u64{
    races.iter().map(|r| number_of_ways(r)).fold(1, |acc, x| acc*x)
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
    fn test_nways() {
        let races = example_races!();
        let win_ways = vec![4, 8, 9];
        for (i, r) in races.iter().enumerate(){
            assert_eq!(number_of_ways(&r), win_ways[i]);
        }
        
    }

    #[test]
    fn test_solve_part1() {
        let races = example_races!();
        assert_eq!(solve_part1(&races), 288);
    }
}
