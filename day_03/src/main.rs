use std::fs;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = read_input();
    println!("Part 1: {}", schematic_part_sum(&input));
    println!("Part 2: {}", schematic_gear_ratio_sum(&input));
}

fn schematic_part_sum(input: &str) -> u32 {
    let empty_symbols_row: HashSet<usize> = HashSet::new();
    let symbols = symbols(input, any_symbol_matcher);
    let numbers = numbers(input);
    numbers.iter().enumerate()
        .map(|(i, numbers_row)| adjacent_part_sum(numbers_row,
        if i == 0 { &empty_symbols_row } else { &symbols[i-1] },
        &symbols[i],
        if i == symbols.len() - 1 { &empty_symbols_row } else { &symbols[i+1] }))
        .sum()
}

fn schematic_gear_ratio_sum(input: &str) -> u32 {
    let gears = symbols(input, gear_matcher);
    let numbers = numbers(input);
    let gears_to_numbers: HashMap<(usize, usize), Vec<u32>> = numbers.iter().enumerate()
        .map(|(i, numbers_row)| gears_to_adjacent_numbers(numbers_row, i, &gears))
        .flatten()
        .map(|(gear_row, gear_col, number)| ((gear_row, gear_col), number))
        .fold(HashMap::new(),
        |the_map, entry| {
            add_entry(the_map, entry)
        });
    gears_to_numbers.iter()
        .filter(|(_, v)| v.len() > 1)
        .map(|(_, v)| v.iter().product::<u32>())
        .sum()
}

fn add_entry(mut the_map: HashMap<(usize, usize), Vec<u32>>, entry: ((usize, usize), u32))
    -> HashMap<(usize, usize), Vec<u32>> {
    let numbers = the_map.entry(entry.0).or_default();
    numbers.push(entry.1);
    the_map
}

fn gears_to_adjacent_numbers(numbers_row: &Vec<(u32, usize)>, row: usize, gears: &Vec<HashSet<usize>>)
    -> Vec<(usize, usize, u32)> {
    let from_row = if row == 0 { 0 } else { row - 1};
    let to_row = if row == gears.len() - 1 { gears.len() } else { row + 2 };
    (from_row..to_row).collect::<Vec<usize>>().iter()
        .map(|row_number| (row_number, gears.get(*row_number).unwrap()))
        .map(|(row_number, gears_row)| adjacent_numbers(gears_row, numbers_row, *row_number))
        .flatten()
        .collect()
}

fn adjacent_numbers(gears_row: &HashSet<usize>, numbers_row: &Vec<(u32, usize)>, row_number: usize)
-> Vec<(usize, usize, u32)> {
    numbers_row.iter()
        .map(|(number, start_pos)| (number, number_span(*number, *start_pos)))
        .map(|(number, pos_s)| (number, filter_gear_hits(&pos_s, &gears_row)))
        .map(|(number, gear_hits)| create_hit_entries(*number, &gear_hits, row_number))
        .flatten()
        .collect()
}

fn create_hit_entries(number: u32, gear_hits: &[usize], row_number: usize) -> Vec<(usize, usize, u32)> {
    gear_hits.iter()
        .map(|pos| (row_number, *pos, number))
        .collect()
}

fn filter_gear_hits<'a>(pos_s: &'a Vec<usize>, gears_row: &'a HashSet<usize>) -> Vec<usize> {
    pos_s.iter()
        .filter(|pos| gears_row.contains(pos))
        .map(|pos| *pos)
        .collect()
}

fn number_span(number: u32, start_pos: usize) -> Vec<usize> {
    let number_len = number.to_string().len();
    let from_pos = if start_pos == 0 as usize { 0 } else { start_pos - 1 };
    let to_pos = start_pos + number_len + 1;
    (from_pos..to_pos).collect::<Vec<usize>>()
}

fn symbols(input: &str, symbol_matcher: fn(&char) -> bool) -> Vec<HashSet<usize>> {
    input.lines()
        .map(|line| symbols_from_line(line, symbol_matcher))
        .collect()
}

fn symbols_from_line(input: &str, symbol_matcher: fn(&char) -> bool) -> HashSet<usize> {
    input.chars().enumerate()
        .filter(|(_, c)| symbol_matcher(c))
        .map(|(i, _)| i)
        .collect()
}

fn any_symbol_matcher(c: &char) -> bool {
    !(c.is_digit(10) || c.eq(&'.'))
}

fn gear_matcher(c: &char) -> bool {
    c.eq(&'*')
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
    number_span(*number, *start_pos).iter()
        .any(|pos| prev_symbols_row.contains(pos)
            || same_symbols_row.contains(pos)
            || next_symbols_row.contains(pos))
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

    #[test]
    fn part_2() {
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
        assert_eq!(467835, schematic_gear_ratio_sum(input));
    }
}
