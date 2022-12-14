#![allow(dead_code)]
use super::cell::*;
use super::grid::*;

use rand::Rng;

pub fn on(grid: &mut Grid) {
    let mut rng = rand::thread_rng();
    for row in &grid.grid {
        for cell in row {
            // let cell = cell.clone();
            let mut neighbors: Vec<CellLink> = Vec::new();
            {
                let cell_borrowed = cell.borrow();

                if let Some(north) = cell_borrowed.north() {
                    neighbors.push(north);
                }
                if let Some(east) = cell_borrowed.east() {
                    neighbors.push(east);
                }
            }

            if !neighbors.is_empty() {
                let index = rng.gen_range(0..neighbors.len());

                link(cell.clone(), neighbors[index].clone());
            }
        }
    }
}
