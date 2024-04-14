use core::panic;
use std::cmp::Ordering;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> usize {
    let mut hands: Vec<(&str, usize)> = input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(h, b)| (h, b.parse().unwrap()))
        .collect();

    hands.sort_by(|a, b| compare_hands(a.0, b.0));
    let mut total_winnings = 0;
    for (index, (_, bid)) in hands.into_iter().enumerate() {
        total_winnings += (index + 1) * bid;
    }
    total_winnings
}

enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

fn get_hand_type(hand: &str) -> HandType {
    let counts: Vec<usize> = hand.chars().map(|c| hand.matches(c).count()).collect();
    if counts.contains(&5) {
        return HandType::FiveOfAKind;
    }
    if counts.contains(&4) {
        return HandType::FourOfAKind;
    }
    if counts.contains(&3) {
        if counts.contains(&2) {
            return HandType::FullHouse;
        }
        return HandType::ThreeOfAKind;
    }
    if counts.iter().filter(|x| x == &&2).count() == 4 {
        return HandType::TwoPair;
    }
    if counts.contains(&2) {
        return HandType::OnePair;
    }
    HandType::HighCard
}

fn compare_hands(a: &str, b: &str) -> Ordering {
    let a_type = get_hand_type(a) as u8;
    let b_type = get_hand_type(b) as u8;

    if a_type == b_type {
        let a = a.chars().map(|c| get_card_value(&c)).collect::<String>();
        let b = b.chars().map(|c| get_card_value(&c)).collect::<String>();
        return a.cmp(&b);
    }
    a_type.cmp(&b_type)
}

fn get_card_value(card: &char) -> char {
    match card {
        '2' => 'a',
        '3' => 'b',
        '4' => 'c',
        '5' => 'd',
        '6' => 'e',
        '7' => 'f',
        '8' => 'g',
        '9' => 'h',
        'T' => 'i',
        'J' => 'j',
        'Q' => 'k',
        'K' => 'l',
        'A' => 'm',
        _ => panic!("Invalid card!"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let test_input = include_str!("../test_input.txt");
        assert_eq!(part1(test_input), 6440);
    }
}
