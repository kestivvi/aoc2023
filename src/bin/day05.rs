use aoc2023::{read_input, InputType};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use timed::timed;

const DAY: u8 = 5;

fn main() {
    let real_input = read_input(DAY, InputType::Real).unwrap();

    println!("Part1: {}", part1(&real_input));
    println!("Part2: {}", part2(&real_input));
}

#[derive(Debug, Default)]
struct Map {
    map: HashMap<std::ops::Range<i64>, i64>,
    // source_range: std::ops::Range<i64>,
    // destination_offset: i64,
}

impl Map {
    fn get(&self, source: i64) -> i64 {
        let found_map = self.map.iter().find(|(k, _)| k.contains(&source));
        match found_map {
            Some((_, destination_offset)) => source + destination_offset,
            None => source,
        }
    }

    fn insert(&mut self, destination_start: i64, source_start: i64, range: i64) {
        let range = source_start..(source_start + range);
        // dbg!(&self, &destination_start, &source_start, &range);
        let destination_offset = destination_start - source_start;
        self.map.insert(range, destination_offset);
    }
}

fn get_min_location(seeds: Vec<i64>, map_of_maps: HashMap<&str, Map>) -> i64 {
    seeds
        .iter()
        .map(|seed| {
            let soil = map_of_maps.get("seed-to-soil").unwrap().get(*seed);
            // dbg!(&soil);
            let fertilizer = map_of_maps.get("soil-to-fertilizer").unwrap().get(soil);
            let water = map_of_maps
                .get("fertilizer-to-water")
                .unwrap()
                .get(fertilizer);
            let light = map_of_maps.get("water-to-light").unwrap().get(water);
            let temperature = map_of_maps.get("light-to-temperature").unwrap().get(light);
            let humidity = map_of_maps
                .get("temperature-to-humidity")
                .unwrap()
                .get(temperature);
            let location = map_of_maps
                .get("humidity-to-location")
                .unwrap()
                .get(humidity);
            dbg!(location)
            // location
        })
        .min()
        .unwrap()
        .clone()
}

// fn get_min_location2(seeds: impl Iterator<Item = i64>, map_of_maps: HashMap<&str, Map>) -> i64 {
//     seeds

// }

fn get_map<'a>(categories: &'a [&'a str]) -> HashMap<&'a str, Map> {
    categories
        .iter()
        .map(|category| {
            let (name, numbers_part) = category.split(" map:").collect_tuple().unwrap();
            // dbg!(name);
            let map = numbers_part
                .trim()
                .lines()
                .map(|line| {
                    let (destination_start, source_start, range) = line
                        .trim()
                        .split_whitespace()
                        .flat_map(|v| v.parse::<i64>())
                        .collect_tuple()
                        .unwrap();
                    // dbg!((destination_start, source_start, range));

                    // let map = (0..range).fold(HashMap::new(), |mut map, index| {
                    //     map.insert(source_start + index, destination_start + index);
                    //     map
                    // });
                    (destination_start, source_start, range)
                })
                .fold(
                    Map::default(),
                    |mut map, (destination_start, source_start, range)| {
                        map.insert(destination_start, source_start, range);
                        // dbg!(map)
                        map
                    },
                );
            (name, map)
        })
        .fold(HashMap::new(), |mut map_of_maps, (name, map)| {
            map_of_maps.insert(name, map);
            map_of_maps
        })
}

#[timed]
fn part1(input: &str) -> i64 {
    let v = input.split("\r\n\r\n").collect_vec();
    let seeds = v
        .first()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .flat_map(|v| v.parse::<i64>())
        .collect_vec();

    // dbg!(&seeds);

    let collect_vec = v.iter().skip(1).copied().collect_vec();
    let map_of_maps = get_map(&collect_vec);
    get_min_location(seeds, map_of_maps)
}

#[timed]
fn part2(input: &str) -> i64 {
    let v = input.split("\r\n\r\n").collect_vec();

    // dbg!(&seeds.size_hint());

    let collect_vec = v.iter().skip(1).copied().collect_vec();
    let map_of_maps = get_map(&collect_vec);
    v.first()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .flat_map(|v| v.parse::<i64>())
        .tuples()
        .collect_vec()
        .par_iter()
        .flat_map(|(start, range)| (*start..(*start + *range)).into_iter())
        .map(|seed| {
            let soil = map_of_maps.get("seed-to-soil").unwrap().get(seed);
            // dbg!(&soil);
            let fertilizer = map_of_maps.get("soil-to-fertilizer").unwrap().get(soil);
            let water = map_of_maps
                .get("fertilizer-to-water")
                .unwrap()
                .get(fertilizer);
            let light = map_of_maps.get("water-to-light").unwrap().get(water);
            let temperature = map_of_maps.get("light-to-temperature").unwrap().get(light);
            let humidity = map_of_maps
                .get("temperature-to-humidity")
                .unwrap()
                .get(temperature);
            let location = map_of_maps
                .get("humidity-to-location")
                .unwrap()
                .get(humidity);
            location
            // location
        })
        .min()
        .unwrap()
        .clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        read_input(DAY, InputType::Test).unwrap()
    }

    #[test]
    fn part1_test() {
        let expected = 35;
        let result = part1(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn part1_real() {
        let expected = 196167384;
        let result = part1(&read_input(DAY, InputType::Real).unwrap());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let expected = 46;
        let result = part2(&get_test_input());
        assert_eq!(result, expected);
    }
}
