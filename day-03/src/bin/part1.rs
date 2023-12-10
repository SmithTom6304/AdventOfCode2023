use std::{env, fs};

use day_03::data::EngineSchematic;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("Reading from file '{}'", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let height = contents.lines().count();
    let width = (contents.len() / height) as u8;
    let height = height as u8;
    let contents = contents.replace("\n", "");
    let schematic = EngineSchematic::new(contents.chars().collect(), (width, height))
        .expect("Failed creating schematic from input");
    let sum: u32 = schematic
        .part_numbers
        .iter()
        .filter(|part| part.is_adjacent_to_symbol(&schematic))
        .map(|part| part.number as u32)
        .sum();
    println!("Result = {}", sum);
}

#[cfg(test)]
mod tests {
    use std::fs;

    use day_03::data::EngineSchematic;

    #[test]
    fn can_sum_engine_parts() {
        let file_path = "res/test_data.txt";
        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");
        let height = contents.lines().count();
        let width = (contents.len() / height) as u8;
        let height = height as u8;
        let contents = contents.replace("\n", "");
        let schematic = EngineSchematic::new(contents.chars().collect(), (width, height))
            .expect("Failed creating schematic from input");

        assert_eq!(
            2,
            schematic
                .part_numbers
                .iter()
                .filter(|part| false == part.is_adjacent_to_symbol(&schematic))
                .count()
        );

        let sum: u32 = schematic
            .part_numbers
            .iter()
            .filter(|part| part.is_adjacent_to_symbol(&schematic))
            .map(|part| part.number as u32)
            .sum();
        assert_eq!(4361, sum);
    }
}
