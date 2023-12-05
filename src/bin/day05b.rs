use aoc2023::{read_input, InputType};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use timed::timed;

const DAY: u8 = 5;

fn main() {
    // let real_input = read_input(DAY, InputType::Real).unwrap();

    // println!("Part2: {}", part2(&real_input));

    let expected = 125742456;
    let result = part2(&read_input(DAY, InputType::Real).unwrap());
    dbg!(&result);
    assert_eq!(result, expected);
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

    fn reverse_get(&self, destination: i64) -> Option<i64> {
        let found_map = self.map.iter().find(|(source_range, destination_offset)| {
            // dbg!((source_range, destination_offset));
            let source_start = source_range.start;
            let destination_start = source_start + **destination_offset;
            let destination_end = source_range.end + **destination_offset; // Adjust the end of the range

            // dbg!((source_start, destination_start, destination_end));

            destination >= destination_start && destination <= destination_end
        });

        match found_map {
            Some((_, destination_offset)) => Some(destination - destination_offset),
            None => None,
        }
    }

    fn insert(&mut self, destination_start: i64, source_start: i64, range: i64) {
        let range = source_start..(source_start + range);
        // dbg!(&self, &destination_start, &source_start, &range);
        let destination_offset = destination_start - source_start;
        self.map.insert(range, destination_offset);
    }
}

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
fn part2(input: &str) -> i64 {
    let v = input.split("\r\n\r\n").collect_vec();

    let collect_vec = v.iter().skip(1).copied().collect_vec();
    let map_of_maps = get_map(&collect_vec);
    // let mut seeds = v
    //     .first()
    //     .unwrap()
    //     .split(":")
    //     .last()
    //     .unwrap()
    //     .trim()
    //     .split_whitespace()
    //     .flat_map(|v| v.parse::<i64>())
    //     .tuples()
    //     .flat_map(|(start, range)| (start..(start + range)).into_iter());

    // .collect_vec()
    // .par_iter()
    // .flat_map(|(start, range)| (*start..(*start + *range)).into_iter());

    //     .map(|seed| {
    //         let soil = map_of_maps.get("seed-to-soil").unwrap().get(seed);
    //         // dbg!(&soil);
    //         let fertilizer = map_of_maps.get("soil-to-fertilizer").unwrap().get(soil);
    //         let water = map_of_maps
    //             .get("fertilizer-to-water")
    //             .unwrap()
    //             .get(fertilizer);
    //         let light = map_of_maps.get("water-to-light").unwrap().get(water);
    //         let temperature = map_of_maps.get("light-to-temperature").unwrap().get(light);
    //         let humidity = map_of_maps
    //             .get("temperature-to-humidity")
    //             .unwrap()
    //             .get(temperature);
    //         let location = map_of_maps
    //             .get("humidity-to-location")
    //             .unwrap()
    //             .get(humidity);
    //         location
    //         // location
    //     })
    //     .min()
    //     .unwrap()
    //     .clone()

    // dbg!(&seeds);

    let seeds = v
        .first()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .flat_map(|v| v.parse::<i64>())
        .tuples()
        .map(|(start, range)| (start..(start + range)))
        .collect_vec();

    // let locations = 0..;

    (0..i64::MAX)
        .into_par_iter()
        .find_first(|location| {
            // dbg!(location);
            let humidity = map_of_maps
                .get("humidity-to-location")
                .unwrap()
                .reverse_get(*location)
                .unwrap_or(*location);
            // dbg!(humidity);

            let temperature = map_of_maps
                .get("temperature-to-humidity")
                .unwrap()
                .reverse_get(humidity)
                .unwrap_or(humidity);
            // dbg!(temperature);

            let light = map_of_maps
                .get("light-to-temperature")
                .unwrap()
                .reverse_get(temperature)
                .unwrap_or(temperature);
            // dbg!(light);

            let water = map_of_maps
                .get("water-to-light")
                .unwrap()
                .reverse_get(light)
                .unwrap_or(light);
            // dbg!(water);

            let fertilizer = map_of_maps
                .get("fertilizer-to-water")
                .unwrap()
                .reverse_get(water)
                .unwrap_or(water);
            // dbg!(fertilizer);

            let soil = map_of_maps
                .get("soil-to-fertilizer")
                .unwrap()
                .reverse_get(fertilizer)
                .unwrap_or(fertilizer);
            // dbg!(soil);

            let seed = map_of_maps
                .get("seed-to-soil")
                .unwrap()
                .reverse_get(soil)
                .unwrap_or(soil);
            // dbg!(seed);

            if location % 1_000_000 == 0 {
                dbg!(location);
            }

            // seeds.clone().contains(&seed)
            seeds.iter().any(|range| range.contains(&seed))
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_input() -> String {
        read_input(DAY, InputType::Test).unwrap()
    }

    #[test]
    fn part2_test() {
        let expected = 46;
        let result = part2(&get_test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_real() {
        let expected = 125742456;
        let result = part2(&read_input(DAY, InputType::Real).unwrap());
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_wj() {
        let expected = 20358599;
        let result = part2(&read_input(DAY, InputType::Other("WJ".to_string())).unwrap());
        assert_eq!(result, expected);
    }
}
