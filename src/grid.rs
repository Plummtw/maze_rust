#![allow(dead_code)]
use rand::Rng;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::{cell::RefCell, rc::Rc};

use super::cell::*;
use super::distance::*;

pub struct Grid {
    pub rows: usize,
    pub columns: usize,
    pub grid: Vec<Vec<CellLink>>,
    pub distance: Option<Distance>,
}

impl Grid {
    pub fn new(rows: usize, columns: usize) -> Self {
        let mut grid = Vec::new();

        for row in 0..rows {
            let mut row_grid = Vec::new();
            for column in 0..columns {
                row_grid.push(Rc::new(RefCell::new(Cell::new(row, column))));
            }
            grid.push(row_grid);
        }

        Grid {
            rows,
            columns,
            grid,
            distance: None
        }
    }

    pub fn configure_cells(&mut self) {
        for row in &self.grid {
            for cell in row {
                let mut cell = cell.borrow_mut();
                let row = cell.row;
                let column = cell.column;

                if row >= 1 {
                    cell.north = self.get_cell(row - 1, column).map(|v| v.downgrade());
                }

                if row < self.rows - 1 {
                    cell.south = self.get_cell(row + 1, column).map(|v| v.downgrade());
                }

                if column >= 1 {
                    cell.west = self.get_cell(row, column - 1).map(|v| v.downgrade());
                }

                if column < self.columns - 1 {
                    cell.east = self.get_cell(row, column + 1).map(|v| v.downgrade());
                }
            }
        }
    }

    pub fn set_distance(&mut self, distance: Distance) {
        self.distance = Some(distance);
    }

    pub fn get_cell(&self, row: usize, column: usize) -> Option<CellLink> {
        if row >= self.rows || column >= self.columns {
            None
        } else {
            Some(self.grid[row as usize][column as usize].clone())
        }
    }

    pub fn print_cells(&self, row: usize, column: usize) {
        match self.get_cell(row, column) {
            Some(cell) => {
                let cell = cell.borrow();
                println!("row: {}, column: {}", cell.row, cell.column);

                let north = &cell.north();
                match north {
                    Some(north) => {
                        let north = north.borrow();
                        println!("north: row: {}, column: {}", north.row, north.column);
                    }
                    None => (),
                }

                let south = &cell.south();
                match south {
                    Some(south) => {
                        let south = south.borrow();
                        println!("south: row: {}, column: {}", south.row, south.column);
                    }
                    None => (),
                }

                let east = &cell.east();
                match east {
                    Some(east) => {
                        let east = east.borrow();
                        println!("east: row: {}, column: {}", east.row, east.column);
                    }
                    None => (),
                }

                let west = &cell.west();
                match west {
                    Some(west) => {
                        let west = west.borrow();
                        println!("west: row: {}, column: {}", west.row, west.column);
                    }
                    None => (),
                }
            }
            None => (),
        }
    }

    pub fn random_cell(&self) -> Option<CellLink> {
        let mut rng = rand::thread_rng();
        let row: usize = rng.gen_range(0..self.rows);
        let column: usize = rng.gen_range(0..self.rows);
        self.get_cell(row, column)
    }

    pub fn size(&self) -> usize {
        self.rows * self.columns
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for _ in 0..self.columns {
            write!(f, "+---")?;
        }
        writeln!(f, "+")?;

        for row in &self.grid {
            let mut top_str = String::from("|");
            let mut bottom_str = String::from("+");
            for cell in row {
                let cell_borrowed = &cell.borrow();
                top_str.push(' ');
                let distance_str = match &self.distance {
                    Some(distance) => distance_str(distance.get(cell.clone())),
                    None => String::from(" "),
                };
                top_str.push_str(distance_str.as_str());
                top_str.push(' ');
                match cell_borrowed.east() {
                    Some(east) => {
                        if cell_borrowed.linked(east).is_some() {
                            top_str.push(' ');
                        } else {
                            top_str.push('|');
                        }
                    }
                    None => top_str.push('|'),
                }

                match cell_borrowed.south() {
                    Some(south) => {
                        if cell_borrowed.linked(south).is_some() {
                            bottom_str.push_str("   ");
                        } else {
                            bottom_str.push_str("---");
                        }
                    }
                    None => bottom_str.push_str("---"),
                }

                bottom_str.push('+');
            }
            writeln!(f, "{}", top_str)?;
            writeln!(f, "{}", bottom_str)?;
        }

        Ok(())
    }
}
