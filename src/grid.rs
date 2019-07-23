use std::collections::HashSet;
use std::fmt::{Display, Formatter, Error};
use std::f64::consts::PI;

#[derive(Clone)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
    pub links: HashSet<(usize, usize)>,
}

#[allow(dead_code)]
impl Cell {
    pub fn new(row: usize, col: usize) -> Cell {
        Cell { row, col, links: HashSet::new() }
    }

    pub fn link(a: &mut Cell, other: &mut Cell) {
        a.links.insert((other.row, other.col));
        other.links.insert((a.row, a.col));
    }

    pub fn unlink(a: &mut Cell, other: &mut Cell) {
        a.links.remove(&(other.row, other.col));
        other.links.remove(&(a.row, a.col));
    }

    pub fn linked(&self, other: &Cell) -> bool {
        self.links.contains(&(other.row, other.col))
    }
}

#[derive(Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new(row: usize, col: usize) -> Grid {
        let mut gridarr = Vec::new();
        for i in 0..row {
            for j in 0..col {
                gridarr.push(Cell::new(i, j));
            }
        }
        Grid { width: col, height: row, cells: gridarr }
    }

    #[allow(dead_code)]
    pub fn access(&self, row: usize, col: usize) -> Option<&Cell> {
        self._ix_opt(row, col).map(|ix| &self.cells[ix])
    }

    pub fn _ix(&self, row: usize, col: usize) -> usize {
        col + row * self.width
    }

    pub fn _ix_opt(&self, row: usize, col: usize) -> Option<usize> {
        if row >= self.height || col >= self.width {
            return None;
        }
        return Some(self._ix(row, col));
    }

    pub fn north_ix(&self, row: usize, col: usize) -> Option<usize> {
        return self._ix_opt(row.wrapping_sub(1), col);
    }

    pub fn east_ix(&self, row: usize, col: usize) -> Option<usize> {
        return self._ix_opt(row, col.wrapping_add(1));
    }

    pub fn west_ix(&self, row: usize, col: usize) -> Option<usize> {
        return self._ix_opt(row, col.wrapping_sub(1));
    }

    pub fn south_ix(&self, row: usize, col: usize) -> Option<usize> {
        return self._ix_opt(row.wrapping_add(1), col);
    }

    pub fn link(&mut self, ix1: usize, ix2: usize) {
        let Cell { row: a_row, col: a_col, .. } = self.cells[ix1];
        let Cell { row: b_row, col: b_col, .. } = self.cells[ix2];
        &(self.cells[ix1].links).insert((b_row, b_col));
        &(self.cells[ix2].links).insert((a_row, a_col));
    }

    #[allow(dead_code)]
    pub fn to_img_buf(&self, cellsize: usize) -> image::RgbImage {
        let imwidth = (self.width * cellsize + 1) as u32;
        let imheigh = (self.height * cellsize + 1) as u32;
        // Create a new ImgBuf with width: imgx and height: imgy
        let mut imgbuf = image::ImageBuffer::new(imwidth, imheigh);
        for (_, _, p) in imgbuf.enumerate_pixels_mut() {
            *p = image::Rgb([255, 255, 255])
        }
        let pixel_color = image::Rgb([0, 0, 0]);

        for ix in 0..self.cells.len() {
            let cur_cell = &self.cells[ix];

            let mut draw_line =
                |item: &Option<usize>, start: (f32, f32), end: (f32, f32)| {
                    if let Some(r_idx) = item {
                        if cur_cell.linked(&(self.cells[*r_idx])) {
                            imageproc::drawing::draw_line_segment_mut(
                                &mut imgbuf, start, end, pixel_color,
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
            draw_line(&self.east_ix(cur_cell.row, cur_cell.col), (x2, y1), (x2, y2));
            draw_line(&self.south_ix(cur_cell.row, cur_cell.col), (x1, y2), (x2, y2));
            draw_line(&self.west_ix(cur_cell.row, cur_cell.col), (x1, y1), (x1, y2));
            draw_line(&self.north_ix(cur_cell.row, cur_cell.col), (x1, y1), (x2, y1));
        }

        return imgbuf;
    }

    #[allow(dead_code)]
    pub fn to_dot(&self) -> String {
        let mut res = "graph g {".to_owned();

        for c in &self.cells {
            for (n_row, n_col) in &c.links {
                // print only south and east neighbours to avoid duplicates
                if *n_row >= c.row && *n_col >= c.col {
                    res.push_str(
                        format!("\"({},{})\" -- \"({},{})\" \n", c.row, c.col, n_row, n_col
                        ).as_str());
                }
            }
        }

        res.push_str("\n}\n");
        return res;
    }
}

impl Display for Grid {
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
                let f =
                    |neighbour: Option<usize>, ok: &str, bound: &str| -> String {
                        match neighbour {
                            Some(c) if current_cell.linked(&self.cells[c]) => ok.to_string(),
                            _ => bound.to_string()
                        }
                    };
                let east_bound = f(self.east_ix(i, j), " ", "|");
                let south_bound = f(self.south_ix(i, j), "   ", "---");

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

pub struct PolarCell {
    pub inner_r: f64,
    pub outer_r: f64,
    pub theta_cw: f64,
    pub theta_ccw: f64,
    pub row: usize,
    pub col: usize,
    pub links: HashSet<(usize, usize)>
}

impl PolarCell {
    pub fn new(ring_height: usize, cell_count: usize, row: usize, col: usize) -> PolarCell {
        let theta = 2.* PI/(cell_count as f64);
        let inner_r = (ring_height * row) as f64;
        let outer_r = (ring_height * (row + 1)) as f64;
        let theta_cw = theta * (col as f64);
        let theta_ccw = theta * ((col + 1) as f64);
        let links = HashSet::new();
        PolarCell {inner_r, outer_r, theta_cw, theta_ccw, row, col, links}
    }
}

pub struct CircularGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<PolarCell>
}

pub trait AbstractGrid {
    fn neighbours(&self, ix: usize) -> Vec<usize>;
    fn links(&self, ix: usize) -> HashSet<(usize, usize)>;
    fn len(&self) -> usize;
    fn link(&mut self, ix1: usize, ix2: usize);
}

impl AbstractGrid for CircularGrid {
    fn neighbours(&self, ix: usize) -> Vec<usize> {
        let row = self.cells[ix].row;
        let col= self.cells[ix].col;
        let neighbors = &vec![self.north_ix(row, col), self.east_ix(row,col),
                    self.west_ix(row, col), self.south_ix(row, col)];

        let neighbors: Vec<usize> = neighbors.iter().filter_map(|x| *x).collect();
        neighbors
    }
    fn links(&self, ix: usize) -> HashSet<(usize, usize)> {
        self.cells[ix].links.iter().cloned().collect()
    }
    fn len(&self) -> usize {
        self.cells.len()
    }

    fn link(&mut self, ix1: usize, ix2: usize) {
        let PolarCell { row: a_row, col: a_col, .. } = self.cells[ix1];
        let PolarCell { row: b_row, col: b_col, .. } = self.cells[ix2];
        &(self.cells[ix1].links).insert((b_row, b_col));
        &(self.cells[ix2].links).insert((a_row, a_col));
    }
}

impl AbstractGrid for Grid {
    fn neighbours(&self, ix: usize) -> Vec<usize> {
        let row = self.cells[ix].row;
        let col= self.cells[ix].col;
        let neighbors = &vec![self.north_ix(row, col), self.east_ix(row,col),
                    self.west_ix(row, col), self.south_ix(row, col)];

        let neighbors: Vec<usize> = neighbors.iter().filter_map(|x| *x).collect();
        neighbors
    }

    fn links(&self, ix: usize) -> HashSet<(usize, usize)> {
        self.cells[ix].links.iter().cloned().collect()
    }

        fn len(&self) -> usize {
        self.cells.len()
    }

    fn link(&mut self, ix1: usize, ix2: usize) {
        let Cell { row: a_row, col: a_col, .. } = self.cells[ix1];
        let Cell { row: b_row, col: b_col, .. } = self.cells[ix2];
        &(self.cells[ix1].links).insert((b_row, b_col));
        &(self.cells[ix2].links).insert((a_row, a_col));
    }
}

impl CircularGrid {

    pub fn from_rect_grid(g: &Grid, ring_height: usize) -> CircularGrid {
        let mut cells = Vec::new();
        let height = g.height;
        let width = g.width;
        for cell in g.cells.iter() {
            let mut new_cell = PolarCell::new(ring_height, width, cell.row, cell.col);
            new_cell.links = cell.links.iter().cloned().collect();
            cells.push(new_cell);
        }

        CircularGrid {height, width, cells}
    }

    pub fn _ix(&self, row: usize, col: usize) -> usize {
        col + row * self.width
    }

    pub fn _ix_opt(&self, row: usize, col: usize) -> Option<usize> {
        if row >= self.height || col >= self.width {
            return None;
        }
        return Some(self._ix(row, col));
    }

    pub fn north_ix(&self, row: usize, col: usize) -> Option<usize> {
        return self._ix_opt(row.wrapping_sub(1), col);
    }

    pub fn east_ix(&self, row: usize, col: usize) -> Option<usize> {
        let east_col = if col == self.width - 1  { 0 } else {col + 1};
        return self._ix_opt(row, east_col);
    }

    pub fn west_ix(&self, row: usize, col: usize) -> Option<usize> {
        let west_col = if col == 0 { self.width - 1 } else {col - 1};
        return self._ix_opt(row, west_col);
    }

    pub fn south_ix(&self, row: usize, col: usize) -> Option<usize> {
        return self._ix_opt(row.wrapping_add(1), col);
    }
}