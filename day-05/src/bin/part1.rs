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
    let seeds: Vec<u64> = seed_line
        .split_whitespace()
        .map(|seed| seed.parse::<u64>().expect("Could not parse seed as u64"))
        .collect();
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

    let lowest_location_number = seeds
        .iter()
        .map(|seed| map_continuous(&maps, seed))
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

#[cfg(test)]
mod tests {
    use day_05::item_map::{ItemMap, ItemMapEntry};
    use rstest::rstest;

    #[rstest]
    #[case(79, 81)]
    #[case(14, 14)]
    #[case(55, 57)]
    #[case(13, 13)]
    fn can_map_seeds_to_soil(#[case] seed: u64, #[case] soil: u64) {
        let mut mappings = vec![];
        mappings.push(ItemMapEntry {
            destination_range_start: 50,
            source_range_start: 98,
            range_length: 2,
        });
        mappings.push(ItemMapEntry {
            destination_range_start: 52,
            source_range_start: 50,
            range_length: 48,
        });

        assert_eq!(soil, ItemMap::get(&mappings, &seed));
    }
}
