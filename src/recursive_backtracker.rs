#![allow(dead_code)]
use super::cell::*;
use super::grid::*;

use rand::Rng;

pub fn on(grid: &mut Grid) {
    let mut rng = rand::thread_rng();
    let mut stack: Vec<CellLink> = Vec::with_capacity(grid.size());
    stack.push(grid.get_cell(0, 0).unwrap());

    while let Some(current) = stack.last() {
    // loop {
    //     let current = match stack.last() {
    //       Some(v) => v.clone(),
    //       None => break,
    //     };
      
        let neighbors = {
          let cell_borrowed = current.borrow();
          let neighbors = cell_borrowed
            .neighbors().clone();
          neighbors
            .into_iter()
            .filter(|v| (*v.clone().borrow()).links.is_empty())
            .collect::<Vec<_>>()
        };
        
        if neighbors.is_empty() {
          stack.pop();
        } else {
          let neighbor = neighbors[rng.gen_range(0..neighbors.len())].clone();
          link(current.clone(), neighbor.clone());
          stack.push(neighbor.clone());
        }
    }

}
