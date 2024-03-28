use std::cmp::Ordering;

fn parse_input(input: &str) -> Vec<(&str, u64)> {
    fn extract(line: &str) -> (&str, u64){
        let mut it = line.split(" ").filter(|s| s.len() > 0);
        let hand = it.next().unwrap();
        let bid = it.next().unwrap().parse::<u64>().unwrap();
        (hand, bid)
    }
    input.lines().map(|line| extract(line)).collect()
}

fn card_strength(card: char) -> u64{
    match card{
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => panic!("unknown card")
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
enum HandType{
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn hand_type(hand: &str) -> HandType{
    let mut hand_bytes = [0; 5];
    hand_bytes.clone_from_slice(&hand.as_bytes());
    hand_bytes.sort();
    let hand = hand_bytes;
    let mut groups = vec![1];
    for (i, _) in hand.iter().enumerate().skip(1){
        if hand[i] == hand[i-1]{
            *(groups.last_mut().unwrap()) += 1;
        }else{
            groups.push(1);
        }
    }
    groups.sort_by(|a, b| b.cmp(a));
    match groups.len(){
        1 => HandType::FiveOfAKind,
        2 => if groups[0] == 3{
            HandType::FullHouse
        }else{
            HandType::FourOfAKind
        },
        3 => {
            if groups[0] == 3{
                HandType::ThreeOfAKind
            }else{
                HandType::TwoPair
            }
        },
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => panic!("invalid hand length")
    }
}

fn cmp_by_strength(a: &(&str, u64), b: &(&str, u64)) -> Ordering{
    let a_hand_type = hand_type(a.0);
    let b_hand_type = hand_type(b.0);
    if a_hand_type < b_hand_type{
        Ordering::Less
    }else if a_hand_type > b_hand_type{
        Ordering::Greater
    }else{
        let a_hand = a.0.as_bytes();
        let b_hand = b.0.as_bytes();
        assert_eq!(a_hand.len(), b_hand.len());
        for i in 0..a_hand.len(){
            let a_card_strength = card_strength(a_hand[i] as char);
            let b_card_strength = card_strength(b_hand[i] as char);
            if a_card_strength < b_card_strength{
                return Ordering::Less;
            }else if b_card_strength < a_card_strength{
                return Ordering::Greater;
            }else{
                continue;
            }
        }
        Ordering::Equal
    }
}

fn solve_part1(hands: &Vec<(&str, u64)>) -> u64{
    let mut hands = hands.clone();
    hands.sort_by(cmp_by_strength); 
    hands.iter().enumerate().fold(0, |acc, (i, (_, bid))|acc + bid*(i as u64+1))
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
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    macro_rules! example_hands {
        () => {
            vec![("32T3K", 765), ("T55J5", 684), ("KK677", 28), ("KTJJT", 220), ("QQQJA", 483)]
        };
    }

    #[test]
    fn test_parsing() {
        let hands = example_hands!();
        assert_eq!(parse_input(&EXAMPLE), hands);
    }

    #[test]
    fn test_hand_type() {
        let hands = example_hands!();
        let hand_types = vec![HandType::OnePair, HandType::ThreeOfAKind, HandType::TwoPair, HandType::TwoPair, HandType::ThreeOfAKind];
        for (i, (hand, _)) in hands.iter().enumerate(){
            assert_eq!(hand_type(&hand), hand_types[i]);
        } 
        
    }

    #[test]
    fn test_solve_part1() {
        let hands = example_hands!();
        assert_eq!(solve_part1(&hands), 6440);
    }
}
