use crate::generate::recursive_backtracker;
use crate::grid::{AbstractCell, AbstractGrid};
use crate::solve::solve_with_longest_path;
use std::collections::HashSet;
use std::fmt::{Display, Error, Formatter};

use crate::solve::DijkstraStep;
use cairo::Context;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea};
use std::f64::consts::PI;

#[derive(Clone)]
pub struct PolarCell {
    pub outward: Vec<usize>,
    pub inward: Option<usize>,
    pub clockwise: usize,
    pub counter_clockwise: usize,
    pub row: usize,
    pub col: usize,
    pub columns: usize,
    pub links: HashSet<usize>,
}

impl AbstractCell for PolarCell {
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

impl Display for PolarCell {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "PolarCell(row:{}, col: {}, columns: {}, cw: {}, ccw: {}, inward: {}, outward: [",
            self.row,
            self.col,
            self.columns,
            self.clockwise,
            self.counter_clockwise,
            match self.inward {
                Some(ix) => format!("{}", ix),
                _ => "None".to_string(),
            }
        )?;
        for o in &self.outward {
            write!(f, "{}, ", o)?;
        }
        write!(f, "], links: [")?;
        for o in &self.links {
            write!(f, "{}, ", o)?;
        }
        write!(f, "])")?;
        Ok(())
    }
}

impl PolarCell {
    pub fn new(
        row: usize,
        col: usize,
        clockwise: usize,
        counter_clockwise: usize,
        columns: usize,
        outward: Vec<usize>,
    ) -> PolarCell {
        let links = HashSet::new();
        PolarCell {
            inward: None,
            clockwise,
            counter_clockwise,
            outward,
            links,
            col,
            row,
            columns,
        }
    }
}

#[derive(Clone)]
pub struct CircularGrid {
    pub height: usize,
    pub cells: Vec<PolarCell>,
}

impl AbstractGrid<PolarCell> for CircularGrid {
    fn neighbours(&self, ix: usize) -> Vec<usize> {
        let cell = &self.cells[ix];
        let mut neighbours = cell.outward.clone();
        if let Some(ix) = cell.inward {
            neighbours.push(ix)
        };
        neighbours.push(cell.counter_clockwise);
        neighbours.push(cell.clockwise);
        neighbours
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

    fn cell(&self, ix: usize) -> &PolarCell {
        &self.cells[ix]
    }
}

impl CircularGrid {
    pub fn new(rows: usize) -> CircularGrid {
        let mut cells = Vec::new();
        let mut cells_by_rows = Vec::new();
        let row_height = 1.0 / (rows as f64);

        cells.push(PolarCell::new(0, 0, 0, 0, 1, Vec::new()));
        cells_by_rows.push(vec![0]);
        let mut previous_count = 1;
        for i in 1..rows {
            let radius = (i as f64) / (rows as f64);
            let circ = 2. * PI * radius;
            let estimated_width = circ / (previous_count as f64);
            let ratio = (estimated_width / row_height).round() as usize;

            let cell_count = previous_count * ratio;
            let mut cells_in_row = Vec::new();
            for j in 0..cell_count {
                let current_cell_id = cells.len();
                let ccw = if j == 0 {
                    current_cell_id + cell_count - 1
                } else {
                    current_cell_id - 1
                };
                let cw = if j == cell_count - 1 {
                    cells_in_row[0]
                } else {
                    current_cell_id + 1
                };
                cells.push(PolarCell::new(i, j, cw, ccw, cell_count, Vec::new()));
                cells_in_row.push(current_cell_id);
            }
            previous_count = cell_count;
            cells_by_rows.push(cells_in_row);
        }

        for i in 1..cells.len() {
            let row = cells[i].row;
            let col = cells[i].col;
            let ratio = cells_by_rows[row].len() as f64 / cells_by_rows[row - 1].len() as f64;
            // TODO pay attention here
            let parent = cells_by_rows[row - 1][(col as f64 / ratio) as usize];
            cells[parent].outward.push(i);
            cells[i].inward = Some(parent);
        }

        CircularGrid {
            height: rows,
            cells,
        }
    }

    #[allow(dead_code)]
    pub fn outward_ixs(&self, ix: usize) -> Vec<usize> {
        self.cells[ix].outward.clone()
    }

    pub fn cw_ix(&self, ix: usize) -> usize {
        self.cells[ix].clockwise
    }

    #[allow(dead_code)]
    pub fn ccw_ix(&self, ix: usize) -> usize {
        self.cells[ix].counter_clockwise
    }

    pub fn inward_ix(&self, ix: usize) -> Option<usize> {
        // return self._ix_opt(row.wrapping_add(1), col);
        self.cells[ix].inward
    }

    #[allow(dead_code)]
    pub fn to_dot(&self) -> String {
        let mut res = "graph g {".to_owned();

        for i in 0..self.cells.len() {
            let row = self.cells[i].row;
            let col = self.cells[i].col;
            let links = &self.cells[i].links;
            // for ix in self.neighbours(i) {
            for &ix in links {
                let _col = self.cells[ix].col;
                let _row = self.cells[ix].row;
                // print only south and east neighbours to avoid duplicates

                res.push_str(
                    format!(
                        "\"({},{})\" -> \"({},{})\" [color=blue] \n",
                        row, col, _row, _col
                    )
                    .as_str(),
                );
            }
        }
        res.push_str("\n}\n");
        res
    }
}

pub fn draw_polar_maze(
    w: &DrawingArea,
    cr: &Context,
    g_polar: &CircularGrid,
    actual_ring_height: usize,
) {
    let scalex = w.get_allocated_width() as f64 / (g_polar.height * actual_ring_height * 2) as f64;
    let scaley = w.get_allocated_height() as f64 / (g_polar.height * actual_ring_height * 2) as f64;
    cr.scale(scalex, scaley);
    cr.set_line_width(1.0);

    let center_x = g_polar.height as f64 * actual_ring_height as f64;
    let center_y = center_x;
    let ring_height = actual_ring_height as f64;

    cr.arc(
        center_x,
        center_y,
        ring_height * g_polar.height as f64,
        0.,
        2. * PI,
    );
    cr.stroke();
    for i in 0..g_polar.len() {
        if i == 0 {
            continue;
        }
        let cell = g_polar.cell(i);
        let inward = g_polar.inward_ix(i).unwrap();
        let theta = 2. * PI / (cell.columns as f64);
        let inner_r = ring_height * cell.row() as f64;
        let outer_r = ring_height * (cell.row() + 1) as f64;
        let theta_cw = theta * (cell.col() as f64);
        let theta_ccw = theta * ((cell.col() + 1) as f64);
        if !cell.links().contains(&inward) {
            cr.arc(center_x, center_y, inner_r, theta_cw, theta_ccw);
            cr.stroke();
        }

        let east = g_polar.cw_ix(i);

        if !cell.links.contains(&east) {
            let cx = center_x + inner_r * theta_ccw.cos();
            let dx = center_x + outer_r * theta_ccw.cos();
            let cy = center_x + inner_r * theta_ccw.sin();
            let dy = center_x + outer_r * theta_ccw.sin();
            cr.move_to(cx, cy);
            cr.line_to(dx, dy);
            cr.stroke();
        }
    }
}

pub fn draw_polar_pathfind(
    w: &DrawingArea,
    cr: &Context,
    g: &CircularGrid,
    step_state: &DijkstraStep,
    cellsize: usize,
) {
    let scalex = w.get_allocated_width() as f64 / (g.height * cellsize * 2) as f64;
    let scaley = w.get_allocated_height() as f64 / (g.height * cellsize * 2) as f64;
    cr.scale(scalex, scaley);
    cr.set_line_width(cellsize as f64 + 1.); // 1. to not create gaps between rows
    let center_x = g.height as f64 * cellsize as f64;
    let center_y = center_x;

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

    // returns inner radius, theta1, theta2
    let pixcoord = |ix: usize| -> (f64, f64, f64) {
        let row = g.cell(ix).row() as f64;
        let col = g.cell(ix).col() as f64;
        let total_cols = g.cell(ix).columns as f64;
        let theta = 2. * PI / total_cols;
        let inner_r = (cellsize as f64) * (row + 0.5);

        (inner_r, theta * col, theta * (col + 1.01)) // 1.01 to not create gaps between clockwise neighbours
    };

    for (i, c) in step_state.cell_weights.iter().enumerate() {
        let intensity = (max_length - c.path_length) as f64 / max_length as f64;
        let dark = intensity;
        let bright = 0.5 + intensity / 2.;
        cr.set_source_rgb(dark, bright, dark);
        // cr.set_source_rgb(1., 1., 0.);
        let (r, theta1, theta2) = pixcoord(i);
        cr.arc(center_x, center_y, r, theta1, theta2);
        cr.stroke();
    }

    let connect = |ix1: usize, ix2: usize| {
        let r1 = g.cell(ix1).row();
        let r2 = g.cell(ix2).row();
        let theta = 2. * PI / (g.cell(ix1).columns as f64);

        if r1 == r2 {
            let col1 = g.cell(ix1).col;
            let col2 = g.cell(ix2).col;
            let total = g.cell(ix1).columns;

            let a1 = theta * (0.5 + col1 as f64);
            let a2 = theta * (0.5 + col2 as f64);
            // when last and first columns are connected, should draw counter-clockwise instead of clockwise
            let (a_from, a_to) =
                if usize::min(col1, col2) == 0 && usize::max(col1, col2) == total - 1 {
                    (f64::max(a2, a1), f64::min(a2, a1))
                } else {
                    (f64::min(a2, a1), f64::max(a2, a1))
                };

            cr.arc(
                center_x,
                center_y,
                (r1 as f64 + 0.5) * (cellsize as f64),
                a_from,
                a_to,
            );
            cr.stroke();
        } else {
            let start_r = (0.5 + r1 as f64) * (cellsize as f64);
            let end_r = (0.5 + r2 as f64) * (cellsize as f64);
            let theta2 = 2. * PI / (g.cell(ix2).columns as f64);
            let a = theta * (0.5 + g.cell(ix1).col as f64);
            let a2 = theta2 * (0.5 + g.cell(ix2).col as f64);
            let cx = center_x + start_r * a.cos();
            let dx = center_x + end_r * a2.cos();
            let cy = center_x + start_r * a.sin();
            let dy = center_x + end_r * a2.sin();
            cr.move_to(cx, cy);
            cr.line_to(dx, dy);
            cr.stroke();
        }
    };

    if step_state.cell_weights[max_idx].parent >= 0 {
        let mut cur_cell = max_idx as i32;
        cr.set_source_rgb(1., 0., 0.);
        cr.set_line_width(4.0);
        while cur_cell != (min_idx as i32) {
            connect(
                cur_cell as usize,
                step_state.cell_weights[cur_cell as usize].parent as usize,
            );
            cur_cell = step_state.cell_weights[cur_cell as usize].parent;
        }
    }
}

#[allow(dead_code)]
pub fn build_polar_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    window.set_default_size(400, 400);

    let img = gtk::DrawingArea::new();
    vbox.add(&img);

    img.set_vexpand(true);
    img.set_hexpand(true);

    let mut rng = rand::thread_rng();
    let actual_ring_height = 20;
    let mut g_polar = CircularGrid::new(10);
    recursive_backtracker(&mut g_polar, &mut rng);
    let step_state = solve_with_longest_path(&g_polar);

    let clone = g_polar.clone();
    img.connect_draw(move |w, cr| {
        draw_polar_pathfind(w, cr, &clone, &step_state, actual_ring_height);
        gtk::Inhibit(false)
    });

    img.connect_draw(move |w, cr| {
        draw_polar_maze(w, cr, &g_polar, actual_ring_height);
        gtk::Inhibit(false)
    });

    window.add(&vbox);
    window.show_all();
}
