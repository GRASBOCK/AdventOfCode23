type Mapping = (u64, u64, u64);
type Category = Vec<Mapping>;
type Seeds = Vec<u64>;
type Almanac = (Seeds, [Category; 7]);

fn category_map(category: &Category, val: u64) -> u64{
    let mut mapping = None;
    for m in category{
        let (_, source_start, len) = *m;
        if source_start <= val && val <= source_start + len{
            // within mapping
            mapping = Some(m); 
        }
    }
    if let Some((dest_start, source_start, _)) = mapping{
        let offset = val - source_start;
        dest_start + offset 
    }else{
        val
    }
}

fn parse_input(input: &str) -> (Seeds, [Category; 7]) {
    fn parse_seeds(line: &str) -> Seeds {
        line.split(":").skip(1).next().unwrap().split(" ").filter(|s| s.len() > 0).map(|s| s.parse::<u64>().unwrap()).collect()
    }
    fn parse_mapping(line: &str) -> Mapping {
        let values: Vec<u64> = line.split(" ").filter(|s| s.len() > 0).map(|s| s.parse::<u64>().unwrap()).collect();
        (values[0], values[1], values[2])
    }
    
    let first_line = input.lines().take(1).next().unwrap();
    let categories: Vec<Category> = input.split("\n\n").skip(1).map(|category_str| 
        category_str.split(":").skip(1).next().unwrap().lines().filter(|s| s.len() > 0).map(|line| parse_mapping(line)).collect()
    ).collect();
    let seeds: Seeds = parse_seeds(first_line);
    input.split(":\n");
    let mut a: [Category; 7] = Default::default();
    a.clone_from_slice(&categories[0..7]);
    (seeds, a)
}

fn solve_part1(almanac: &Almanac) -> u64 {
    let mut results = vec![[0; 8]; almanac.0.len()];
    for (i, &seed) in almanac.0.iter().enumerate() {
        let vals = &mut results[i];
        vals[0] = seed;
        for (j, category) in almanac.1.iter().enumerate() {
            vals[j+1] = category_map(&category, vals[j])
        }
        //println!("Seed {}, soil {}, fertilizer {}, water {}, light {}, temperature {}, humidity {}, location {}", vals[0], vals[1], vals[2], vals[3], vals[4], vals[5], vals[6], vals[7]);      
    }
    results.iter().map(|vals| vals[7]).min().unwrap()
}

fn main() {
    let input = include_str!("../input");
    let almanac = parse_input(input);

    println!("Part 1: {}", solve_part1(&almanac));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = 
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    macro_rules! example_almanac {
        () => {
            (vec![79, 14, 55, 13], 
            [
                vec![(50, 98,  2), (52, 50, 48)],
                vec![( 0, 15, 37), (37, 52,  2), (39, 0, 15)],
                vec![(49, 53,  8), ( 0, 11, 42), (42, 0, 7), (57, 7, 4)],
                vec![(88, 18,  7), (18, 25, 70)],
                vec![(45, 77, 23), (81, 45, 19), (68, 64, 13)],
                vec![( 0, 69,  1), ( 1,  0, 69)],
                vec![(60, 56, 37), (56, 93,  4)],
            ])
        };
    }

    #[test]
    fn test_parsing() {
        let almanac = example_almanac!();
        assert_eq!(parse_input(&EXAMPLE), almanac);
    }

    #[test]
    fn test_solve_part1() {
        let almanac = example_almanac!();
        assert_eq!(solve_part1(&almanac), 35);
    }
}
