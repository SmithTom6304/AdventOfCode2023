use anyhow::{bail, Result};
use std::{collections::HashSet, fmt};

pub struct EngineSchematic {
    data: Vec<char>,
    size: (u8, u8),
    part_numbers: Vec<PartNumber>,
    symbols: Vec<Symbol>,
}

#[derive(Debug, Clone)]
struct SizeError;
impl fmt::Display for SizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec size does not match provided size value")
    }
}

#[derive(Debug, Clone)]
struct OutOfBoundsError;
impl fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cell was out of bounds")
    }
}

impl EngineSchematic {
    pub fn new(data: Vec<char>, size: (u8, u8)) -> Result<Self> {
        if data.len() != (size.0 as u32 * size.1 as u32) as usize {
            bail!(SizeError);
        }

        let mut part_numbers = vec![];
        let mut symbols = vec![];
        let mut current_part_indices = vec![];

        let mut i: u32 = 0;
        while i < data.len() as u32 {
            let (column, row) = Self::index_to_coordinate(i, &size);
            let c = data.get(i as usize).expect("Data out of range");
            if (!Self::char_is_part_number(c) || i % size.0 as u32 == 0)
                && !current_part_indices.is_empty()
            {
                part_numbers.push(Self::create_part_number_from_indices(
                    row,
                    &current_part_indices,
                    &size,
                    &data,
                ));
                current_part_indices.clear();
            }
            if Self::char_is_part_number(c) {
                current_part_indices.push(i);
            } else if Self::char_is_symbol(c) {
                symbols.push(Symbol {
                    symbol: c.clone(),
                    row,
                    column,
                });
            }
            i += 1;
        }

        if !current_part_indices.is_empty() {
            // Check for any remaining part numbers
            let row = Self::index_to_coordinate(i, &size).1;
            part_numbers.push(Self::create_part_number_from_indices(
                row,
                &current_part_indices,
                &size,
                &data,
            ));
            current_part_indices.clear();
        }

        Ok(EngineSchematic {
            data,
            size,
            part_numbers,
            symbols,
        })
    }

    fn create_part_number_from_indices(
        row: u8,
        indices: &Vec<u32>,
        size: &(u8, u8),
        data: &Vec<char>,
    ) -> PartNumber {
        let columns = indices
            .iter()
            .map(|ind| Self::index_to_coordinate(*ind, &size).1)
            .collect();
        PartNumber {
            number: Self::indices_to_string(indices, &data)
                .parse()
                .expect("Error parsing part number"),
            row,
            columns,
        }
    }

    fn char_is_part_number(c: &char) -> bool {
        c.is_numeric()
    }

    pub fn char_is_symbol(c: &char) -> bool {
        !Self::char_is_part_number(c) && c != &'.'
    }

    fn in_bounds(&self, cell: &(u8, u8)) -> bool {
        cell.0 < self.size.0 && cell.1 < self.size.1
    }

    fn index_to_coordinate(index: u32, size: &(u8, u8)) -> (u8, u8) {
        let height = index / size.0 as u32;
        let width = index % size.0 as u32;
        (width as u8, height as u8)
    }

    fn indices_to_string(indices: &Vec<u32>, data: &Vec<char>) -> String {
        indices
            .iter()
            .map(|i| {
                data.get(*i as usize)
                    .expect("Error creating string from indices")
            })
            .collect()
    }

    pub fn at(&self, cell: &(u8, u8)) -> Result<&char> {
        if !self.in_bounds(cell) {
            bail!(OutOfBoundsError)
        }
        let index = (self.size.0 as u32 * cell.1 as u32) as usize + cell.0 as usize;
        Ok(self.data.get(index).expect("Index was not in bounds"))
    }

    pub fn get_adjacent_cells(&self, cell: &(u8, u8)) -> Vec<(u8, u8)> {
        let mut cell_indexes = vec![];
        let offsets: Vec<(i32, i32)> = vec![
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        for offset in offsets {
            let cell_index = match Self::try_get_cell_from_offset(cell, &offset) {
                Some(val) => val,
                None => continue,
            };
            if !self.in_bounds(&cell_index) {
                continue;
            }
            cell_indexes.push(cell_index);
        }
        cell_indexes
    }

    fn try_get_cell_from_offset(cell: &(u8, u8), offset: &(i32, i32)) -> Option<(u8, u8)> {
        let x = cell.0 as i32 + offset.0;
        let y = cell.1 as i32 + offset.1;
        if x < 0 || y < 0 {
            return None;
        }
        Some((x as u8, y as u8))
    }
}

#[derive(Debug, PartialEq)]
pub struct PartNumber {
    pub number: u32,
    pub row: u8,
    pub columns: Vec<u8>,
}

impl PartNumber {
    pub fn is_adjacent_to_symbol(&self, schematic: &EngineSchematic) -> bool {
        let mut cells_to_check = HashSet::new();
        for column in self.columns.iter() {
            let cell = (column.clone(), self.row);
            let adjacent_cells = schematic.get_adjacent_cells(&cell);
            adjacent_cells
                .into_iter()
                .for_each(|adj| _ = cells_to_check.insert(adj));
        }
        cells_to_check.iter().any(|cell| {
            let c = schematic.at(cell).expect("Error indexing into schematic");
            EngineSchematic::char_is_symbol(c)
        })
    }
}

pub struct Symbol {
    pub symbol: char,
    pub row: u8,
    pub column: u8,
}

#[cfg(test)]
mod an_engine_schematic {
    use super::EngineSchematic;

    #[test]
    fn is_created_from_a_vec_and_a_size() {
        let data = vec!['.', '&', '.', '1', '.', '.', '.', '.', '6'];
        let size: (u8, u8) = (3, 3);

        let schematic = match EngineSchematic::new(data.clone(), size.clone()) {
            Ok(sch) => sch,
            Err(err) => panic!("{}", err),
        };
        assert_eq!(data, schematic.data);
        assert_eq!(size, schematic.size);
        assert_eq!(2, schematic.part_numbers.len());
        assert_eq!(1, schematic.symbols.len());
    }
}

#[cfg(test)]
mod a_part_number {
    use super::EngineSchematic;

    #[test]
    fn can_determine_if_adjacent_to_symbol() {
        let data = vec!['.', '&', '.', '1', '.', '.', '.', '.', '6'];
        let size: (u8, u8) = (3, 3);

        let schematic = match EngineSchematic::new(data.clone(), size.clone()) {
            Ok(sch) => sch,
            Err(err) => panic!("{}", err),
        };

        let part_number_1 = schematic
            .part_numbers
            .iter()
            .find(|pn| pn.number == 1)
            .expect("Could not find part number 1");
        let part_number_6 = schematic
            .part_numbers
            .iter()
            .find(|pn| pn.number == 6)
            .expect("Could not find part number 6");
        assert!(part_number_1.is_adjacent_to_symbol(&schematic));
        assert!(false == part_number_6.is_adjacent_to_symbol(&schematic));
    }
}
