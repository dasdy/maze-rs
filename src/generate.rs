use rand::prelude::*;
use crate::grid::{AbstractGrid, Grid};
use std::collections::{HashSet, VecDeque};

fn random_neighbor(neighbors: &Vec<Option<usize>>, r: &mut rand::rngs::ThreadRng) -> Option<usize> {
    let results: Vec<usize>  = neighbors.iter().filter_map(|x| *x).collect();

    if results.len() == 0 {
        return None
    }
    Some(results[r.gen_range(0, results.len())])
}

#[allow(dead_code)]
pub fn binary_tree(g: &mut Grid, mut r: &mut rand::rngs::ThreadRng) {
    for i in 0..g.cells.len() {
        let c = &g.cells[i];
        if let Some(neighbor) = random_neighbor(
            &vec![g.north_ix(c.row, c.col), g.east_ix(c.row, c.col)], &mut r) {
            g.link(i, neighbor)
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
                if let Some(ix1) = oix1 {
                    g.link(ix1, ix2)
                }
            } else {
                let ix1 = g._ix(i, j);
                let ix2 = g.east_ix(i, j).unwrap();
                g.link(ix1, ix2);
            }
        }
    }
}

#[allow(dead_code)]
pub fn aldous_broder(g: &mut Grid, mut r: &mut rand::rngs::ThreadRng) {
    let mut visited = HashSet::new();
    let target_size = g.cells.len();
    let mut current_cell = r.gen_range(0, g.cells.len());
    visited.insert(current_cell);

    while visited.len() < target_size {
        let row = g.cells[current_cell].row;
        let col = g.cells[current_cell].col;
        // At least one neighbor is guaranteed to exist, unwrap is safe
        let random_neighbor = random_neighbor(
            &vec![g.north_ix(row, col), g.east_ix(row,col),
                  g.west_ix(row, col), g.south_ix(row, col)],
            &mut r).unwrap();


        if !visited.contains(&random_neighbor) {
            g.link(random_neighbor, current_cell);
        }
        visited.insert(random_neighbor);
        current_cell = random_neighbor;

    }

}

fn hunt_kill_unvisited_cell_neighbors(g: &AbstractGrid, cell_idx: usize) -> Vec<usize> {
    g.neighbours(cell_idx).iter().map(|x| *x)
        .filter(|x| g.links(*x).len() == 0).collect()
}

fn hunt_kill_visited_cell_neighbors(g: &AbstractGrid, cell_idx: usize) -> Vec<usize> {
    g.neighbours(cell_idx).iter().map(|x| *x)
        .filter(|x| g.links(*x).len() != 0).collect()
}

#[allow(dead_code)]
pub fn hunt_and_kill(g: &mut Grid, r: &mut rand::rngs::ThreadRng) {
    let mut current_idx = Some(r.gen_range(0, g.cells.len()));
    while current_idx.is_some() {
        let cell_neighbors =
            hunt_kill_unvisited_cell_neighbors(g, current_idx.unwrap());

        if cell_neighbors.len() != 0 {
            let next_cell = cell_neighbors[r.gen_range(0, cell_neighbors.len())];
            g.link(current_idx.unwrap(), next_cell);
            current_idx = Some(next_cell);
        } else {
            current_idx = None;
            for i in 0..g.cells.len() {
                let i_neighbors = hunt_kill_visited_cell_neighbors(g, i);
                if g.cells[i].links.len() == 0 && i_neighbors.len() != 0 {
                    current_idx = Some(i);
                    g.link(i, i_neighbors[r.gen_range(0, i_neighbors.len())]);
                    break;
                }
            }
        }
    }
}

#[allow(dead_code)]
pub fn recursive_backtracker(g: &mut AbstractGrid, r: &mut rand::rngs::ThreadRng) {
    let current_idx = r.gen_range(0, g.len());
    let mut cell_stack = VecDeque::new();
    cell_stack.push_back(current_idx);

    while !cell_stack.is_empty() {
        let current_idx = *cell_stack.back().unwrap();
        let neighbors = hunt_kill_unvisited_cell_neighbors(g, current_idx);
        if neighbors.is_empty() {
            cell_stack.pop_back();
        } else {
            let n_ix = neighbors[r.gen_range(0, neighbors.len())];
            g.link(current_idx, n_ix);
            cell_stack.push_back(n_ix);
        }
    }
}