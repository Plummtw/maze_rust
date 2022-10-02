#![allow(dead_code)]
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

use super::cell::*;
use super::grid::*;

pub type KruskalLink = Rc<RefCell<KruskalNode>>;
pub type WeakKruskalLink = Weak<RefCell<KruskalNode>>;

impl Downgradable<WeakKruskalLink> for KruskalLink {
    fn downgrade(self) -> WeakKruskalLink {
        Rc::downgrade(&self)
    }
}

impl Downgradable<Option<WeakKruskalLink>> for Option<KruskalLink> {
    fn downgrade(self) -> Option<WeakKruskalLink> {
        self.map(|v| Rc::downgrade(&v))
    }
}

impl Upgradable<Option<KruskalLink>> for Option<WeakKruskalLink> {
    fn upgrade(self) -> Option<KruskalLink> {
        match self.map(|v| v.upgrade()) {
            None => None,
            Some(v) => v,
        }
    }
}

#[derive(Debug, Clone)]
pub struct KruskalNode {
    pub row: usize,
    pub column: usize,
    pub parent: Option<WeakKruskalLink>,
}

impl KruskalNode {
    pub fn root(&self) -> (usize, usize) {
        match &self.parent {
            Some(parent) => {
                let mut current = parent.upgrade();
                loop {
                    match current {
                        Some(current_node) => {
                            let current_borrow = current_node.borrow();
                            if current_borrow.parent.is_none() {
                                return (current_borrow.row, current_borrow.column);
                            }
                            current = (&current_node.borrow().parent).clone().upgrade();
                        }
                        None => panic!("root error"),
                    }
                }
            }
            None => (self.row, self.column),
        }
    }
}

impl PartialEq for KruskalNode {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.column == other.column
    }
}

#[derive(Debug, Copy, Clone)]
pub struct KruskalEdge {
    pub row: usize,
    pub column: usize,
    pub horizontal: bool,
}

impl KruskalEdge {
    pub fn find_node(&self, links: &Vec<KruskalLink>) -> Option<(KruskalLink, KruskalLink)> {
        let node1 = links.iter().find(|v| {
            let cell = v.borrow().clone();
            cell.row == self.row && cell.column == self.column
        });

        let node1 = match node1 {
            Some(node1) => node1.clone(),
            None => return None,
        };

        let (row1, column1) = if self.horizontal {
            (self.row + 1, self.column)
        } else {
            (self.row, self.column + 1)
        };

        let node2 = links.iter().find(|v| {
            let cell = v.borrow().clone();
            cell.row == row1 && cell.column == column1
        });

        let node2 = match node2 {
            Some(node2) => node2.clone(),
            None => return None,
        };

        Some((node1, node2))
    }
}

pub struct Kruskal {
    pub nodes: Vec<KruskalLink>,
    pub edges: Vec<KruskalEdge>,
}

impl Kruskal {
    pub fn new(grid: &Grid) -> Self {
        let mut nodes: Vec<KruskalLink> = Vec::with_capacity(grid.rows * grid.columns);
        let mut edges: Vec<KruskalEdge> =
            Vec::with_capacity((grid.rows - 1) * (grid.columns - 1) * 2);

        for row in 0..grid.rows {
            for column in 0..grid.columns {
                if row != grid.rows - 1 {
                    let edge_h = KruskalEdge {
                        row,
                        column,
                        horizontal: true,
                    };
                    edges.push(edge_h);
                }
                if column != grid.columns - 1 {
                    let edge_v = KruskalEdge {
                        row,
                        column,
                        horizontal: false,
                    };
                    edges.push(edge_v);
                }

                let node = KruskalNode {
                    row,
                    column,
                    parent: None,
                };
                let node = Rc::new(RefCell::new(node));
                nodes.push(node);
            }
        }
        edges.shuffle(&mut thread_rng());
        Kruskal { nodes, edges }
    }
}

pub fn on(grid: &mut Grid) {
    // Create Kruskal Structure
    let mut kruskal = Kruskal::new(grid);

    loop {
        let current = match kruskal.edges.pop() {
            Some(v) => v,
            None => break,
        };

        let find_result = current.find_node(&kruskal.nodes);
        match find_result {
            Some((node1, node2)) => {
                {
                    let node1_borrowed = node1.borrow();
                    let node2_borrowed = node2.borrow();
                    if node1_borrowed.root() == node2_borrowed.root() {
                        continue;
                    }
                    link(
                        grid.get_cell(node1_borrowed.row, node1_borrowed.column)
                            .unwrap(),
                        grid.get_cell(node2_borrowed.row, node2_borrowed.column)
                            .unwrap(),
                    );
                }
                {
                    let node1_root = { node1.borrow().root() };
                    let node1_root_borrow = kruskal
                        .nodes
                        .iter()
                        .find(|v| {
                            v.borrow().row == node1_root.0 && v.borrow().column == node1_root.1
                        })
                        .unwrap();
                    node1_root_borrow.borrow_mut().parent = Some(node2.clone().downgrade());
                }
            }
            None => (),
        }
    }
}
