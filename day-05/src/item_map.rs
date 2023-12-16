pub struct ItemMapEntry {
    pub destination_range_start: u8,
    pub source_range_start: u8,
    pub range_length: u8,
}

pub struct ItemMap {}

impl ItemMap {
    pub fn get(mappings: &Vec<ItemMapEntry>, item: &u8) -> u8 {
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
