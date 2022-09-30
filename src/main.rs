use crate::grid::Grid;

mod binary_tree;
mod cell;
mod grid;
mod sidewinder;

fn main() {
    println!("Hello, world!");
    let mut grid = Grid::initialize(8, 8);
    grid.configure_cells();

    println!("{:?}", grid);

    // binary_tree::on(&mut grid);
    sidewinder::on(&mut grid);

    println!("{:?}", grid);
}
