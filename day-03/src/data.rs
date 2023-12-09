use anyhow::{bail, Result};
use std::{
    cell::{RefCell, RefMut},
    fmt,
    rc::Rc,
};

pub struct EngineSchematic {
    data: Vec<char>,
    size: (u8, u8),
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
        if data.len() != (size.0 * size.1) as usize {
            bail!(SizeError);
        }

        Ok(EngineSchematic { data, size })
    }

    fn in_bounds(&self, cell: &(u8, u8)) -> bool {
        cell.0 < self.size.0 && cell.1 < self.size.1
    }

    pub fn at(&self, cell: &(u8, u8)) -> Result<&char> {
        if !self.in_bounds(cell) {
            bail!(OutOfBoundsError)
        }
        let index = (self.size.0 * cell.1) as usize + cell.0 as usize;
        Ok(self.data.get(index).expect("Index was not in bounds"))
    }

    pub fn get_adjacent_part_numbers(&self, cell: (u8, u8)) -> Vec<PartNumber> {
        let mut part_numbers: Vec<PartNumber> = vec![];
        let shared_adjacent_cells_to_check =
            Rc::new(RefCell::new(self.get_adjacent_cell_indexes(&cell)));
        let binding = Rc::clone(&shared_adjacent_cells_to_check);
        let binding = binding.borrow();
        let mut iter = binding.iter();
        while iter.len() > 0 {
            let cell = match iter.next() {
                None => break,
                Some(val) => val,
            };
            let binding = Rc::clone(&shared_adjacent_cells_to_check);
            let mut adjacent_cells = binding.borrow_mut();
            adjacent_cells.retain(|check| check != cell);
            let binding = Rc::clone(&shared_adjacent_cells_to_check);
            let part_number = match self.get_part_number(cell, binding) {
                None => continue,
                Some(pn) => pn,
            };
            part_numbers.push(part_number);
        }
        part_numbers
    }

    fn get_adjacent_cell_indexes(&self, cell: &(u8, u8)) -> Vec<(u8, u8)> {
        let mut cell_indexes = vec![];
        let offsets: Vec<(i8, i8)> = vec![
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

    fn try_get_cell_from_offset(cell: &(u8, u8), offset: &(i8, i8)) -> Option<(u8, u8)> {
        let x = cell.0 as i8 + offset.0;
        let y = cell.1 as i8 + offset.1;
        if x < 0 || y < 0 {
            return None;
        }
        Some((x as u8, y as u8))
    }

    fn get_part_number(
        &self,
        cell: &(u8, u8),
        cells_to_check: Rc<RefCell<Vec<(u8, u8)>>>,
    ) -> Option<PartNumber> {
        let value = self.get_part_number_string_from_cell(cell);
        let value = match value {
            None => return None,
            Some(val) => val,
        };

        let mut cells_to_check = cells_to_check.borrow_mut();
        cells_to_check.retain(|check| check != cell);
        let mut part_number_string = value.to_string();
        let mut columns = vec![cell.1];

        if let Some(offset_cell) = Self::try_get_cell_from_offset(cell, &(-1, 0)) {
            if let Some(part_char) = self.get_part_number_string_from_cell(&offset_cell) {
                part_number_string.insert(0, part_char);
                cells_to_check.retain(|check| check != cell);
                columns.insert(0, offset_cell.1);
            }
        }

        let number = part_number_string
            .parse::<u8>()
            .expect("Error parsing part_number to u8");
        Some(PartNumber {
            number,
            row: cell.0,
            columns: columns,
        })
    }

    fn get_part_number_string_from_cell(&self, cell: &(u8, u8)) -> Option<char> {
        if !self.in_bounds(cell) {
            return None;
        }
        let value = self.at(cell);
        let value = match value {
            Err(err) => {
                println!("{}", err);
                return None;
            }
            Ok(val) => val,
        };
        if !value.is_numeric() {
            return None;
        }
        Some(*value)
    }
}

#[derive(Debug, PartialEq)]
pub struct PartNumber {
    pub number: u8,
    pub row: u8,
    pub columns: Vec<u8>,
}

pub struct Symbol {
    pub symbol: char,
    pub row: u8,
    pub column: u8,
}

#[cfg(test)]
mod an_engine_schematic {
    use crate::data::PartNumber;

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
    }

    #[test]
    fn can_get_adjacent_part_numbers() {
        let data = vec!['.', '&', '.', '1', '2', '.', '.', '.', '6'];
        let size: (u8, u8) = (3, 3);
        let expected_part_number = PartNumber {
            number: 12,
            row: 1,
            columns: vec![0, 1],
        };

        let schematic = match EngineSchematic::new(data.clone(), size.clone()) {
            Ok(sch) => sch,
            Err(err) => panic!("{}", err),
        };

        let part_numbers = schematic.get_adjacent_part_numbers((0, 1));
        let number = part_numbers.first().unwrap();
        // assert_eq!(expected_part_number.number, number.number);
        // assert_eq!(expected_part_number.row, number.row);
        // assert_eq!(expected_part_number.columns, number.columns);
        assert_eq!(&expected_part_number, number);
    }
}
