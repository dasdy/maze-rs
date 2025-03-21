use std::collections::HashSet;

use crate::draw_utils::GtkDrawable;
use crate::grid::{AbstractCell, AbstractGrid, RectangularGrid};
use crate::gtk::prelude::WidgetExt;
use crate::rectangle::Cell;
use crate::solve::DijkstraStep;
use gtk::cairo::Context;
use gtk::DrawingArea;

#[derive(Clone)]
pub struct HexagonalGrid {
    pub height: usize,
    pub width: usize,
    pub cells: Vec<Cell>,
}

impl AbstractGrid<Cell> for HexagonalGrid {
    fn neighbours(&self, ix: usize) -> Vec<usize> {
        let neighbors = [
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

    fn len(&self) -> usize {
        self.cells.len()
    }

    fn cell(&self, ix: usize) -> &Cell {
        &self.cells[ix]
    }

    fn cell_mut(&mut self, ix: usize) -> &mut Cell {
        &mut self.cells[ix]
    }
}

impl HexagonalGrid {
    pub fn new(rows: usize, cols: usize) -> HexagonalGrid {
        let mut gridarr = Vec::new();
        for i in 0..rows {
            for j in 0..cols {
                gridarr.push(Cell::new(i, j));
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
        self.ix_opt(self.north_diag(col, row), col.wrapping_add(1))
    }

    fn southeast_ix(&self, ix: usize) -> Option<usize> {
        let row = self.cells[ix].row;
        let col = self.cells[ix].col;
        self.ix_opt(self.south_diag(col, row), col.wrapping_add(1))
    }

    fn northwest_ix(&self, ix: usize) -> Option<usize> {
        let row = self.cells[ix].row;
        let col = self.cells[ix].col;
        self.ix_opt(self.north_diag(col, row), col.wrapping_sub(1))
    }

    fn southwest_ix(&self, ix: usize) -> Option<usize> {
        let row = self.cells[ix].row;
        let col = self.cells[ix].col;
        self.ix_opt(self.south_diag(col, row), col.wrapping_sub(1))
    }

    fn south_ix(&self, ix: usize) -> Option<usize> {
        let row = self.cells[ix].row;
        let col = self.cells[ix].col;
        self.ix_opt(row.wrapping_add(1), col)
    }

    fn north_ix(&self, ix: usize) -> Option<usize> {
        let row = self.cells[ix].row;
        let col = self.cells[ix].col;
        self.ix_opt(row.wrapping_sub(1), col)
    }
}

impl RectangularGrid for HexagonalGrid {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
}

fn center_coords(row: usize, col: usize, cellsize: f64) -> (f64, f64) {
    let a = cellsize / 2.;
    let b = cellsize * 3f64.sqrt() / 2.;

    let cx = cellsize + 3. * col as f64 * a;
    let cy = b + row as f64 * 2. * b + (if col % 2 == 0 { 0. } else { b });

    (cx, cy)
}

struct HexagonalCoords {
    pub x_fw: f64,
    pub x_nw: f64,
    pub x_ne: f64,
    pub x_fe: f64,

    pub y_s: f64,
    pub y_n: f64,
    pub y_m: f64,

    pub cx: f64,
    pub cy: f64,
}

fn hex_points(row: usize, col: usize, cellsize: f64) -> HexagonalCoords {
    let (cx, cy) = center_coords(row, col, cellsize);
    let a = cellsize / 2.;
    let b = cellsize * 3f64.sqrt() / 2.;

    let x_fw = cx - cellsize;
    let x_nw = cx - a;
    let x_ne = cx + a;
    let x_fe = cx + cellsize;

    let y_n = cy - b;
    let y_m = cy;
    let y_s = cy + b;

    HexagonalCoords {
        x_fw,
        x_nw,
        x_ne,
        x_fe,
        y_s,
        y_n,
        y_m,
        cx,
        cy,
    }
}

impl GtkDrawable for HexagonalGrid {
    fn draw_maze(&self, w: &DrawingArea, cr: &Context, cellsize: f64) {
        cr.save().expect("error while saving coords");
        let a = cellsize / 2.;
        let b = cellsize * 3f64.sqrt() / 2.;

        let canvas_width = 3. * self.width as f64 * a + a;
        let canvas_height = 2. * self.height as f64 * b + b + 0.1 * cellsize;

        let scalex = w.allocated_width() as f64 / canvas_width;
        let scaley = w.allocated_height() as f64 / canvas_height;
        cr.scale(scalex, scaley);

        for ix in 0..self.len() {
            let cur_cell = self.cell(ix);
            let draw_line = |item: &Option<usize>, end: (f64, f64)| match item {
                Some(r_idx) if (!cur_cell.links().contains(r_idx)) => cr.line_to(end.0, end.1),
                None => cr.line_to(end.0, end.1),
                _ => cr.move_to(end.0, end.1),
            };

            let coords = hex_points(cur_cell.row(), cur_cell.col(), cellsize);

            cr.move_to(coords.x_fw, coords.y_m);
            draw_line(&self.southwest_ix(ix), (coords.x_nw, coords.y_s));
            draw_line(&self.south_ix(ix), (coords.x_ne, coords.y_s));
            draw_line(&self.southeast_ix(ix), (coords.x_fe, coords.y_m));
            draw_line(&self.northeast_ix(ix), (coords.x_ne, coords.y_n));
            draw_line(&self.north_ix(ix), (coords.x_nw, coords.y_n));
            draw_line(&self.northwest_ix(ix), (coords.x_fw, coords.y_m));
            cr.stroke().expect("error while drawing stroke");
        }

        cr.restore().expect("error while restoring coords");
    }

    fn draw_pathfind(
        &self,
        w: &DrawingArea,
        cr: &Context,
        step_state: &DijkstraStep,
        cellsize: f64,
    ) {
        cr.save().expect("error while saving coords");
        let a = cellsize / 2.;
        let b = cellsize * 3f64.sqrt() / 2.;

        let canvas_width = 3. * self.width as f64 * a + a;
        let canvas_height = 2. * self.height as f64 * b + b + 0.1 * cellsize;

        let scalex = w.allocated_width() as f64 / canvas_width;
        let scaley = w.allocated_height() as f64 / canvas_height;
        cr.scale(scalex, scaley);

        let mut max_idx = 0;
        let mut min_idx = 0;
        let mut max_length = step_state.cell_weights[max_idx].path_length;
        let mut min_length = max_length;
        for (i, c) in step_state.cell_weights.iter().enumerate() {
            if c.path_length > max_length {
                max_length = c.path_length;
                max_idx = i;
            }
            if c.path_length < min_length {
                min_length = c.path_length;
                min_idx = i;
            }
        }

        let coords = |ix: usize| {
            let row = self.cell(ix).row();
            let col = self.cell(ix).col();
            hex_points(row, col, cellsize)
        };

        for (i, c) in step_state.cell_weights.iter().enumerate() {
            let intensity = (max_length - c.path_length) as f64 / max_length as f64;
            let dark = intensity;
            let bright = 0.5 + intensity / 2.;
            cr.set_source_rgb(dark, bright, dark);
            cr.set_line_width(0.1);
            let coords = coords(i);

            cr.move_to(coords.x_fw, coords.y_m);
            cr.line_to(coords.x_nw, coords.y_s);
            cr.line_to(coords.x_ne, coords.y_s);
            cr.line_to(coords.x_fe, coords.y_m);
            cr.line_to(coords.x_ne, coords.y_n);
            cr.line_to(coords.x_nw, coords.y_n);
            cr.line_to(coords.x_fw, coords.y_m);
            cr.fill().expect("error while drawing stroke");
        }

        if step_state.cell_weights[max_idx].parent >= 0 {
            let mut cur_cell = max_idx;
            cr.set_source_rgb(1., 0., 0.);
            cr.set_line_width(4.0);
            let coords_1 = coords(cur_cell);
            cr.move_to(coords_1.cx, coords_1.cy);
            let mut seen = HashSet::new();
            while cur_cell != (min_idx) && !seen.contains(&cur_cell) {
                seen.insert(cur_cell);
                let coords_2 = coords(step_state.cell_weights[cur_cell].parent as usize);
                cr.line_to(coords_2.cx, coords_2.cy);
                cur_cell = step_state.cell_weights[cur_cell].parent as usize;
            }
            cr.stroke().expect("error while drawing stroke");
        }

        cr.restore().expect("error while restoring coords");
    }
}
