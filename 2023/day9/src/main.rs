fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> isize {
    let mut sum = 0;
    for line in input.lines() {
        let history: Vec<isize> = line.split(' ').map(|num| num.parse().unwrap()).collect();
        let histories = extrapolate_down(history);
        let new_history = extrapolate_up(histories);
        sum += new_history.last().unwrap();
    }
    sum
}

fn part2(input: &str) -> isize {
    let mut sum = 0;
    for line in input.lines() {
        let history: Vec<isize> = line.split(' ').map(|num| num.parse().unwrap()).collect();
        let histories = extrapolate_down(history);
        let new_history = extrapolate_up_backwards(histories);
        sum += new_history.first().unwrap();
    }
    sum
}

fn extrapolate_down(history: Vec<isize>) -> Vec<Vec<isize>> {
    let mut histories = vec![history];
    while histories
        .last()
        .unwrap()
        .iter()
        .filter(|x| x != &&0)
        .count()
        != 0
    {
        let history = histories.last().unwrap();
        let mut new_history = Vec::new();
        for i in 0..history.len() - 1 {
            new_history.push(history[i + 1] - history[i]);
        }
        histories.push(new_history);
    }
    histories
}

fn extrapolate_up(mut histories: Vec<Vec<isize>>) -> Vec<isize> {
    histories.last_mut().unwrap().push(0);

    for i in (0..histories.len() - 1).rev() {
        let difference = *histories[i + 1].last().unwrap();
        let previous = *histories[i].last().unwrap();
        histories[i].push(previous + difference);
    }
    histories.first().unwrap().to_vec()
}

fn extrapolate_up_backwards(mut histories: Vec<Vec<isize>>) -> Vec<isize> {
    histories.last_mut().unwrap().push(0);

    for i in (0..histories.len() - 1).rev() {
        let difference = *histories[i + 1].first().unwrap();
        let next = *histories[i].first().unwrap();
        histories[i].insert(0, next - difference);
    }
    histories.first().unwrap().to_vec()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part1_works() {
        let test_input = include_str!("../test_input.txt");
        assert_eq!(part1(test_input), 114);
    }

    #[test]
    fn part2_works() {
        let test_input = include_str!("../test_input.txt");
        assert_eq!(part2(test_input), 2);
    }
}
