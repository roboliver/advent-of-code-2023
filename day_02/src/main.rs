use std::{cmp, fs};

fn main() {
    let input = read_input();
    println!("Part 1: {}", possible_id_sum(&input));
    println!("Part 2: {}", power_sum(&input));
}

fn possible_id_sum(input: &str) -> u32 {
    let game_mins = game_mins(input);
    game_mins.iter()
        .filter(|game_mins| game_mins.within_maxes(12, 13, 14))
        .map(|game_mins| game_mins.id)
        .sum()
}

fn power_sum(input: &str) -> u32 {
    let game_mins = game_mins(input);
    game_mins.iter()
        .map(|game_mins| game_mins.power())
        .sum()
}

fn game_mins(input: &str) -> Vec<GameMins> {
    input.lines()
        .map(|line| game_mins_for_line(line))
        .collect()
}

fn game_mins_for_line(line: &str) -> GameMins {
    let parts: Vec<&str> = line.split(":").collect();
    let game: &str = parts.first().expect("No game number part");
    let id = game_id(&game);
    let draws: Vec<&str> = parts.last().expect("No draws part")
        .split(";").collect();
    let red_min = min_draws(&draws, "red");
    let green_min = min_draws(&draws, "green");
    let blue_min = min_draws(&draws, "blue");
    GameMins { id, red_min, green_min, blue_min }
}

fn game_id(game: &str) -> u32 {
    let game_id_str = *game.split(" ").collect::<Vec<&str>>()
        .last().expect("No game ID number part");
    game_id_str.parse().expect("Game ID was not a number")
}

fn min_draws(draws: &[&str], colour: &str) -> u32 {
    let mut min = 0;
    draws.iter()
        .for_each(|draw| min = cmp::max(min, number_of_colour_drawn(draw, colour)));
    min
}

fn number_of_colour_drawn(draw: &str, colour: &str) -> u32 {
    let draw_colour_parts: Vec<&str> = draw.split(",").collect();
    draw_colour_parts.iter()
        .filter(|draw_colour_part| draw_colour_part.contains(colour))
        .map(|colour_part| colour_count(colour_part))
        .sum()
}

fn colour_count(colour_part: &str) -> u32 {
    colour_part.trim()
        .split(" ")
        .collect::<Vec<&str>>()
        .first()
        .expect("colour found, but no number part")
        .parse::<u32>()
        .expect("could not interpret number part as a number")
}

struct GameMins {
    id: u32,
    red_min: u32,
    green_min: u32,
    blue_min: u32
}

impl GameMins {
    fn within_maxes(&self, red: u32, green: u32, blue: u32) -> bool {
        self.red_min <= red && self.green_min <= green && self.blue_min <= blue
    }

    fn power(&self) -> u32 {
        self.red_min * self.green_min * self.blue_min
    }
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
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, possible_id_sum(input));
    }

    #[test]
    fn part_2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(2286, power_sum(input));
    }
}
