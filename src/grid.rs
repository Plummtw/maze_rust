#![allow(dead_code)]
use std::{rc::Rc, cell::RefCell};
use std::fmt;
use std::fmt::{Debug, Formatter};
use rand::Rng;

use super::cell::*;

pub struct Grid {
  pub rows: usize,
  pub columns: usize,
  pub grid: Vec<Vec<CellLink>>
}

impl Grid {
  pub fn initialize(rows: usize, columns: usize) -> Self {
    let mut grid = Vec::new();
    
    for row in 0..rows {
      let mut row_grid = Vec::new();
      for column in 0..columns {
        row_grid.push(Rc::new(RefCell::new(Cell::initialize(row, column))));
      }
      grid.push(row_grid);
    }

    Grid {
      rows,
      columns,
      grid
    }
  }

  pub fn configure_cells(&mut self) {
    for row in &self.grid {
      for cell in row {
        let mut cell = cell.borrow_mut();
        let row = cell.row;
        let column = cell.column;

        if row >= 1 {
          cell.north =  self.get_cell(row-1, column);
        }

        if row < self.rows-1 {
          cell.south =  self.get_cell(row+1, column);
        }

        if column >= 1 {
          cell.west =  self.get_cell(row, column-1);
        }

        if column < self.columns-1 {
          cell.east =  self.get_cell(row, column+1);
        }
      }
    }

  }

// (defn- configure-cells [self]
//   (each-cell [cell self]
//              (let [row (:row @cell) column (:column @cell)]
//               ;;  (println (str row column))
//                (dosync
//                 (swap! cell assoc :north (get-cell self (dec row) column)
//                        :south (get-cell self (inc row) column)
//                        :west (get-cell self row (dec column))
//                        :east (get-cell self row (inc column))))))
//   self)  

  pub fn get_cell(&self, row: usize, column: usize) -> Option<CellLink> {
    if  row >= self.rows || column >= self.columns {
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

        let north = &cell.north;
        match north {
          Some(north) => {
            let north = north.borrow();
            println!("north: row: {}, column: {}", north.row, north.column);
          }
          None => (),
        }

        let south = &cell.south;
        match south {
          Some(south) => {
            let south = south.borrow();
            println!("south: row: {}, column: {}", south.row, south.column);
          }
          None => (),
        }

        let east = &cell.east;
        match east {
          Some(east) => {
            let east = east.borrow();
            println!("east: row: {}, column: {}", east.row, east.column);
          }
          None => (),
        }

        let west = &cell.west;
        match west {
          Some(west) => {
            let west = west.borrow();
            println!("west: row: {}, column: {}", west.row, west.column);
          }
          None => (),
        }
      },
      None => (),
    }
  }

  pub fn random_cell(&self) -> Option<CellLink> {
    let mut rng = rand::thread_rng();
    let row: usize = rng.gen();
    let column: usize = rng.gen();
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
      let mut bottom_str = String::from("-");
      for cell in row {
        let cell = &cell.borrow();
        top_str.push_str("   ");
        match cell.east {
          Some(ref east) => {
            if let Some(_) = cell.linked(east.clone()) {
              top_str.push(' ');
            } else {
              top_str.push('|');
            }
          },
          None => top_str.push('|'),
        }

        bottom_str.push('-');
        match cell.south {
          Some(ref south) => {
            if let Some(_) = cell.linked(south.clone()) {
              bottom_str.push_str("   ");
            } else {
              bottom_str.push_str("---");
            }
          },
          None => bottom_str.push_str("---"),
        }
      }
      writeln!(f, "{}", top_str)?;
      writeln!(f, "{}", bottom_str)?;
    }

    Ok(())
  }
}


