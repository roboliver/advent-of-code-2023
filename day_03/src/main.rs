use std::fs;
use std::collections::HashSet;

fn main() {
    let input = read_input();
    println!("Part 1: {}", schematic_part_sum(&input));
}

fn schematic_part_sum(input: &str) -> u32 {
    let empty_symbols_row: HashSet<usize> = HashSet::new();
    let symbols = symbols(input);
    let numbers = numbers(input);
    numbers.iter().enumerate()
        .map(|(i, numbers_row)| adjacent_part_sum(numbers_row,
        if i == 0 { &empty_symbols_row } else { &symbols[i-1] },
        &symbols[i],
        if i == symbols.len() - 1 { &empty_symbols_row } else { &symbols[i+1] }))
        .sum()
}

fn symbols(input: &str) -> Vec<HashSet<usize>> {
    input.lines()
        .map(|line| symbols_from_line(line))
        .collect()
}

fn symbols_from_line(input: &str) -> HashSet<usize> {
    input.chars().enumerate()
        .filter(|(_, char)| !(char.is_digit(10) || char.eq(&'.')))
        .map(|(i, _)| i)
        .collect()
}

fn numbers(input: &str) -> Vec<Vec<(u32, usize)>> {
    input.lines()
        .map(|line| numbers_from_line(line))
        .collect()
}

fn numbers_from_line(input: &str) -> Vec<(u32, usize)> {
    let mut numbers = Vec::new();
    let mut cur_number: Option<u32> = None;
    for (i, c) in input.chars()
        .chain(['.'].into_iter())
        .enumerate() {
        if c.is_digit(10) {
            let c_digit = c.to_digit(10).expect("not a digit");
            if cur_number.is_none() {
                cur_number = Some(c_digit);
            } else {
                cur_number = Some(cur_number.unwrap() * 10 + c_digit);
            }
        } else if cur_number.is_some() {
            let cur_number_unwrapped = cur_number.unwrap();
            numbers.push((cur_number_unwrapped,
                          i - cur_number_unwrapped.to_string().len()));
            cur_number = None;
        }
    };
    numbers
}

fn adjacent_part_sum(numbers_row: &Vec<(u32, usize)>, prev_symbols_row: &HashSet<usize>,
                       same_symbols_row: &HashSet<usize>, next_symbols_row: &HashSet<usize>) -> u32 {
    numbers_row.iter()
        .filter(|(number, start_pos)| is_adjacent(number, start_pos,
                                                  prev_symbols_row, same_symbols_row, next_symbols_row))
        .map(|(number, _)| number)
        .sum()
}

fn is_adjacent(number: &u32, start_pos: &usize, prev_symbols_row: &HashSet<usize>,
                 same_symbols_row: &HashSet<usize>, next_symbols_row: &HashSet<usize>) -> bool {
    let number_len = number.to_string().len();
    let from_pos = if *start_pos == 0 as usize { 0 } else { *start_pos - 1 };
    let to_pos = *start_pos + number_len + 1;
    let result = (from_pos..to_pos).collect::<Vec<usize>>().iter()
        .any(|pos| prev_symbols_row.contains(pos)
                || same_symbols_row.contains(pos)
                || next_symbols_row.contains(pos));
    result
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
        let input = "467..114..\n\
        ...*......\n\
        ..35..633.\n\
        ......#...\n\
        617*......\n\
        .....+.58.\n\
        ..592.....\n\
        ......755.\n\
        ...$.*....\n\
        .664.598..";
        assert_eq!(4361, schematic_part_sum(input));
    }
}
