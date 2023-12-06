use std::collections::HashSet;
use std::fs;

fn main() {
    let input = read_input();
    println!("Part 1: {}", card_points_sum(&input));
    println!("Part 2: {}", card_copies_sum(&input));
}

fn card_copies_sum(input: &str) -> u32 {
    let mut card_copies: Vec<u32> = input.lines().enumerate()
        .map(|_| 1)
        .collect();
    input.lines().enumerate()
        .for_each(|(i, line)| {
            let line_wins = line_wins(line);
            for j in (i + 1)..(i + 1 + usize::try_from(line_wins).unwrap()) {
                if j <= card_copies.len() {
                    card_copies[j] += card_copies[i];
                }
            }
        });
    card_copies.iter()
        .sum()
}

fn card_points_sum(input: &str) -> u32 {
    input.lines()
        .map(|line| line_points_sum(line))
        .sum()
}

fn line_wins(line: &str) -> u32 {
    let parts: Vec<&str> = line.split("|").collect();
    let winning_numbers = winning_numbers(parts.first().expect("no winning numbers"));
    let my_numbers = my_numbers(parts.last().expect("no my numbers"));
    let wins = my_numbers.iter()
        .filter(|num| winning_numbers.contains(num))
        .count();
    u32::try_from(wins).unwrap()
}

fn line_points_sum(line: &str) -> u32 {
    let wins = line_wins(line);
    if wins == 0 {
        return 0
    }
    let init: u32 = 2;
    init.pow(wins - 1) as u32
}

fn winning_numbers(winning_numbers_str: &str) -> HashSet<u32> {
    let parts = winning_numbers_str.split(":").collect::<Vec<&str>>();
    trim_and_parse_numbers(parts.last().expect("no numbers part")).iter()
        .map(|num| *num)
        .collect()
}

fn my_numbers(my_numbers_str: &str) -> Vec<u32> {
    trim_and_parse_numbers(my_numbers_str)
}

fn trim_and_parse_numbers(numbers_str: &str) -> Vec<u32> {
    numbers_str.split(" ").collect::<Vec<&str>>().iter()
        .map(|num| num.trim())
        .filter(|num| !num.is_empty())
        .map(|num| num.parse().expect("number was not a number"))
        .collect()
}

fn read_input() -> String {
    fs::read_to_string("input.txt")
        .expect("Could not read file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(13, card_points_sum(input));
    }

    #[test]
    fn part_2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(30, card_copies_sum(input));
    }
}