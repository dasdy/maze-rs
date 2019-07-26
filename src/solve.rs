use crate::grid::{AbstractGrid, AbstractCell};
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
    #[allow(dead_code)]
    pub fn initial<T: AbstractCell>(g: &AbstractGrid<T>, start: usize) -> DijkstraStep {
        let mut cell_weights: Vec<PathBacktrackItem> = Vec::new();
        for _ in 0..g.len() {
            cell_weights.push(PathBacktrackItem {path_length : -1, parent: -1 });
        }
        cell_weights[start].path_length = 0;
        cell_weights[start].parent = start as i32;

        let mut lookup_queue = VecDeque::new();
        for &ix in &g.links(start) {
            lookup_queue.push_back(ix);
            cell_weights[ix].path_length = 1;
            cell_weights[ix].parent = start as i32;
        }

        DijkstraStep {cell_weights, lookup_queue}
    }

    #[allow(dead_code)]
    pub fn next_step<T: AbstractCell>(&self, g: &AbstractGrid<T>) -> DijkstraStep {
        let mut lookup_queue = self.lookup_queue.clone();
        let mut cell_weights = self.cell_weights.clone();
        let cur_cell = lookup_queue.pop_front().unwrap();
        let cur_weight = cell_weights[cur_cell].path_length;

        for &ix in &g.links(cur_cell) {
            if cell_weights[ix].parent < 0 {
                lookup_queue.push_back(ix);
                cell_weights[ix].path_length = cur_weight + 1;
                cell_weights[ix].parent = cur_cell as i32;
            }
        }

        DijkstraStep {lookup_queue, cell_weights}
    }
}

pub fn solve_with_longest_path<T: AbstractCell>(g: &AbstractGrid<T>) -> DijkstraStep {
    let start = 0;
    // solve initially from random point
    let mut result = DijkstraStep::initial(g, start);
    while !result.lookup_queue.is_empty() {
        result = result.next_step(g);
    }

    let mut max_length = 0;
    let mut max_idx = 0;
    for (i, c) in result.cell_weights.iter().enumerate() {
        if c.path_length > max_length {
            max_length = c.path_length;
            max_idx = i;
        }
    }

    if max_idx != 0 {
        result = DijkstraStep::initial(g, start);
        while !result.lookup_queue.is_empty() {
            result = result.next_step(g);
        }
    }
    result
}