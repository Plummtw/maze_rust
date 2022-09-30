#![allow(dead_code)]
use std::{cell::RefCell, rc::Rc};

pub type CellLink = Rc<RefCell<Cell>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub row: usize,
    pub column: usize,
    pub north: Option<CellLink>,
    pub south: Option<CellLink>,
    pub east: Option<CellLink>,
    pub west: Option<CellLink>,
    pub links: Vec<CellLink>,
}

impl Cell {
    pub fn initialize(row: usize, column: usize) -> Self {
        Cell {
            row,
            column,
            north: None,
            south: None,
            east: None,
            west: None,
            links: Vec::new(),
        }
    }

    pub fn linked(&self, cell: CellLink) -> Option<&CellLink> {
        self.links.iter().find(|v| *v.borrow() == *cell.borrow())
    }

    pub fn neighbors(&self) -> Vec<CellLink> {
        let mut result: Vec<CellLink> = Vec::new();
        if let Some(ref cell) = self.north {
            result.push(cell.clone());
        }
        if let Some(ref cell) = self.south {
            result.push(cell.clone());
        }
        if let Some(ref cell) = self.east {
            result.push(cell.clone());
        }
        if let Some(ref cell) = self.west {
            result.push(cell.clone());
        }
        result
    }
}

pub fn link(cell1: CellLink, cell2: CellLink) {
    cell1.borrow_mut().links.push(cell2.clone());
    cell2.borrow_mut().links.push(cell1.clone());
}

pub fn unlink(cell1: CellLink, cell2: CellLink) {
    let mut cell1_borrowed = cell1.borrow_mut();
    match cell1_borrowed
        .links
        .iter()
        .position(|v| *v.borrow() == *cell2.borrow())
    {
        Some(index) => {
            cell1_borrowed.links.remove(index);
        }
        None => {}
    };

    let mut cell2_borrowed = cell2.borrow_mut();
    match cell2_borrowed
        .links
        .iter()
        .position(|v| *v.borrow() == *cell1.borrow())
    {
        Some(index) => {
            cell2_borrowed.links.remove(index);
        }
        None => {}
    };
}
