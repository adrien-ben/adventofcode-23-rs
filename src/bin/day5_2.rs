use std::{ops::Range, str::FromStr};

use adventofcode_23_rs::util;

fn main() {
    let result = Day5::parse().min_location();

    println!("{result}"); // 79874951
}

#[derive(Debug, Default)]
struct Day5 {
    seeds: Vec<Range<u64>>,
    seed_to_soil_map: Vec<MapEntry>,
    soil_to_fertilizer: Vec<MapEntry>,
    fertilizer_to_water: Vec<MapEntry>,
    water_to_light: Vec<MapEntry>,
    light_to_temperature: Vec<MapEntry>,
    temperature_to_humidity: Vec<MapEntry>,
    humidity_to_location: Vec<MapEntry>,
}

impl Day5 {
    fn parse() -> Self {
        #[derive(Debug, PartialEq, Eq)]
        enum ParseState {
            Soil,
            Fertilizer,
            Water,
            Light,
            Temp,
            Humidity,
            Location,
        }

        let mut day5 = Self::default();

        let mut iter = util::lines("inputs/day5.txt").filter(|l| !l.is_empty());

        let mut min = None;
        iter.next()
            .unwrap()
            .replacen("seeds: ", "", 1)
            .split_ascii_whitespace()
            .map(|v| v.parse::<u64>().unwrap())
            .for_each(|v| {
                if let Some(min) = min.take() {
                    day5.seeds.push(min..(min + v));
                } else {
                    min = Some(v);
                }
            });

        let mut state = ParseState::Soil;
        for line in iter {
            if line == "seed-to-soil map:" {
                state = ParseState::Soil;
            } else if line == "soil-to-fertilizer map:" {
                state = ParseState::Fertilizer;
            } else if line == "fertilizer-to-water map:" {
                state = ParseState::Water;
            } else if line == "water-to-light map:" {
                state = ParseState::Light;
            } else if line == "light-to-temperature map:" {
                state = ParseState::Temp;
            } else if line == "temperature-to-humidity map:" {
                state = ParseState::Humidity;
            } else if line == "humidity-to-location map:" {
                state = ParseState::Location;
            } else if state == ParseState::Soil {
                day5.seed_to_soil_map.push(line.parse().unwrap());
            } else if state == ParseState::Fertilizer {
                day5.soil_to_fertilizer.push(line.parse().unwrap());
            } else if state == ParseState::Water {
                day5.fertilizer_to_water.push(line.parse().unwrap());
            } else if state == ParseState::Light {
                day5.water_to_light.push(line.parse().unwrap());
            } else if state == ParseState::Temp {
                day5.light_to_temperature.push(line.parse().unwrap());
            } else if state == ParseState::Humidity {
                day5.temperature_to_humidity.push(line.parse().unwrap());
            } else if state == ParseState::Location {
                day5.humidity_to_location.push(line.parse().unwrap());
            }
        }

        day5
    }

    fn min_location(self) -> u64 {
        for location in 0..u64::MAX {
            let humidity = Self::map(&self.humidity_to_location, location);
            let temp = Self::map(&self.temperature_to_humidity, humidity);
            let light = Self::map(&self.light_to_temperature, temp);
            let water = Self::map(&self.water_to_light, light);
            let fertilizer = Self::map(&self.fertilizer_to_water, water);
            let soil = Self::map(&self.soil_to_fertilizer, fertilizer);
            let seed = Self::map(&self.seed_to_soil_map, soil);

            if self.seeds.iter().any(|r| r.contains(&seed)) {
                return location;
            }
        }

        panic!("not found !")
    }

    fn map(map: &[MapEntry], value: u64) -> u64 {
        let r = map.iter().find_map(|e| e.map(value)).unwrap_or(value);
        r
    }
}

#[derive(Debug, Clone, Copy)]
struct MapEntry {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl MapEntry {
    fn map(&self, value: u64) -> Option<u64> {
        if value >= self.destination_start && value < self.destination_start + self.length {
            Some(self.source_start + (value - self.destination_start))
        } else {
            None
        }
    }
}

impl FromStr for MapEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_ascii_whitespace();

        Ok(Self {
            destination_start: it.next().unwrap().parse().unwrap(),
            source_start: it.next().unwrap().parse().unwrap(),
            length: it.next().unwrap().parse().unwrap(),
        })
    }
}
