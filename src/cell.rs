#![allow(dead_code)]
use std::{cell::RefCell, rc::{Rc, Weak}};

pub type CellLink = Rc<RefCell<Cell>>;
pub type WeakCellLink = Weak<RefCell<Cell>>;

pub trait Downgradable<T> {
    fn downgrade(self) -> T;
}

impl Downgradable<WeakCellLink> for CellLink {
    fn downgrade(self) -> WeakCellLink {
        Rc::downgrade(&self)
    }
}

impl Downgradable<Option<WeakCellLink>> for Option<CellLink> {
    fn downgrade(self) -> Option<WeakCellLink> {
        self.map(|v| Rc::downgrade(&v))
    }
}

pub trait Upgradable<T> {
    fn upgrade(self) -> T;
}

impl Upgradable<Option<CellLink>> for Option<WeakCellLink> {
    fn upgrade(self) -> Option<CellLink> {
        match self.map(|v| v.upgrade()) {
            None => None,
            Some(v) => v,
        }
    }
}


#[derive(Debug, Clone)]
pub struct Cell {
    pub row: usize,
    pub column: usize,
    pub north: Option<WeakCellLink>,
    pub south: Option<WeakCellLink>,
    pub east: Option<WeakCellLink>,
    pub west: Option<WeakCellLink>,
    pub links: Vec<WeakCellLink>,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.column == other.column
    }
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

    pub fn linked(&self, cell: CellLink) -> Option<CellLink> {
        let result = self.links.iter().find(|v|
            *v.upgrade().unwrap().borrow() == *cell.borrow()
        );
        match result {
            Some(x)  => x.upgrade(),
            None => None,
        }
    }

    pub fn neighbors(&self) -> Vec<CellLink> {
        let mut result: Vec<CellLink> = Vec::new();
        if let Some(ref cell) = self.north {
            result.push(cell.upgrade().unwrap());
        }
        if let Some(ref cell) = self.south {
            result.push(cell.upgrade().unwrap());
        }
        if let Some(ref cell) = self.east {
            result.push(cell.upgrade().unwrap());
        }
        if let Some(ref cell) = self.west {
            result.push(cell.upgrade().unwrap());
        }
        result
    }
}

pub fn link(cell1: CellLink, cell2: CellLink) {
    cell1.borrow_mut().links.push(cell2.clone().downgrade());
    cell2.borrow_mut().links.push(cell1.downgrade());
}

pub fn unlink(cell1: CellLink, cell2: CellLink) {
    let mut cell1_borrowed = cell1.borrow_mut();
    match cell1_borrowed
        .links
        .iter()
        .position(|v| *v.upgrade().unwrap().borrow() == *cell2.borrow())
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
        .position(|v| *v.upgrade().unwrap().borrow() == *cell1.borrow())
    {
        Some(index) => {
            cell2_borrowed.links.remove(index);
        }
        None => {}
    };
}
