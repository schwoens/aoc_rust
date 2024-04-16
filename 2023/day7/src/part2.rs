use std::cmp::Ordering;

use crate::HandType;

pub fn get_hand_type(hand: &str) -> HandType {
    let joker_count = hand.matches('J').count();
    let hand: String = hand.chars().filter(|c| c != &'J').collect();
    let counts: Vec<usize> = hand.chars().map(|c| hand.matches(c).count()).collect();
    if counts.contains(&5) {
        return HandType::FiveOfAKind;
    }
    if counts.contains(&4) {
        if joker_count >= 1 {
            return HandType::FiveOfAKind;
        }
        return HandType::FourOfAKind;
    }
    if counts.contains(&3) {
        if joker_count >= 2 {
            return HandType::FiveOfAKind;
        }
        if joker_count >= 1 {
            return HandType::FourOfAKind;
        }
        if counts.contains(&2) {
            return HandType::FullHouse;
        }
        if joker_count >= 2 {
            return HandType::FullHouse;
        }
        return HandType::ThreeOfAKind;
    }
    if counts.iter().filter(|x| x == &&2).count() == 4 {
        if joker_count >= 1 {
            return HandType::FullHouse;
        }
        return HandType::TwoPair;
    }
    if counts.contains(&2) {
        if joker_count >= 3 {
            return HandType::FiveOfAKind;
        }
        if joker_count >= 2 {
            return HandType::FourOfAKind;
        }
        if joker_count >= 1 {
            return HandType::ThreeOfAKind;
        }
        return HandType::OnePair;
    }
    if joker_count >= 5 {
        return HandType::FiveOfAKind;
    }
    if joker_count >= 4 {
        return HandType::FiveOfAKind;
    }
    if joker_count >= 3 {
        return HandType::FourOfAKind;
    }
    if joker_count >= 2 {
        return HandType::ThreeOfAKind;
    }
    if joker_count >= 1 {
        return HandType::OnePair;
    }
    HandType::HighCard
}

pub fn compare_hands(a: &str, b: &str) -> Ordering {
    let a_type = get_hand_type(a) as u8;
    let b_type = get_hand_type(b) as u8;

    if a_type == b_type {
        let a = a.chars().map(|c| get_card_value(&c)).collect::<String>();
        let b = b.chars().map(|c| get_card_value(&c)).collect::<String>();
        return a.cmp(&b);
    }
    a_type.cmp(&b_type)
}

pub fn get_card_value(card: &char) -> char {
    match card {
        'J' => 'a',
        '2' => 'b',
        '3' => 'c',
        '4' => 'd',
        '5' => 'e',
        '6' => 'f',
        '7' => 'g',
        '8' => 'h',
        '9' => 'i',
        'T' => 'j',
        'Q' => 'k',
        'K' => 'l',
        'A' => 'm',
        _ => panic!("Invalid card!"),
    }
}
