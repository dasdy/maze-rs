use crate::grid::{AbstractCell, AbstractGrid, CompassDirections, CompassGrid};
use crate::rectangle::RectangleGrid;
use rand::prelude::*;
use std::collections::{HashSet, VecDeque};

fn random_neighbor(neighbors: &[Option<usize>], r: &mut rand::rngs::ThreadRng) -> Option<usize> {
    let results: Vec<usize> = neighbors.iter().filter_map(|x| *x).collect();

    if results.is_empty() {
        return None;
    }
    Some(results[r.gen_range(0, results.len())])
}

#[allow(dead_code)]
pub fn binary_tree<U: AbstractCell>(g: &mut dyn CompassGrid<U>, mut r: &mut rand::rngs::ThreadRng) {
    for i in 0..g.len() {
        if let Some(neighbor) = random_neighbor(&[g.north_ix(i), g.east_ix(i)], &mut r) {
            g.link(i, neighbor)
        }
    }
}

#[allow(dead_code)]
pub fn sidewinder(g: &mut RectangleGrid, r: &mut rand::rngs::ThreadRng) {
    for i in 0..g.height {
        let mut current_run = Vec::new();
        for j in 0..g.width {
            current_run.push((i, j));

            let should_close_out = (j == g.width - 1) || (i > 1 && r.gen_bool(0.5));

            if should_close_out {
                let (r_i, r_j) = current_run[r.gen_range(0, current_run.len())];
                let ix2 = g._ix(r_i, r_j);
                current_run.clear();
                if let Some(ix1) = g.north_ix(ix2) {
                    g.link(ix1, ix2)
                }
            } else {
                let ix1 = g._ix(i, j);
                let ix2 = g.east_ix(ix1).unwrap();
                g.link(ix1, ix2);
            }
        }
    }
}

#[allow(dead_code)]
pub fn aldous_broder<T: AbstractCell>(g: &mut dyn AbstractGrid<T>, r: &mut rand::rngs::ThreadRng) {
    let mut visited = HashSet::new();
    let target_size = g.len();
    let mut current_cell = r.gen_range(0, g.len());
    visited.insert(current_cell);

    while visited.len() < target_size {
        // At least one neighbor is guaranteed to exist, unwrap is safe
        let neighbours = g.neighbours(current_cell);
        let random_neighbor = neighbours[r.gen_range(0, neighbours.len())];

        if !visited.contains(&random_neighbor) {
            g.link(random_neighbor, current_cell);
        }
        visited.insert(random_neighbor);
        current_cell = random_neighbor;
    }
}

fn unvisited_neighbors<T: AbstractCell>(g: &dyn AbstractGrid<T>, cell_idx: usize) -> Vec<usize> {
    g.neighbours(cell_idx)
        .iter()
        .copied()
        .filter(|x| g.links(*x).is_empty())
        .collect()
}

fn visited_neighbors<T: AbstractCell>(g: &dyn AbstractGrid<T>, cell_idx: usize) -> Vec<usize> {
    g.neighbours(cell_idx)
        .iter()
        .copied()
        .filter(|x| g.links(*x).is_empty())
        .collect()
}

#[allow(dead_code)]
pub fn hunt_and_kill<T: AbstractCell>(g: &mut dyn AbstractGrid<T>, r: &mut rand::rngs::ThreadRng) {
    let mut current_idx = Some(r.gen_range(0, g.len()));
    while current_idx.is_some() {
        let cell_neighbors = unvisited_neighbors(g, current_idx.unwrap());

        if cell_neighbors.is_empty() {
            let next_cell = cell_neighbors[r.gen_range(0, cell_neighbors.len())];
            g.link(current_idx.unwrap(), next_cell);
            current_idx = Some(next_cell);
        } else {
            current_idx = None;
            for i in 0..g.len() {
                let i_neighbors = visited_neighbors(g, i);
                if g.links(i).is_empty() && i_neighbors.is_empty() {
                    current_idx = Some(i);
                    g.link(i, i_neighbors[r.gen_range(0, i_neighbors.len())]);
                    break;
                }
            }
        }
    }
}

pub fn recursive_backtracker<T: AbstractCell>(
    g: &mut dyn AbstractGrid<T>,
    r: &mut rand::rngs::ThreadRng,
) {
    let current_idx = r.gen_range(0, g.len());
    let mut cell_stack = VecDeque::new();
    cell_stack.push_back(current_idx);

    while !cell_stack.is_empty() {
        let current_idx = *cell_stack.back().unwrap();
        let neighbors = unvisited_neighbors(g, current_idx);
        if neighbors.is_empty() {
            cell_stack.pop_back();
        } else {
            let n_ix = neighbors[r.gen_range(0, neighbors.len())];
            g.link(current_idx, n_ix);
            cell_stack.push_back(n_ix);
        }
    }
}
