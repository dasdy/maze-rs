use rand::prelude::*;
use crate::grid::{Grid};

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