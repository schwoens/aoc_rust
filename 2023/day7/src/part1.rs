use std::cmp::Ordering;

use crate::HandType;

pub fn get_hand_type(hand: &str) -> HandType {
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
