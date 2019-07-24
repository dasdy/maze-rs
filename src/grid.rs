use std::collections::HashSet;
use std::fmt::{Display, Formatter, Error};
use std::f64::consts::PI;

#[derive(Clone)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
    pub links: HashSet<usize>,
}

#[allow(dead_code)]
impl Cell {
    pub fn new(row: usize, col: usize) -> Cell {
        Cell { row, col, links: HashSet::new() }
    }
}

#[derive(Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

impl Grid {
    #[allow(dead_code)]
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
        &(self.cells[ix1].links).insert(ix2);
        &(self.cells[ix2].links).insert(ix1);
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
                        if cur_cell.links.contains(&r_idx) {
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
            for &ix in c.links.iter() {
                let col = self.cells[ix].col;
                let row = self.cells[ix].row;
                // print only south and east neighbours to avoid duplicates
                if row >= c.row && col >= c.col {
                    res.push_str(
                        format!("\"({},{})\" -- \"({},{})\" \n", c.row, c.col, col, row
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
                            Some(c) if current_cell.links.contains(&c) => ok.to_string(),
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

#[derive(Clone)]
pub struct PolarCell {
    pub outward: Vec<usize>,
    pub inward: Option<usize>,
    pub clockwise: usize,
    pub counter_clockwise: usize,
    pub row: usize,
    pub col: usize,
    pub columns: usize,
    pub links: HashSet<usize>
}

impl Display for PolarCell {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "PolarCell(row:{}, col: {}, columns: {}, cw: {}, ccw: {}, inward: {}, outward: [",
        self.row, self.col, self.columns, self.clockwise, self.counter_clockwise, match self.inward {
            Some(ix) => format!("{}", ix),
            _ => format!("None")
        })?;
        for o in &self.outward {
            write!(f, "{}, ", o)?;
        };
        write!(f, "], links: [")?;
        for o in &self.links {
            write!(f, "{}, ", o)?;
        }
        write!(f, "])")?;
        return Ok(());
    }
}

impl PolarCell {
    pub fn new(row: usize, col: usize, clockwise: usize, counter_clockwise: usize,
               columns: usize, outward: Vec<usize>) -> PolarCell {
        let links = HashSet::new();
        PolarCell {inward: None, clockwise, counter_clockwise, outward: outward.clone(), links, col, row, columns}
    }
}

#[derive(Clone)]
pub struct CircularGrid {
    pub height: usize,
    pub cells: Vec<PolarCell>
}

pub trait AbstractGrid {
    fn neighbours(&self, ix: usize) -> Vec<usize>;
    fn links(&self, ix: usize) -> HashSet<usize>;
    fn len(&self) -> usize;
    fn link(&mut self, ix1: usize, ix2: usize);
}

impl AbstractGrid for CircularGrid {
    fn neighbours(&self, ix: usize) -> Vec<usize> {
        let cell = &self.cells[ix];
        let mut neighbours = cell.outward.clone();
        match cell.inward {
            Some(ix) =>  neighbours.push(ix),
            None => {}
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
        &(self.cells[ix1].links).insert(ix2);
        &(self.cells[ix2].links).insert(ix1);
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

    fn links(&self, ix: usize) -> HashSet<usize> {
        self.cells[ix].links.iter().cloned().collect()
    }

    fn len(&self) -> usize {
        self.cells.len()
    }

    fn link(&mut self, ix1: usize, ix2: usize) {
        &(self.cells[ix1].links).insert(ix2);
        &(self.cells[ix2].links).insert(ix1);
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
                let ccw = if j == 0 {current_cell_id + cell_count - 1} else {current_cell_id - 1};
                let cw = if j == cell_count - 1 {cells_in_row[0] } else {current_cell_id + 1};;
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
            let parent = cells_by_rows[row - 1][(col as f64/ ratio) as usize];
            cells[parent].outward.push(i);
            cells[i].inward = Some(parent);
        }

        CircularGrid {height: rows, cells}
    }

    #[allow(dead_code)]
    pub fn outward_ixs(&self, ix: usize) -> Vec<usize> {
        return self.cells[ix].outward.clone();
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
                    format!("\"({},{})\" -> \"({},{})\" [color=blue] \n", row, col, _row, _col
                    ).as_str());
                
            }
        }
        res.push_str("\n}\n");
        return res;
    }
}