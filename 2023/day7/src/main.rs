pub mod part1;
pub mod part2;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut hands: Vec<(&str, usize)> = input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(h, b)| (h, b.parse().unwrap()))
        .collect();

    hands.sort_by(|a, b| crate::part1::compare_hands(a.0, b.0));
    let mut total_winnings = 0;
    for (index, (_, bid)) in hands.into_iter().enumerate() {
        total_winnings += (index + 1) * bid;
    }
    total_winnings
}

fn part2(input: &str) -> usize {
    let mut hands: Vec<(&str, usize)> = input
        .lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(h, b)| (h, b.parse().unwrap()))
        .collect();

    hands.sort_by(|a, b| crate::part2::compare_hands(a.0, b.0));
    let mut total_winnings = 0;
    for (index, (hand, bid)) in hands.into_iter().enumerate() {
        println!("{}. {} {}", index + 1, hand, bid);
        total_winnings += (index + 1) * bid;
    }
    total_winnings
}

pub enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let test_input = include_str!("../test_input.txt");
        assert_eq!(part1(test_input), 6440);
    }

    #[test]
    fn part2_works() {
        let test_input = include_str!("../test_input.txt");
        assert_eq!(part2(test_input), 5905);
    }
}
