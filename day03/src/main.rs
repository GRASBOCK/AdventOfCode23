use std::collections::BTreeMap;

fn part_numbers(input: &str) -> Vec<u64> {
    let line_width = input.find('\n').unwrap() + 1;
    let input = input.as_bytes();

    let max_x = line_width - 1;
    let max_y = input.len() / line_width - 1;
    let xy_2_index = |x: i64, y: i64| -> Option<usize> {
        if x < 0 || y < 0 || y > max_y as i64 || x > max_x as i64 {
            None
        } else {
            Some(x as usize + y as usize * line_width)
        }
    };
    let index_2_xy = |idx: usize| -> (i64, i64) {
        let x = (idx % line_width) as i64;
        let y = (idx / line_width) as i64;
        (x, y)
    };
    let mut part_numbers = vec![];
    let mut digits: Vec<char> = vec![];
    let mut is_part_number = false;
    for (idx, c) in input.iter().enumerate() {
        if c.is_ascii_digit() {
            digits.push(*c as char);
            let (x, y) = index_2_xy(idx);
            let is_symbol = |x: i64, y: i64| -> bool {
                if let Some(idx) = xy_2_index(x, y) {
                    let other = input[idx];
                    if other != b'.' && !other.is_ascii_digit() && other != b'\n' {
                        // it's a symbol
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            };
            let offsets = [
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ];
            let has_symbol = offsets
                .iter()
                .map(|offset| {
                    let (other_x, other_y) = (x + offset.0, y + offset.1);

                    is_symbol(other_x, other_y)
                })
                .any(|val| val);
            if has_symbol {
                is_part_number = true;
            };
        } else {
            if !digits.is_empty() && is_part_number {
                let part_number_str: String = digits.iter().collect();
                part_numbers.push(part_number_str.parse::<u64>().unwrap());
            }
            digits.clear();
            is_part_number = false;
        }
    }

    part_numbers
}

fn solve_part1(input: &str) -> u64 {
    part_numbers(input).iter().sum()
}

fn solve_part2(input: &str) -> u64 {
    let line_width = input.find('\n').unwrap() + 1;
    let input = input.as_bytes();

    let max_x = line_width - 1;
    let max_y = input.len() / line_width - 1;
    let xy_2_index = |x: i64, y: i64| -> Option<usize> {
        if x < 0 || y < 0 || y > max_y as i64 || x > max_x as i64 {
            None
        } else {
            Some(x as usize + y as usize * line_width)
        }
    };
    let index_2_xy = |idx: usize| -> (i64, i64) {
        let x = (idx % line_width) as i64;
        let y = (idx / line_width) as i64;
        (x, y)
    };
    let mut gears: BTreeMap<usize, Vec<u64>> = BTreeMap::new();
    let mut digits: Vec<char> = vec![];
    let mut gear_index = None;
    for (idx, c) in input.iter().enumerate() {
        if c.is_ascii_digit() {
            digits.push(*c as char);
            let (x, y) = index_2_xy(idx);
            let is_symbol = |x: i64, y: i64| -> Option<usize> {
                if let Some(idx) = xy_2_index(x, y) {
                    let other = input[idx];
                    if other == b'*' {
                        // it's a gear symbol
                        Some(idx)
                    } else {
                        None
                    }
                } else {
                    None
                }
            };
            let offsets = [
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ];
            let has_symbol = offsets
                .iter()
                .map(|offset| {
                    let (other_x, other_y) = (x + offset.0, y + offset.1);

                    is_symbol(other_x, other_y)
                })
                .find(|val| val.is_some());
            if let Some(gear_idx) = has_symbol {
                gear_index = Some(gear_idx.unwrap());
            };
        } else {
            if let Some(gear_idx) = gear_index {
                if !digits.is_empty() {
                    let part_number_str: String = digits.iter().collect();
                    let part_number = part_number_str.parse::<u64>().unwrap();
                    if let Some(part_numbers) = gears.get_mut(&gear_idx) {
                        part_numbers.push(part_number);
                    } else {
                        gears.insert(gear_idx, vec![part_number]);
                    }
                }
            }
            gear_index = None;
            digits.clear();
        }
    }

    //println!("{:?}", gears);
    gears
        .values()
        .filter_map(|numbers| {
            if numbers.len() == 2 {
                Some(numbers[0] * numbers[1])
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", solve_part1(input));
    println!("Part 2: {}", solve_part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn test_part_numbers() {
        assert_eq!(
            part_numbers(EXAMPLE),
            vec![467, 35, 633, 617, 592, 755, 664, 598]
        );
    }

    #[test]
    fn test_solve_part1() {
        assert_eq!(solve_part1(EXAMPLE), 4361);
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2(EXAMPLE), 467835);
    }
}
