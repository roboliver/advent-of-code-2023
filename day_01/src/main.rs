use std::{cmp, fs};

fn main() {
    let input = read_input();
    println!("Part 1: {}", calibration_sum(&input));
    println!("Part 2: {}", calibration_sum_actual(&input));
}

fn calibration_sum(input: &str) -> u32 {
    input.lines()
        .map(|line| calibration_value(line))
        .sum()
}

fn calibration_value(line: &str) -> u32 {
    let digits = line.chars()
        .filter(|char| char.is_digit(10))
        .map(|char| char.to_digit(10).expect("Not a digit"))
        .collect::<Vec<u32>>();
    10 * digits.first().expect("No first digit") + digits.last().expect("No last digit")
}

fn calibration_sum_actual(input: &str) -> u32 {
    input.lines()
        .map(|line| calibration_value_actual(line))
        .sum()
}

fn calibration_value_actual(line: &str) -> u32 {
    let digits = digits();
    let first_matches = matches_ordered(line, &digits, first_in_line);
    let last_matches = matches_ordered(line, &digits, last_in_line);
    let first_digit = first_matches.first().expect("No first digit found").0.numeric;
    let last_digit = last_matches.last().expect("No last digit found").0.numeric;
    first_digit * 10 + last_digit
}

fn matches_ordered<'a>(line: &'a str, digits: &'a Vec<Digit>,
                         finder: fn(&'a Digit, &'a str) -> (&'a Digit, Option<usize>))
    -> Vec<(&'a Digit, usize)> {
    let mut matches: Vec<(&Digit, usize)> = digits.iter()
        .map(|digit| finder(digit, line))
        .filter(|result| result.1.is_some())
        .map(|result| (result.0, result.1.unwrap()))
        .collect();
    matches.sort_by(|a, b| a.1.cmp(&b.1));
    matches
}

fn first_in_line<'a>(digit: &'a Digit, line: &'a str) -> (&'a Digit, Option<usize>) {
    let first_word = line.find(&digit.word);
    let first_numeric = line.find(digit.numeric_as_chars());
    (digit, best_match(first_word, first_numeric, cmp::min))
}

fn last_in_line<'a>(digit: &'a Digit, line: &'a str) -> (&'a Digit, Option<usize>) {
    let last_word = line.rfind(&digit.word);
    let last_numeric = line.rfind(digit.numeric_as_chars());
    (digit, best_match(last_word, last_numeric, cmp::max))
}

fn best_match(word_match: Option<usize>, numeric_match: Option<usize>,
              comparator: fn(usize, usize) -> usize) -> Option<usize> {
    match word_match {
        None => match numeric_match {
            None => None,
            Some(i) => Some(i)
        },
        Some (i) => match numeric_match {
            None => Some(i),
            Some(_) => Some(comparator(word_match.unwrap(), numeric_match.unwrap()))
        }
    }
}

struct Digit {
    word: String,
    numeric: u32
}

impl Digit {
    fn new(word: &str, numeric: u32) -> Self {
        Self { word: String::from(word), numeric }
    }

    fn numeric_as_chars(&self) -> char {
        char::from_digit(self.numeric, 10).unwrap()
    }
}

fn digits() -> Vec<Digit> {
    vec!(Digit::new("one", 1),
         Digit::new("two", 2),
         Digit::new("three", 3),
         Digit::new("four", 4),
         Digit::new("five", 5),
         Digit::new("six", 6),
         Digit::new("seven", 7),
         Digit::new("eight", 8),
         Digit::new("nine", 9))
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
        let input = "1abc2\n\
        pqr3stu8vwx\n\
        a1b2c3d4e5f\n\
        treb7uchet";
        assert_eq!(142, calibration_sum(input));
    }

    #[test]
    fn part_2() {
        let input = "two1nine\n\
        eightwothree\n\
        abcone2threexyz\n\
        xtwone3four\n\
        4nineeightseven2\n\
        zoneight234\n\
        7pqrstsixteen";
        assert_eq!(281, calibration_sum_actual(input));
    }
}
