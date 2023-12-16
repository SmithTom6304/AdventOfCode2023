use std::{collections::VecDeque, env, fs};

use day_05::item_map::{ItemMap, ItemMapEntry};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("Reading from file '{}'", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut lines = contents.lines();

    // Get seeds
    let seed_line = lines.next().expect("Input had no seed line");
    let seed_line = &seed_line[7..];
    let seeds = seed_line
        .split_whitespace()
        .map(|seed| seed.parse::<u64>().expect("Could not parse seed as u64"));
    let mut seed_ranges = vec![];
    let mut seeds = seeds.into_iter();
    while let Some(seed) = seeds.next() {
        let range = seeds.next().expect("Odd number of seed values");
        seed_ranges.push((seed, range));
    }

    _ = lines.next();

    let mut maps = VecDeque::<ItemMap>::new();
    let line = lines.next().unwrap();
    let mut map_id = &line[..line.len() - 1];
    let mut current_map = ItemMap {
        id: map_id.to_string(),
        entries: vec![],
    };
    while let Some(line) = lines.next() {
        if line.contains("map") {
            maps.push_back(current_map);
            map_id = &line[..line.len() - 1];
            current_map = ItemMap {
                id: map_id.to_string(),
                entries: vec![],
            };
        } else if line.len() == 0 {
            continue;
        } else {
            current_map.entries.push(ItemMapEntry::from_line(line));
        }
    }
    maps.push_back(current_map);

    let lowest_location_number = seed_ranges
        .iter()
        .flat_map(|seed_range| {
            let (seed, range) = *seed_range;
            let result = (seed..(seed + range))
                .map(|seed| map_continuous(&maps, &seed))
                .min();
            result
        })
        .min()
        .expect("Map was empty");
    print!("Result = {}", lowest_location_number);
}

fn map_continuous(maps: &VecDeque<ItemMap>, seed: &u64) -> u64 {
    let mut result = *seed;
    for map in maps.iter() {
        result = ItemMap::get(&map.entries, &result);
    }
    result
}
