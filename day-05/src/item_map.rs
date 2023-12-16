#[derive(Debug)]
pub struct ItemMapEntry {
    pub destination_range_start: u64,
    pub source_range_start: u64,
    pub range_length: u64,
}

impl ItemMapEntry {
    pub fn from_line(line: &str) -> Self {
        let mut parts = line.split_whitespace();
        let destination_range_start = parts
            .next()
            .expect("No destination range start provided")
            .parse::<u64>()
            .expect("Could not parse destination range start as u64");
        let source_range_start = parts
            .next()
            .expect("No source range start provided")
            .parse::<u64>()
            .expect("Could not source range start as u64");
        let range_length = parts
            .next()
            .expect("No range length provided")
            .parse::<u64>()
            .expect("Could not parse range length as u64");
        Self {
            destination_range_start,
            source_range_start,
            range_length,
        }
    }
}

#[derive(Debug)]
pub struct ItemMap {
    pub id: String,
    pub entries: Vec<ItemMapEntry>,
}

impl ItemMap {
    pub fn get(mappings: &Vec<ItemMapEntry>, item: &u64) -> u64 {
        for map in mappings.iter() {
            if map.source_range_start <= *item && *item < map.source_range_start + map.range_length
            {
                let offset = *item - map.source_range_start;
                return map.destination_range_start + offset;
            }
        }
        item.clone()
    }
}
