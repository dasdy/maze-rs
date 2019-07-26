use std::f64::consts::PI;
use crate::grid::AbstractGrid;
use std::collections::HashSet;
use std::fmt::{Display, Formatter, Error};

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