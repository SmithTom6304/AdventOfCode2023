pub struct ItemMapEntry {
    pub destination_range_start: u8,
    pub source_range_start: u8,
    pub range_length: u8,
}

pub struct ItemMap {
    entries: Vec<ItemMapEntry>,
}

impl ItemMap {
    pub fn new() -> Self {
        let entries = vec![];
        ItemMap { entries }
    }

    pub fn add(&mut self, entry: ItemMapEntry) {
        self.entries.push(entry)
    }

    pub fn get(&self, item: &u8) -> u8 {
        for map in self.entries.iter() {
            if map.source_range_start <= *item && *item < map.source_range_start + map.range_length
            {
                let offset = *item - map.source_range_start;
                return map.destination_range_start + offset;
            }
        }
        item.clone()
    }
}
