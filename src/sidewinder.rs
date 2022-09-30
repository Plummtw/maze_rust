#![allow(dead_code)]
use super::cell::*;
use super::grid::*;

use rand::Rng;

pub fn on(grid: &mut Grid) {
    let mut rng = rand::thread_rng();
    for row in &grid.grid {
        let mut run: Vec<CellLink> = Vec::new();
        for cell in row {
            run.push(cell.clone());
            let should_close_out = {
                let cell_borrowed = cell.borrow();
                cell_borrowed.east.is_none()
                    || (cell_borrowed.north.is_some() && rng.gen_range(0..2) == 0u8)
            };

            if should_close_out {
                let member = run[rng.gen_range(0..run.len())].clone();
                if member.borrow().north.is_some() {
                    let cell2 = { member.borrow().north.clone().unwrap() };
                    link(member.clone(), cell2);
                    run.clear();
                }
            } else {
                let cell2: CellLink = { cell.borrow().east.clone().unwrap() };
                link(cell.clone(), cell2);
            }
        }
    }
}
