use crate::draw_utils::GtkDrawable;
use crate::grid::{AbstractCell, AbstractGrid, CompassDirections, RectangularGrid};
use crate::gtk::prelude::WidgetExt;
use crate::rectangle::Cell;
use crate::solve::DijkstraStep;
use cairo::Context;
use gtk::DrawingArea;

#[derive(Clone)]
pub struct DeltaGrid {
    pub height: usize,
    pub width: usize,
    pub cells: Vec<Cell>,
}

impl CompassDirections for DeltaGrid {
    fn north_ix(&self, ix: usize) -> Option<usize> {
        let row = self.cells[ix].row;
        let col = self.cells[ix].col;
        if is_up(row, col) {
            None
        } else {
            self.ix_opt(row.wrapping_sub(1), col)
        }
    }

    fn east_ix(&self, ix: usize) -> Option<usize> {
        let row = self.cells[ix].row;
        let col = self.cells[ix].col;
        self.ix_opt(row, col.wrapping_add(1))
    }

    fn west_ix(&self, ix: usize) -> Option<usize> {
        let row = self.cells[ix].row;
        let col = self.cells[ix].col;
        self.ix_opt(row, col.wrapping_sub(1))
    }

    fn south_ix(&self, ix: usize) -> Option<usize> {
        let row = self.cells[ix].row;
        let col = self.cells[ix].col;

        if is_up(row, col) {
            self.ix_opt(row.wrapping_add(1), col)
        } else {
            None
        }
    }
}

impl AbstractGrid<Cell> for DeltaGrid {
    fn neighbours(&self, ix: usize) -> Vec<usize> {
        let neighbors = [
            self.north_ix(ix),
            self.south_ix(ix),
            self.east_ix(ix),
            self.west_ix(ix),
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

impl DeltaGrid {
    pub fn new(rows: usize, cols: usize) -> DeltaGrid {
        let mut gridarr = Vec::new();
        for i in 0..rows {
            for j in 0..cols {
                gridarr.push(Cell::new(i, j));
            }
        }
        DeltaGrid {
            width: cols,
            height: rows,
            cells: gridarr,
        }
    }
}

impl RectangularGrid for DeltaGrid {
    fn width(&self) -> usize {
        self.width
    }
    fn height(&self) -> usize {
        self.height
    }
}

fn is_up(row: usize, col: usize) -> bool {
    (row + col) % 2 == 0
}

struct DeltaCellPoints {
    pub westx: f64,
    pub eastx: f64,
    pub midx: f64,
    pub apexy: f64,
    pub basey: f64,
    pub cx: f64,
    pub cy: f64,
}

fn delta_points(row: usize, col: usize, cellsize: f64) -> DeltaCellPoints {
    let half_w = cellsize / 2.;
    let height = cellsize * 3f64.sqrt() / 2.;
    let half_h = height / 2.;
    let cx = half_w + col as f64 * half_w;
    let cy = half_h + row as f64 * height;
    let westx = cx - half_w;
    let midx = cx;
    let eastx = cx + half_w;

    let (basey, apexy) = if is_up(row, col) {
        (cy + half_h, cy - half_h)
    } else {
        (cy - half_h, cy + half_h)
    };

    DeltaCellPoints {
        westx,
        eastx,
        midx,
        apexy,
        basey,
        cx,
        cy,
    }
}

impl GtkDrawable for DeltaGrid {
    fn draw_maze(&self, w: &DrawingArea, cr: &Context, cellsize: f64) {
        cr.save().expect("error while saving coords");

        let canvas_width = (1 + self.width) as f64 * (cellsize) / 2. + cellsize * 0.1;
        let canvas_height = self.height as f64 * cellsize * 3f64.sqrt() / 2. + cellsize * 0.1;

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

            let coords = delta_points(cur_cell.row(), cur_cell.col(), cellsize);

            cr.move_to(coords.westx, coords.basey);
            draw_line(&self.west_ix(ix), (coords.midx, coords.apexy));
            draw_line(&self.east_ix(ix), (coords.eastx, coords.basey));

            let isup = is_up(cur_cell.row(), cur_cell.col());
            let no_south = isup && self.south_ix(ix).is_none();
            let not_linked = !isup
                && self
                    .north_ix(ix)
                    .map(|r_idx| !cur_cell.links().contains(&r_idx))
                    .unwrap_or(false);
            if no_south || not_linked {
                cr.line_to(coords.westx, coords.basey);
            }
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
        let canvas_width = (1 + self.width) as f64 * (cellsize) / 2. + cellsize * 0.1;
        let canvas_height = self.height as f64 * cellsize * 3f64.sqrt() / 2. + cellsize * 0.1;

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
            delta_points(row, col, cellsize)
        };

        for (i, c) in step_state.cell_weights.iter().enumerate() {
            let intensity = (max_length - c.path_length) as f64 / max_length as f64;
            let dark = intensity;
            let bright = 0.5 + intensity / 2.;
            cr.set_source_rgb(dark, bright, dark);
            cr.set_line_width(0.1);
            let coords = coords(i);

            cr.move_to(coords.westx, coords.basey);
            cr.line_to(coords.midx, coords.apexy);
            cr.line_to(coords.eastx, coords.basey);
            cr.fill().expect("error while drawing stroke");
        }

        if step_state.cell_weights[max_idx].parent >= 0 {
            let mut cur_cell = max_idx;
            cr.set_source_rgb(1., 0., 0.);
            cr.set_line_width(1.0);
            let coords_1 = coords(cur_cell);
            cr.move_to(coords_1.cx, coords_1.cy);
            while cur_cell != (min_idx) {
                let coords_2 = coords(step_state.cell_weights[cur_cell].parent as usize);
                cr.line_to(coords_2.cx, coords_2.cy);
                cur_cell = step_state.cell_weights[cur_cell].parent as usize;
            }
            cr.stroke().expect("error while drawing stroke");
        }

        cr.restore().expect("error while restoring coords");
    }
}
