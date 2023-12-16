use day_05::item_map::ItemMap;

fn main() {}

#[cfg(test)]
mod tests {
    use day_05::item_map::{ItemMap, ItemMapEntry};
    use rstest::rstest;

    #[rstest]
    #[case(79, 81)]
    #[case(14, 14)]
    #[case(55, 57)]
    #[case(13, 13)]
    fn can_map_seeds_to_soil(#[case] seed: u8, #[case] soil: u8) {
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