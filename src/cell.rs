#![allow(dead_code)]
use std::{
    cell::RefCell,
    rc::{Rc, Weak}, hash::Hash,
};

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

impl Hash for Cell {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.column.hash(state);
    }
}

impl Eq for Cell {
    fn assert_receiver_is_total_eq(&self) {
    }
}

impl Cell {
    pub fn new(row: usize, column: usize) -> Self {
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

    pub fn north(&self) -> Option<CellLink> {
        self.north.clone().upgrade()
    }

    pub fn south(&self) -> Option<CellLink> {
        self.south.clone().upgrade()
    }

    pub fn east(&self) -> Option<CellLink> {
        self.east.clone().upgrade()
    }

    pub fn west(&self) -> Option<CellLink> {
        self.west.clone().upgrade()
    }

    pub fn linked(&self, cell: CellLink) -> Option<CellLink> {
        let result = self.links.iter().find(|v| match v.upgrade() {
            Some(v) => *v.borrow() == *cell.borrow(),
            None => false,
        });
        match result {
            Some(x) => x.upgrade(),
            None => None,
        }
    }

    pub fn neighbors(&self) -> Vec<CellLink> {
        let mut result: Vec<CellLink> = Vec::new();
        if let Some(cell) = self.north() {
            result.push(cell);
        }
        if let Some(cell) = self.south() {
            result.push(cell);
        }
        if let Some(cell) = self.east() {
            result.push(cell);
        }
        if let Some(cell) = self.west() {
            result.push(cell);
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
    match cell1_borrowed.links.iter().position(|v| match v.upgrade() {
        Some(v) => *v.borrow() == *cell2.borrow(),
        None => false,
    }) {
        Some(index) => {
            cell1_borrowed.links.remove(index);
        }
        None => {}
    };

    let mut cell2_borrowed = cell2.borrow_mut();
    match cell2_borrowed.links.iter().position(|v| match v.upgrade() {
        Some(v) => *v.borrow() == *cell1.borrow(),
        None => false,
    }) {
        Some(index) => {
            cell2_borrowed.links.remove(index);
        }
        None => {}
    };
}
