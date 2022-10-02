use crate::{grid::Grid, distance::Distance};

mod binary_tree;
mod cell;
mod grid;
mod sidewinder;
mod distance;
mod recursive_backtracker;

fn main() {
    println!("Hello, world!");
    let mut grid = Grid::new(10, 10);
    grid.configure_cells();

    println!("{:?}", grid);

    // binary_tree::on(&mut grid);
    // println!("{:?}", grid);

    // let mut grid = Grid::new(8, 8);
    // grid.configure_cells();

    // sidewinder::on(&mut grid);
    recursive_backtracker::on(&mut grid);
    println!("{:?}", grid);

    let distance = Distance::distances(grid.get_cell(0, 0).unwrap());
    grid.set_distance(distance);
    // let breadcomb = distance.path_to(grid.get_cell(grid.rows-1, grid.columns-1).unwrap());
    // grid.set_distance(breadcomb);
    println!("{:?}", grid);
}
