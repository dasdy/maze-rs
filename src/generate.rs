use rand::prelude::*;
use crate::grid::{Grid};
use std::collections::HashSet;

#[allow(dead_code)]
pub fn binary_tree(g: &mut Grid, r: &mut rand::rngs::ThreadRng) {
    for i in 0..g.cells.len() {
        let c = &g.cells[i];
        let neighbor = match (g.north_ix(c.row, c.col), g.east_ix(c.row, c.col)) {
            (Some(c1), Some(c2)) => {
                let vec = [c1, c2];
                Some(vec[r.gen_range(0, 2)])
            }
            (Some(c1), None) => Some(c1),
            (None, Some(c1)) => Some(c1),
            _ => None
        };
        match neighbor {
            Some(c1) => g.link(i, c1),
            _ => {}
        }
    }
}

#[allow(dead_code)]
pub fn sidewinder(g: &mut Grid, r: &mut rand::rngs::ThreadRng) {
    for i in 0..g.height {
        let mut current_run = Vec::new();
        for j in 0..g.width {
            current_run.push((i, j));

            let should_close_out = (j == g.width - 1) || (i > 1 && r.gen_bool(0.5));

            if should_close_out {
                let (r_i, r_j) = current_run[
                    r.gen_range(0, current_run.len())];
                let oix1 = g.north_ix(r_i, r_j);
                let ix2 = g._ix(r_i, r_j);
                current_run.clear();
                match oix1 {
                    Some(ix1) => g.link(ix1, ix2),
                    _ => {}
                }
            } else {
                let ix1 = g._ix(i, j);
                let ix2 = g.east_ix(i, j).unwrap();
                g.link(ix1, ix2);
            }
        }
    }
}

pub fn aldous_broder(g: &mut Grid, r: &mut rand::rngs::ThreadRng) {
    let mut visited = HashSet::new();
    let target_size = g.cells.len();
    let mut current_cell = r.gen_range(0, g.cells.len());
    visited.insert(current_cell);

    while visited.len() < target_size {
        let row = g.cells[current_cell].row;
        let col = g.cells[current_cell].col;
        let neighbors = vec![g.north_ix(row, col), g.east_ix(row,col),
        g.west_ix(row, col), g.south_ix(row, col)];
        let results: Vec<usize>  = neighbors.iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();

        let random_neighbor = results[r.gen_range(0, results.len())];
        if !visited.contains(&random_neighbor) {
            g.link(random_neighbor, current_cell);
        }
        visited.insert(random_neighbor);
        current_cell = random_neighbor;

    }

}