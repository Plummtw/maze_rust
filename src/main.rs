use crate::grid::Grid;

mod grid;
mod cell;

fn main() {
    println!("Hello, world!");
    let mut grid = Grid::initialize(8, 8);
    grid.configure_cells();

    println!("{:?}", grid);
    
}
