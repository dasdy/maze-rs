use crate::grid::{AbstractCell, AbstractGrid};
use std::collections::HashSet;
use gtk::DrawingArea;
use cairo::Context;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use crate::gtk::WidgetExt;
use crate::generate::recursive_backtracker;
use crate::solve::solve_with_longest_path;
use crate::solve::DijkstraStep;

#[derive(Clone)]
pub struct HexagonalCell {
    pub links: HashSet<usize>,
    pub row: usize,
    pub col: usize,
}

impl AbstractCell for HexagonalCell {
    fn row(&self) -> usize {
        self.row
    }

    fn col(&self) -> usize {
        self.col
    }

    fn links(&self) -> HashSet<usize> {
        self.links.iter().cloned().collect()
    }

    fn link(&mut self, ix: usize) {
        self.links.insert(ix);
    }
}

impl HexagonalCell {
    pub fn new(row: usize, col: usize) -> HexagonalCell {
        let links = HashSet::new();
        HexagonalCell {
            links: links,
            row: row,
            col: col,
        }
    }
}

#[derive(Clone)]
pub struct HexagonalGrid {
    pub height: usize,
    pub width: usize,
    pub cells: Vec<HexagonalCell>,
}

impl AbstractGrid<HexagonalCell> for HexagonalGrid {
    fn neighbours(&self, ix: usize) -> Vec<usize> {
        let neighbors = &vec![
            self.north_ix(ix),
            self.south_ix(ix),
            self.northeast_ix(ix),
            self.northwest_ix(ix),
            self.southeast_ix(ix),
            self.southwest_ix(ix),
        ];

        let neighbors: Vec<usize> = neighbors.iter().filter_map(|x| *x).collect();
        neighbors
    }
    fn links(&self, ix: usize) -> HashSet<usize> {
        self.cells[ix].links.iter().cloned().collect()
    }
    fn len(&self) -> usize {
        self.cells.len()
    }

    fn link(&mut self, ix1: usize, ix2: usize) {
        (self.cells[ix1].links).insert(ix2);
        (self.cells[ix2].links).insert(ix1);
    }

    fn cell(&self, ix: usize) -> &HexagonalCell {
        &self.cells[ix]
    }
}

impl HexagonalGrid {
    pub fn new(rows: usize, cols: usize) -> HexagonalGrid {
        let mut gridarr = Vec::new();
        for i in 0..rows {
            for j in 0..cols {
                gridarr.push(HexagonalCell::new(i, j));
            }
        }
        HexagonalGrid {
            width: cols,
            height: rows,
            cells: gridarr,
        }
    }

    fn north_diag(&self, col: usize, row: usize) -> usize {
        if col % 2 == 0 {
            row.wrapping_sub(1)
        } else {
            row
        }
    }
    fn south_diag(&self, col: usize, row: usize) -> usize {
        if col % 2 == 0 {
            row
        } else {
            row.wrapping_add(1)
        }
    }

    fn northeast_ix(&self, ix: usize) -> Option<usize> {
        let row = self.cells[ix].row;
        let col = self.cells[ix].col;
        self._ix_opt(self.north_diag(col, row), col.wrapping_add(1))
    }

    fn southeast_ix(&self, ix: usize) -> Option<usize> {
        let row = self.cells[ix].row;
        let col = self.cells[ix].col;
        self._ix_opt(self.south_diag(col, row), col.wrapping_add(1))
    }

    fn northwest_ix(&self, ix: usize) -> Option<usize> {
        let row = self.cells[ix].row;
        let col = self.cells[ix].col;
        self._ix_opt(self.north_diag(col, row), col.wrapping_sub(1))
    }

    fn southwest_ix(&self, ix: usize) -> Option<usize> {
        let row = self.cells[ix].row;
        let col = self.cells[ix].col;
        self._ix_opt(self.south_diag(col, row), col.wrapping_sub(1))
    }

    fn south_ix(&self, ix: usize) -> Option<usize> {
        let row = self.cells[ix].row;
        let col = self.cells[ix].col;
        self._ix_opt(row.wrapping_add(1), col)
    }

    fn north_ix(&self, ix: usize) -> Option<usize> {
        let row = self.cells[ix].row;
        let col = self.cells[ix].col;
        self._ix_opt(row.wrapping_sub(1), col)
    }

    pub fn _ix(&self, row: usize, col: usize) -> usize {
        col + row * self.width
    }

    pub fn _ix_opt(&self, row: usize, col: usize) -> Option<usize> {
        if row >= self.height || col >= self.width {
            return None;
        }
        Some(self._ix(row, col))
    }
}


pub fn draw_maze(w: &DrawingArea, cr: &Context, g: &HexagonalGrid, cellsize: f64) {
    cr.save();
    let a = cellsize / 2.;
    let b = cellsize * 3f64.sqrt() / 2.;

    let canvas_width = 3. * g.width as f64 * a + a;
    let canvas_height = 2. * g.height as f64 * b + b;

    let scalex = w.get_allocated_width() as f64 / canvas_width;
    let scaley = w.get_allocated_height() as f64 / canvas_height;
    cr.scale(scalex, scaley);
    
    for ix in 0..g.len() {
        let cur_cell = g.cell(ix);
        let draw_line = |item: &Option<usize>, end: (f64, f64)| match item {
            Some(r_idx) if !cur_cell.links().contains(r_idx) => cr.line_to(end.0, end.1),
            _ => cr.move_to(end.0, end.1),
        };


        let cx = cellsize + 3. * cur_cell.col() as f64 * a;
        let cy = b + cur_cell.row() as f64 * 2. * b + (if cur_cell.col() % 2 == 0 { 0. } else { b });

        let x_fw = cx - cellsize;
        let x_nw = cx - a;
        let x_ne = cx + a;
        let x_fe = cx + cellsize;

        let y_n = cy - b;
        let y_m = cy;
        let y_s = cy + b;
        
        cr.move_to(x_fw, y_m);
        draw_line(&g.southwest_ix(ix), (x_nw, y_s));
        draw_line(&g.south_ix(ix), (x_ne, y_s));
        draw_line(&g.southeast_ix(ix), (x_fe, y_m));
        draw_line(&g.northeast_ix(ix), (x_ne, y_n));
        draw_line(&g.north_ix(ix), (x_nw, y_n));
        draw_line(&g.northwest_ix(ix), (x_fw, y_m));
        cr.stroke();
    }

    cr.restore();
}

pub fn draw_pathfind(
    w: &DrawingArea,
    cr: &Context,
    g: &HexagonalGrid,
    step_state: &DijkstraStep,
    cellsize: f64,
) {
}


pub fn draw_hex_grid(
    img: &gtk::DrawingArea,
    signal_handler: Arc<AtomicUsize>,
    on_value: usize,
) {
    let mut g = HexagonalGrid::new(15, 15);
    let mut rng = rand::thread_rng();
    recursive_backtracker(&mut g, &mut rng);

    let g_copy = g.clone();
    let cellsize = 10.;

    let step_state = solve_with_longest_path(&g);

    img.connect_draw(move |w, cr| {
        // let bool_val = signal_handler;
        if signal_handler.load(Ordering::Relaxed) == on_value {
            draw_pathfind(w, cr, &g, &step_state, cellsize);
            draw_maze(w, cr, &g_copy, cellsize);
        }
        gtk::Inhibit(false)
    });
}
