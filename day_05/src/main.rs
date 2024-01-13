use std::fs;

fn main() {
    let input = read_input();
    println!("Part 1: {}", lowest_location_number(&input));
    println!("Part 2: {}", lowest_location_number_actual(&input));
}

fn lowest_location_number(input: &str) -> u64 {
    let seeds = seed_line(&input);
    let maps = vec!(
        AlmanacMap::new(&input, "seed-to-soil"),
        AlmanacMap::new(&input, "soil-to-fertilizer"),
        AlmanacMap::new(&input, "fertilizer-to-water"),
        AlmanacMap::new(&input, "water-to-light"),
        AlmanacMap::new(&input, "light-to-temperature"),
        AlmanacMap::new(&input, "temperature-to-humidity"),
        AlmanacMap::new(&input, "humidity-to-location"));
    seeds.iter()
        .map(|seed| do_mapping(*seed, &maps))
        .min()
        .expect("no seeds")
}

fn lowest_location_number_actual(input: &str) -> u64 {
    0
}

fn do_mapping(seed: u64, maps: &Vec<AlmanacMap>) -> u64 {
    maps.iter()
        .fold(seed, |acc, map| map.map(acc))
}

fn seed_line(input: &str) -> Vec<u64> {
    let seed_line = input.lines()
        .next()
        .expect("no seeds line");
    seed_line["seeds: ".len()..].split(" ")
        .map(|num| num.parse().expect("not a number"))
        .collect()
}

struct AlmanacMap {
    mappings: Vec<AlmanacMapping>
}

impl AlmanacMap {
    fn new(input: &str, map_type: &str) -> Self {
        let map_type_line = format!("{} map:\n", map_type);
        let from_type = input.split(&map_type_line).last().expect("map type not found");
        let sections_from: Vec<&str> = from_type.split("\n\n").collect();
        let lines = sections_from.first().expect("map section didn't end");
        let mappings = lines.lines()
            .map(|line| AlmanacMapping::new(line))
            .collect();
        Self { mappings }
    }

    fn map(&self, input: u64) -> u64 {
        for mapping in &self.mappings {
            match mapping.map(input) {
                Some(result) => return result,
                None => continue
            }
        }
        input
    }
}

struct AlmanacMapping {
    src: u64,
    dst: u64,
    len: u64
}

impl AlmanacMapping {
    fn new(line: &str) -> Self {
        let parts: Vec<u64> = line.split(" ")
            .map(|part| part.parse().expect("not a number"))
            .collect();
        Self { src: parts[1], dst: parts[0], len: parts[2] }
    }

    fn map(&self, input: u64) -> Option<u64> {
        if input >= self.src && input < self.src + self.len {
            return Some(self.dst + (input - self.src))
        }
        None
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
        let input = "seeds: 79 14 55 13\n\
        \n\
        seed-to-soil map:\n\
        50 98 2\n\
        52 50 48\n\
        \n\
        soil-to-fertilizer map:\n\
        0 15 37\n\
        37 52 2\n\
        39 0 15\n\
        \n\
        fertilizer-to-water map:\n\
        49 53 8\n\
        0 11 42\n\
        42 0 7\n\
        57 7 4\n\
        \n\
        water-to-light map:\n\
        88 18 7\n\
        18 25 70\n\
        \n\
        light-to-temperature map:\n\
        45 77 23\n\
        81 45 19\n\
        68 64 13\n\
        \n\
        temperature-to-humidity map:\n\
        0 69 1\n\
        1 0 69\n\
        \n\
        humidity-to-location map:\n\
        60 56 37\n\
        56 93 4";
        assert_eq!(35, lowest_location_number(input));
    }

    fn part_2() {
        let input = "seeds: 79 14 55 13\n\
        \n\
        seed-to-soil map:\n\
        50 98 2\n\
        52 50 48\n\
        \n\
        soil-to-fertilizer map:\n\
        0 15 37\n\
        37 52 2\n\
        39 0 15\n\
        \n\
        fertilizer-to-water map:\n\
        49 53 8\n\
        0 11 42\n\
        42 0 7\n\
        57 7 4\n\
        \n\
        water-to-light map:\n\
        88 18 7\n\
        18 25 70\n\
        \n\
        light-to-temperature map:\n\
        45 77 23\n\
        81 45 19\n\
        68 64 13\n\
        \n\
        temperature-to-humidity map:\n\
        0 69 1\n\
        1 0 69\n\
        \n\
        humidity-to-location map:\n\
        60 56 37\n\
        56 93 4";
        assert_eq!(46, lowest_location_number_actual(input));
    }
}