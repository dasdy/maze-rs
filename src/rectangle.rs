use crate::draw_utils::GtkDrawable;
use crate::grid::{AbstractCell, AbstractGrid, CompassDirections};
use crate::solve::DijkstraStep;
use cairo::Context;
use gtk::prelude::*;
use gtk::DrawingArea;
use std::collections::HashSet;
use std::f64::consts::PI;
use std::fmt::{Display, Error, Formatter};

#[derive(Clone)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
    pub links: HashSet<usize>,
}

impl AbstractCell for Cell {
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

impl Cell {
    pub fn new(row: usize, col: usize) -> Cell {
        Cell {
            row,
            col,
            links: HashSet::new(),
        }
    }
}

#[derive(Clone)]
pub struct RectangleGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

impl CompassDirections for RectangleGrid {
    fn north_ix(&self, ix: usize) -> Option<usize> {
        let row = self.cells[ix].row;
        let col = self.cells[ix].col;
        self._ix_opt(row.wrapping_sub(1), col)
    }

    fn east_ix(&self, ix: usize) -> Option<usize> {
        let row = self.cells[ix].row;
        let col = self.cells[ix].col;
        self._ix_opt(row, col.wrapping_add(1))
    }

    fn west_ix(&self, ix: usize) -> Option<usize> {
        let row = self.cells[ix].row;
        let col = self.cells[ix].col;
        self._ix_opt(row, col.wrapping_sub(1))
    }

    fn south_ix(&self, ix: usize) -> Option<usize> {
        let row = self.cells[ix].row;
        let col = self.cells[ix].col;
        self._ix_opt(row.wrapping_add(1), col)
    }
}

impl RectangleGrid {
    pub fn new(row: usize, col: usize) -> RectangleGrid {
        let mut gridarr = Vec::new();
        for i in 0..row {
            for j in 0..col {
                gridarr.push(Cell::new(i, j));
            }
        }
        RectangleGrid {
            width: col,
            height: row,
            cells: gridarr,
        }
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

    pub fn link(&mut self, ix1: usize, ix2: usize) {
        (self.cells[ix1].links).insert(ix2);
        (self.cells[ix2].links).insert(ix1);
    }

    #[allow(dead_code)]
    pub fn to_img_buf(&self, cellsize: usize) -> image::RgbImage {
        let imwidth = (self.width * cellsize + 1) as u32;
        let imheigh = (self.height * cellsize + 1) as u32;
        // Create a new ImgBuf with width: imgx and height: imgy
        let mut imgbuf = image::RgbImage::new(imwidth, imheigh);
        for (_, _, p) in imgbuf.enumerate_pixels_mut() {
            *p = image::Rgb([255, 255, 255])
        }
        let pixel_color = image::Rgb([0, 0, 0]);

        for ix in 0..self.cells.len() {
            let cur_cell = &self.cells[ix];

            let mut draw_line = |item: &Option<usize>, start: (f32, f32), end: (f32, f32)| {
                if let Some(r_idx) = item {
                    if cur_cell.links.contains(&r_idx) {
                        imageproc::drawing::draw_line_segment_mut(
                            &mut imgbuf,
                            start,
                            end,
                            pixel_color,
                        );
                    }
                }
            };
            fn asf32(ix: usize, size: usize) -> f32 {
                (ix * size) as f32
            }

            let x1 = asf32(cur_cell.col, cellsize);
            let x2 = asf32(cur_cell.col + 1, cellsize);
            let y1 = asf32(cur_cell.row, cellsize);
            let y2 = asf32(cur_cell.row + 1, cellsize);
            draw_line(&self.east_ix(ix), (x2, y1), (x2, y2));
            draw_line(&self.south_ix(ix), (x1, y2), (x2, y2));
            draw_line(&self.west_ix(ix), (x1, y1), (x1, y2));
            draw_line(&self.north_ix(ix), (x1, y1), (x2, y1));
        }

        imgbuf
    }

    #[allow(dead_code)]
    pub fn to_dot(&self) -> String {
        let mut res = "graph g {".to_owned();

        for c in &self.cells {
            for &ix in c.links.iter() {
                let col = self.cells[ix].col;
                let row = self.cells[ix].row;
                // print only south and east neighbours to avoid duplicates
                if row >= c.row && col >= c.col {
                    res.push_str(
                        format!("\"({},{})\" -- \"({},{})\" \n", c.row, c.col, col, row).as_str(),
                    );
                }
            }
        }

        res.push_str("\n}\n");
        res
    }
}

impl Display for RectangleGrid {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "+")?;
        for _ in 0..self.width {
            write!(f, "---+")?;
        }
        writeln!(f)?;
        for i in 0..self.height {
            let mut top = "|".to_owned();
            let mut bottom = "+".to_owned();
            for j in 0..self.width {
                let body = "   ";
                let current_cell = &self.cells[self._ix(i, j)];
                let f = |neighbour: Option<usize>, ok: &str, bound: &str| -> String {
                    match neighbour {
                        Some(c) if current_cell.links.contains(&c) => ok.to_string(),
                        _ => bound.to_string(),
                    }
                };
                let east_bound = f(self.east_ix(self._ix(i, j)), " ", "|");
                let south_bound = f(self.south_ix(self._ix(i, j)), "   ", "---");

                top.push_str(body);
                top.push_str(&east_bound);
                bottom.push_str(&south_bound);
                bottom.push_str("+");
            }
            writeln!(f, "{}", top)?;
            writeln!(f, "{}", bottom)?;
        }
        Result::Ok(())
    }
}

impl AbstractGrid<Cell> for RectangleGrid {
    fn neighbours(&self, ix: usize) -> Vec<usize> {
        let neighbors = &vec![
            self.north_ix(ix),
            self.east_ix(ix),
            self.west_ix(ix),
            self.south_ix(ix),
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

    fn cell(&self, ix: usize) -> &Cell {
        &self.cells[ix]
    }
}

impl GtkDrawable<Cell> for RectangleGrid {
    fn draw_maze(&self, w: &DrawingArea, cr: &Context, cellsize: f64) {
        let scalex = w.get_allocated_width() as f64 / (self.width as f64 * cellsize);
        let scaley = w.get_allocated_height() as f64 / (self.height as f64 * cellsize);

        cr.scale(scalex, scaley);
        cr.set_line_width(1.0);
        for ix in 0..self.len() {
            let cur_cell = self.cell(ix);
            let draw_line = |item: &Option<usize>, end: (f64, f64)| match item {
                Some(r_idx) if !cur_cell.links().contains(r_idx) => cr.line_to(end.0, end.1),
                _ => cr.move_to(end.0, end.1),
            };
            let pixcoord = |ix: usize| -> f64 { ix as f64 * cellsize };
            let x1 = pixcoord(cur_cell.col());
            let x2 = pixcoord(cur_cell.col() + 1);
            let y1 = pixcoord(cur_cell.row());
            let y2 = pixcoord(cur_cell.row() + 1);
            cr.move_to(x1, y1);
            draw_line(&self.west_ix(ix), (x1, y2));
            draw_line(&self.south_ix(ix), (x2, y2));
            draw_line(&self.east_ix(ix), (x2, y1));
            draw_line(&self.north_ix(ix), (x1, y1));
            cr.stroke();
        }
    }

    fn draw_pathfind(
        &self,
        w: &DrawingArea,
        cr: &Context,
        step_state: &DijkstraStep,
        cellsize: f64,
    ) {
        cr.save();
        let scalex = w.get_allocated_width() as f64 / (self.width as f64 * cellsize);
        let scaley = w.get_allocated_height() as f64 / (self.height as f64 * cellsize);
        cr.scale(scalex, scaley);
        cr.set_line_width(1.0);

        let pixcoord = |ix: usize| -> f64 { (ix as f64 + 0.5) * cellsize };

        let circle = |x: f64, y: f64| {
            cr.save();
            cr.translate(x, y);
            cr.arc(0., 0., cellsize / 2., 0., 2. * PI);
            cr.restore();
        };

        let coords = |ix: i32| {
            let row = self.cell(ix as usize).row();
            let col = self.cell(ix as usize).col();
            (pixcoord(col), pixcoord(row))
        };

        let rect = |i: usize| {
            let row = self.cell(i).row() as f64;
            let col = self.cell(i).col() as f64;
            let (x1, y1) = (col * cellsize, row * cellsize);
            cr.rectangle(x1, y1, cellsize, cellsize);
            cr.fill();
        };

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

        let cur_cell = self.cell(min_idx);
        let end_cell = self.cell(max_idx);

        cr.set_line_width(6.0);
        for (i, c) in step_state.cell_weights.iter().enumerate() {
            let intensity = (max_length - c.path_length) as f64 / max_length as f64;
            let dark = intensity;
            let bright = 0.5 + intensity / 2.;
            cr.set_source_rgb(dark, bright, dark);
            rect(i);
        }

        let x1 = pixcoord(cur_cell.col());
        let x2 = pixcoord(end_cell.col());

        let y1 = pixcoord(cur_cell.row());
        let y2 = pixcoord(end_cell.row());
        cr.set_line_width(1.0);
        cr.set_source_rgb(0., 0., 0.);
        circle(x1, y1);
        cr.stroke();
        circle(x2, y2);
        cr.stroke();
        if step_state.cell_weights[max_idx].parent >= 0 {
            let mut cur_cell = max_idx as i32;
            cr.set_source_rgb(1., 0., 0.);
            cr.set_line_width(4.0);
            let (x1, y1) = coords(cur_cell);
            cr.move_to(x1, y1);

            while cur_cell != (min_idx as i32) {
                let (x2, y2) = coords(step_state.cell_weights[cur_cell as usize].parent);
                cr.line_to(x2, y2);
                cur_cell = step_state.cell_weights[cur_cell as usize].parent;
            }
            cr.stroke();
        }
        cr.restore();
    }
}
