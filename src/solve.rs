use crate::grid::Grid;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter, Error};


#[derive(Clone, Debug)]
pub struct PathBacktrackItem {
    pub path_length: i32,
    pub parent: i32,
}

#[derive(Clone)]
pub struct DijkstraStep {
    pub cell_weights: Vec<PathBacktrackItem>,
    pub lookup_queue: VecDeque<usize>
}

impl Display for DijkstraStep {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "DijkstraStep: \nlookup: ")?;
        for i in &self.lookup_queue {
            write!(f, "{}, ", i)?;
        }
        writeln!(f, "\nAssigned vals: \n")?;
        for i in &self.cell_weights {
            if i.parent >= 0 {
                writeln!(f, "{:?}", i)?;
            }
        }
        writeln!(f, "")
    }
}

impl DijkstraStep {
    pub fn initial(g: &Grid, start: usize) -> DijkstraStep {
        let mut cell_weights: Vec<PathBacktrackItem> = Vec::new();
        for _ in 0..g.cells.len() {
            cell_weights.push(PathBacktrackItem {path_length : -1, parent: -1 });
        }
        cell_weights[start].path_length = 0;
        cell_weights[start].parent = start as i32;
        let c = &g.cells[start];

        let mut lookup_queue = VecDeque::new();
        for (row, col) in &c.links {
            let ix = g._ix(*row, *col);
            lookup_queue.push_back(ix);
            cell_weights[ix].path_length = 1;
            cell_weights[ix].parent = start as i32;
        }

        DijkstraStep {cell_weights, lookup_queue}
    }

    pub fn next_step(&self, g: &Grid) -> DijkstraStep {
        let mut lookup_queue = self.lookup_queue.clone();
        let mut cell_weights = self.cell_weights.clone();
        let cur_cell = lookup_queue.pop_front().unwrap();
        let cur_weight = cell_weights[cur_cell].path_length;

        let c = &g.cells[cur_cell];
        for (row, col) in &c.links {
            let ix = g._ix(*row, *col);
            if cell_weights[ix].parent < 0 {
                lookup_queue.push_back(ix);
                cell_weights[ix].path_length = cur_weight + 1;
                cell_weights[ix].parent = cur_cell as i32;
            }
        }

        DijkstraStep {lookup_queue, cell_weights}
    }
}

