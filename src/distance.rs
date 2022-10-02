#![allow(dead_code)]
use std::collections::{HashMap, VecDeque};

use super::cell::*;

pub struct Distance {
    pub root: CellLink,
    pub cells: HashMap<Cell, usize>,
}

impl Distance {
    pub fn new(root: CellLink) -> Self {
        let mut cells: HashMap<Cell, usize> = HashMap::new();
        cells.insert(root.borrow().clone(), 0);
        Distance { root, cells }
    }

    pub fn get(&self, cell: CellLink) -> Option<usize> {
        self.cells.get(&*cell.borrow()).map(|v| *v)
    }

    pub fn set(&mut self, cell: CellLink, distance: usize) {
        self.cells.insert(cell.borrow().clone(), distance);
    }

    pub fn clear(&mut self, cell: CellLink) {
        self.cells.remove(&*cell.borrow());
    }

    pub fn cells(&self) -> Vec<&Cell> {
        self.cells.keys().collect::<Vec<_>>()
    }

    pub fn distances(cell: CellLink) -> Self {
        let mut distances = Distance::new(cell.clone());
        let mut frontier: VecDeque<CellLink> = VecDeque::new();
        frontier.push_back(cell);

        while let Some(cell) = frontier.pop_front() {
            let cell_borrowed = cell.borrow();
            for linked in &cell_borrowed.links {
                match linked.upgrade().clone() {
                    Some(linked) => {
                        if distances.get(linked.clone()).is_none() {
                            distances.set(
                                linked.clone(),
                                distances.get(cell.clone()).map_or(0, |v| v) + 1,
                            );
                            frontier.push_back(linked);
                        }
                    }
                    None => (),
                }
            }
        }
        distances
    }

    pub fn path_to(&self, goal: CellLink) -> Self {
        let mut current = goal;

        let mut breadcomb = Distance::new(self.root.clone());
        let mut distance = match self.get(current.clone()) {
            Some(distance) => {
                breadcomb.set(current.clone(), distance);
                distance
            }
            None => 0,
        };

        loop {
            let loop_break = { *current.borrow() == *self.root.borrow() };
            if loop_break {
                break;
            }
            let links = { current.borrow().clone().links };
            for neighbor in links {
                let neighbor = match neighbor.upgrade().clone() {
                    Some(neighbor) => neighbor,
                    None => continue,
                };
                let distance_neighbor = match self.get(neighbor.clone()) {
                    Some(distance) => distance,
                    None => 0,
                };
                if distance_neighbor < distance {
                    breadcomb.set(neighbor.clone(), distance_neighbor);
                    distance = distance_neighbor;
                    current = neighbor;
                    break;
                }
            }
        }

        breadcomb
    }
}

pub fn distance_str(input: Option<usize>) -> String {
    match input {
        Some(input) => {
            if input < 10 {
                input.to_string()
            } else if input < 36 {
                (((input - 10) + ('a' as usize)) as u8 as char).to_string()
            } else {
                (((input - 36) + ('A' as usize)) as u8 as char).to_string()
            }
        }
        None => String::from(" "),
    }
}
